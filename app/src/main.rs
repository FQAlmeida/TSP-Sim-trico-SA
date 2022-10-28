use tsa_sim::TSA;

fn main() {
    let mut tsa = TSA::create_with_data();
    println!("{:?}", tsa.solution);
    for _ in 0..10 {
        tsa.gen_next_solution();
        println!("{:?}", tsa.solution);
    }
}
