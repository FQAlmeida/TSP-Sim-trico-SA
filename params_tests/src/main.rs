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
    spawn(move || {});

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
