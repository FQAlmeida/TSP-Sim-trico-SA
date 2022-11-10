use std::{sync::mpsc, thread::spawn};

use tokio::{
    fs::OpenOptions,
    io::{self, AsyncWriteExt},
};

use threadpool::ThreadPool;
use tsa_sim::{cooling_methods::ExpCooling, TSAConfig, TSA};

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
}

const QTD_ITERS_INITIAL: usize = 2_000_000;
const TEMP_INITIAL_INITIAL: f64 = 800.0;
const TEMP_FINAL_INITIAL: f64 = 20.0;
const QTD_ITERS_ON_TEMP_INITIAL: usize = 10;

impl Config {
    pub fn create(id: usize) -> Self {
        Self {
            qtd_iters: QTD_ITERS_INITIAL,
            temp_initial: TEMP_INITIAL_INITIAL,
            temp_final: TEMP_FINAL_INITIAL,
            qtd_iters_on_temp: QTD_ITERS_ON_TEMP_INITIAL,
            id,
        }
    }
}

struct ChannelData {
    distance: f64,
    iter: usize,
    temp: f64,
    method: &'static str,
    id: usize,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let num_workers = 16usize;
    let num_jobs = 16usize;
    let pool = ThreadPool::new(num_workers);

    let data = data_retrieve::load("data/inst_100.txt");

    let config = Config::create(0);
    let (sender, receiver) = mpsc::channel::<ChannelData>();

    spawn(move || {
        for id in 0..num_jobs {
            let sender_clone = sender.clone();
            let mut config_clone = config.clone();
            config_clone.id = id;
            let data_clone = data.clone();
            let sim_config = TSAConfig::<ExpCooling>::create(
                config.temp_final,
                config.temp_initial,
                config.qtd_iters,
                config.qtd_iters_on_temp,
            );
            pool.execute(move || {
                let mut sim = TSA::create(data_clone, sim_config);
                for _ in 0..config.qtd_iters {
                    sim.gen_next_solution();
                    let resp = sender_clone.send(ChannelData {
                        distance: sim.get_current_distance(),
                        id: config_clone.id,
                        iter: sim.get_current_iter(),
                        method: "exp",
                        temp: sim.get_current_temperature(),
                    });
                    match resp {
                        Ok(_) => {}
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                }
                dbg!(id);
                dbg!("Job done");
            });
        }
    });

    let mut options = OpenOptions::new();
    let file_opener = options.create(true).append(true);
    for data in receiver.iter() {
        let fp = format!("data/{}/results_{}.txt", data.method, data.id);
        let mut file = file_opener.open(fp).await?;
        let data_string = format!("{} {} {}\n", data.iter, data.distance, data.temp);
        file.write(data_string.as_bytes()).await?;
    }
    Ok(())
}
