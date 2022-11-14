use std::{
    collections::HashMap,
    sync::mpsc::{self, Sender},
    thread::spawn,
};

use data_retrieve::DataNode;
use tokio::{
    fs::File,
    io::{self, AsyncWriteExt},
};

use threadpool::ThreadPool;
use tsa_sim::{
    cooling_methods::{CoolingMethod, CosCooling, ExpCooling, SigmoidCooling},
    TSAConfig, TSA,
};

/**
 * Ideia
 * Var qtd_iters 10..10_000_000..100
 * Var temp_inicial 1..100..1
 * Var temp_final 1E-1..1E-10..?
 * Var qtd_iters_on_temp 1..10..1
 *  */
#[derive(Debug, Clone, Copy)]
struct Config {
    qtd_iters: usize,
    temp_initial: f64,
    temp_final: f64,
    qtd_iters_on_temp: usize,
    id: usize,
    method: &'static str,
    inst: usize,
}

const QTD_ITERS_INITIAL: usize = 3_000_000;
const TEMP_INITIAL_INITIAL: f64 = 800.0;
const TEMP_FINAL_INITIAL: f64 = 1E-6;
const QTD_ITERS_ON_TEMP_INITIAL: usize = 1;

impl Config {
    pub fn create(id: usize, method: &'static str, inst: usize, qtd_on_temp: usize) -> Self {
        Self {
            qtd_iters: QTD_ITERS_INITIAL,
            temp_initial: TEMP_INITIAL_INITIAL,
            temp_final: TEMP_FINAL_INITIAL,
            qtd_iters_on_temp: qtd_on_temp,
            id,
            method,
            inst,
        }
    }

    pub fn create_first() -> Self {
        Self {
            qtd_iters: QTD_ITERS_INITIAL,
            temp_initial: TEMP_INITIAL_INITIAL,
            temp_final: TEMP_FINAL_INITIAL,
            qtd_iters_on_temp: QTD_ITERS_ON_TEMP_INITIAL,
            id: 0,
            method: "exp",
            inst: 100,
        }
    }
}

impl Iterator for Config {
    type Item = Config;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.id + 1;
        let method = self.method;
        let inst = self.inst;
        let qtd_on_temp = self.qtd_iters_on_temp;
        let next_self: Config;
        if method == "b" {
            return None;
        }
        if id >= 50 {
            let (m, instance, qtd_on_iter) = if method == "exp" {
                ("sigmoid", inst, qtd_on_temp)
            } else if method == "sigmoid" {
                ("cos", inst, qtd_on_temp)
            } else if inst == 100 && qtd_on_temp == 1{
                ("exp", 51, qtd_on_temp)
            } else if qtd_on_temp == 1 {
                ("exp", inst, 10)
            } else if inst == 100 && qtd_on_temp == 10 {
                ("b", 1, 1)
            }
            else {
                ("exp", 100, 10)
            };
            next_self = Config::create(0, m, instance, qtd_on_iter);
        } else {
            next_self = Config::create(id, method, inst, qtd_on_temp);
        }
        let r = Some(self.clone());
        self.id = next_self.id;
        self.method = next_self.method;
        self.inst = next_self.inst;
        self.qtd_iters_on_temp = next_self.qtd_iters_on_temp;
        return r;
    }
}

struct ChannelData {
    distance: f64,
    method: &'static str,
    inst: usize,
    qtd_on_iter: usize,
}

fn worker<T: CoolingMethod + 'static>(
    sender: &Sender<ChannelData>,
    config: Config,
    data: &Vec<DataNode>,
    pool: &ThreadPool,
) {
    // for id in 0..qtd_jobs {
    let sender_clone = sender.clone();
    let data_clone = data.clone();
    let config_clone = config.clone();
    pool.execute(move || {
        dbg!(&config_clone);
        let sim_config = TSAConfig::<T>::create(
            config_clone.temp_final,
            config_clone.temp_initial,
            config_clone.qtd_iters,
            config_clone.qtd_iters_on_temp,
        );
        let mut sim = TSA::create(data_clone, sim_config);
        for _ in 0..config.qtd_iters {
            sim.gen_next_solution();
        }
        sender_clone
            .send(ChannelData {
                distance: sim.get_current_distance(),
                method: config_clone.method,
                inst: config_clone.inst,
                qtd_on_iter: config_clone.qtd_iters_on_temp,
            })
            .unwrap();
        dbg!(&config_clone);
        dbg!("Job done");
    });
    // }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let num_workers = 12usize;
    let pool = ThreadPool::new(num_workers);

    let data = data_retrieve::load("data/inst_100.txt");
    let data_51 = data_retrieve::load("data/inst_51.txt");

    let configs = Config::create_first();
    let (sender, receiver) = mpsc::channel::<ChannelData>();

    let mut files: HashMap<String, File> = HashMap::new();
    let mut data_queue: HashMap<String, Vec<u8>> = HashMap::new();
    for met in ["exp", "cos", "sigmoid"] {
        for inst in [51, 100] {
            for on_temp in [1, 10] {
                let fp = format!("data/runs/inst_{}_{}_on_temp_{}.txt", inst, met, on_temp);
                let key = fp.clone();
                let fd = File::create(fp).await?;
                files.insert(key.clone(), fd);
                data_queue.insert(key, Vec::new());
            }
        }
    }

    let h = spawn(move || {
        for config in configs {
            let data_clone = if config.inst == 100 {
                data.clone()
            } else {
                data_51.clone()
            };
            match config.method {
                "exp" => {
                    worker::<ExpCooling>(&sender, config, &data_clone, &pool);
                }
                "cos" => {
                    worker::<CosCooling>(&sender, config, &data_clone, &pool);
                }
                "sigmoid" => {
                    worker::<SigmoidCooling>(&sender, config, &data_clone, &pool);
                }
                _ => {}
            }
            // dbg!(config);
        }
    });

    for data in receiver.iter() {
        let fp = format!(
            "data/runs/inst_{}_{}_on_temp_{}.txt",
            data.inst, data.method, data.qtd_on_iter
        );
        // dbg!(fp.clone());
        let data_q = data_queue.get_mut(&fp).unwrap();
        let data_string = format!("{}\n", data.distance);
        data_q.extend(data_string.as_bytes());
    }
    for (key, data_q) in data_queue.iter() {
        let fd = files.get_mut(key).unwrap();
        fd.write(data_q).await?;
    }
    h.join().unwrap();
    // dbg!(data_queue);
    Ok(())
}
