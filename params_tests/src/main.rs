use std::{f64::MAX, sync::mpsc, thread::spawn};

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
}

const MAX_QTD_ITERS: usize = 100_000;
const QTD_ITERS_INITIAL: usize = 10000;
const MAX_TEMP_INITIAL: f64 = 100.0;
const TEMP_INITIAL_INITIAL: f64 = 1.0;
const MAX_TEMP_FINAL: f64 = 1.0E-6;
const TEMP_FINAL_INITIAL: f64 = 1.0E-6;
const MAX_QTD_ITERS_ON_TEMP: usize = 5;
const QTD_ITERS_ON_TEMP_INITIAL: usize = 1;

impl Config {
    pub fn create() -> Self {
        Self {
            qtd_iters: QTD_ITERS_INITIAL,
            temp_initial: TEMP_INITIAL_INITIAL,
            temp_final: TEMP_FINAL_INITIAL,
            qtd_iters_on_temp: QTD_ITERS_ON_TEMP_INITIAL,
        }
    }
    fn gen_next(&mut self) -> bool {
        // QTD ITERS
        let next_qtd_iters = if self.qtd_iters < MAX_QTD_ITERS {
            self.qtd_iters + 10000
        } else {
            QTD_ITERS_INITIAL
        };
        self.qtd_iters = next_qtd_iters;
        if next_qtd_iters != QTD_ITERS_INITIAL {
            return true;
        }

        // TEMP INITIAL
        let next_temp_initial = if self.temp_initial < MAX_TEMP_INITIAL {
            self.temp_initial + 1.0
        } else {
            TEMP_INITIAL_INITIAL
        };
        self.temp_initial = next_temp_initial;
        if next_temp_initial != TEMP_INITIAL_INITIAL {
            return true;
        }

        // TEMP FINAL
        let next_final_temp = if self.temp_final < MAX_TEMP_FINAL {
            self.temp_final + 1.0E-6
        } else {
            TEMP_FINAL_INITIAL
        };
        self.temp_final = next_final_temp;
        if next_final_temp != TEMP_FINAL_INITIAL {
            return true;
        }

        // QTD ITERS TEMP
        let next_qtd_iters_on_temp = if self.qtd_iters_on_temp < MAX_QTD_ITERS_ON_TEMP {
            self.qtd_iters_on_temp + 1
        } else {
            QTD_ITERS_ON_TEMP_INITIAL
        };
        self.qtd_iters_on_temp = next_qtd_iters_on_temp;
        if next_qtd_iters_on_temp != QTD_ITERS_ON_TEMP_INITIAL {
            return true;
        }
        // if self.method == CoolingTypes::SIGMOID {
        //     self.qtd_iters = QTD_ITERS_INITIAL;
        //     self.temp_initial = TEMP_INITIAL_INITIAL;
        //     self.temp_final = TEMP_FINAL_INITIAL;
        //     self.qtd_iters_on_temp = QTD_ITERS_ON_TEMP_INITIAL;
        //     self.method = CoolingTypes::EXP;
        //     return true;
        // }
        false
    }
}

impl Iterator for Config {
    type Item = Config;

    fn next(&mut self) -> Option<Self::Item> {
        let cloned = self.clone();
        if !self.gen_next() {
            return None;
        }
        Some(cloned)
    }
}

struct ChannelData {
    config: Config,
    distance: f64,
}

fn main() {
    let num_workers = 16usize;
    let pool = ThreadPool::new(num_workers);

    let data = data_retrieve::load("data/inst_100.txt");

    let config = Config::create();
    let (sender, receiver) = mpsc::channel::<ChannelData>();
    spawn(move || {
        for c in config {
            dbg!(c);
            let cloned_data = data.clone();
            let sim_config = TSAConfig::<ExpCooling>::create(
                c.temp_final,
                c.temp_initial,
                c.qtd_iters,
                c.qtd_iters_on_temp,
            );
            let mut sim = TSA::create(cloned_data, sim_config);
            let cloned_sender = sender.clone();

            pool.execute(move || {
                for _ in 0..c.qtd_iters * c.qtd_iters_on_temp {
                    sim.gen_next_solution();
                }
                cloned_sender
                    .send(ChannelData {
                        config: c,
                        distance: sim.get_current_distance(),
                    })
                    .unwrap();
                dbg!(c);
                dbg!(sim.get_current_distance());
            })
        }
    });

    let mut smallest_dist: f64 = MAX;
    let mut conf: Config = Config::create();
    for data in receiver.iter() {
        if data.distance < smallest_dist {
            smallest_dist = data.distance;
            conf = data.config;
        }
    }
    dbg!(smallest_dist);
    dbg!(conf);
}
