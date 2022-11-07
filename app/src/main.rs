use data_retrieve::load;
use graphics_engine::{App, EventsBridge};
use std::{sync::mpsc::channel, thread::spawn};
use tsa_sim::{
    cooling_methods::{CoolingMethod, ExpCooling, SigmoidCooling},
    TSAConfig, TSA,
};

fn handle_update<T: CoolingMethod + 'static>(tsa: &TSA<T>) -> Vec<graphics_engine::Object> {
    let mut objects: Vec<graphics_engine::Object> = vec![];
    // tsa.gen_next_solution();
    // println!("{:?}", tsa.solution);

    for solution_origem_index in 0..tsa.solution.len() {
        let solution_destiny_index = (solution_origem_index + 1) % tsa.solution.len();
        let origem_index = tsa.solution[solution_origem_index];
        let destiny_index = tsa.solution[solution_destiny_index];

        let origem = &tsa.data[origem_index];
        let destiny = &tsa.data[destiny_index];
        let origem_point = (origem.point.x, origem.point.y);
        let destiny_point = (destiny.point.x, destiny.point.y);

        objects.push(graphics_engine::Object::create(
            [origem_point, destiny_point].to_vec(),
            [0.5; 4],
            graphics_engine::ObjectType::LINE,
        ));
    }

    for item in tsa.data.iter() {
        objects.push(graphics_engine::Object::create_center(
            item.point.x,
            item.point.y,
            [1.0; 4],
            graphics_engine::ObjectType::CIRCLE,
        ));
    }

    return objects;
}

fn main() {
    let data = load("data/inst_100.txt");

    let initial_temperature = 200.0;
    let final_temperature = 1E-5;
    let qtd_iters = 2_000_000;
    let qtd_iters_on_temp = 10;
    let config = TSAConfig::<ExpCooling>::create(
        final_temperature,
        initial_temperature,
        qtd_iters,
        qtd_iters_on_temp,
    );

    let (sender_signal, receiver_signal) = channel::<bool>();
    let (sender_data, receiver_data) = channel::<Vec<graphics_engine::Object>>();

    let mut tsa = TSA::create(data, config);

    let max_x = tsa.data.iter().map(|item| item.point.x).max().unwrap();
    let max_y = tsa.data.iter().map(|item| item.point.y).max().unwrap();
    let min_x = tsa.data.iter().map(|item| item.point.x).min().unwrap();
    let min_y = tsa.data.iter().map(|item| item.point.y).min().unwrap();

    spawn(move || {
        loop {
            tsa.gen_next_solution();
            
            let signal = receiver_signal.try_recv();
            match signal {
                Ok(msg) => {
                    if msg {
                        sender_data.send(handle_update(&tsa)).unwrap();
                    } else {
                        break;
                    }
                }
                Err(_) => {}
            }
        }
        println!("Sim iters {}", tsa.get_current_iter());
    });

    let mut app = App::create("TSA", max_y + min_y, max_x + min_x);

    let mut events = EventsBridge::create();
    while let Some(e) = events.next(&mut app.window_handle) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            sender_signal.send(true).unwrap();
            let objects = receiver_data.recv().unwrap();
            app.update(&args, objects);
        }

        if let Some(_) = e.close_args() {
            sender_signal.send(false).unwrap();
        }
    }
}
