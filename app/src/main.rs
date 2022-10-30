use graphics_engine::{App, EventsBridge};
use tsa_sim::TSA;

// println!("{:?}", tsa.solution);
// for _ in 0..10 {
//     tsa.gen_next_solution();
//     println!("{:?}", tsa.solution);
// }

fn handle_update(tsa: &mut TSA) -> Vec<graphics_engine::Object> {
    let mut objects: Vec<graphics_engine::Object> = vec![];
    tsa.gen_next_solution();
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
    let mut tsa = TSA::create_with_data();
    let max_x = tsa.data.iter().map(|item| item.point.x).max().unwrap();
    let max_y = tsa.data.iter().map(|item| item.point.y).max().unwrap();
    let min_x = tsa.data.iter().map(|item| item.point.x).min().unwrap();
    let min_y = tsa.data.iter().map(|item| item.point.y).min().unwrap();
    let mut app = App::create("TSA", max_y + min_y, max_x + min_x);

    let mut events = EventsBridge::create();
    while let Some(e) = events.next(&mut app.window_handle) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, handle_update(&mut tsa));
        }
    }
}
