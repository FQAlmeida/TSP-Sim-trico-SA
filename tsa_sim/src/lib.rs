pub mod cooling_methods;

use cooling_methods::{CoolingMethod, SigmoidCooling};
use data_retrieve::{Data, DataNode, Point};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct TSAConfig<T>
where
    T: CoolingMethod + 'static,
{
    pub initial_temperature: f64,
    pub final_temperature: f64,
    pub qtd_iters: usize,
    pub qtd_iters_on_temp: usize,
    pub cooling_method: T,
}

pub struct TSA<T>
where
    T: CoolingMethod + 'static,
{
    pub distances: Vec<Vec<f64>>,
    pub data: Vec<DataNode>,
    pub solution: Vec<usize>,
    current_distance: f64,
    temperature: f64,
    iters_on_temp: usize,
    current_iter: usize,
    config: TSAConfig<T>,
}

fn dist(a: &Point, b: &Point) -> f64 {
    let x_diff: f64 = b.x as f64 - a.x as f64;
    let y_diff: f64 = b.y as f64 - a.y as f64;
    let sum = x_diff * x_diff + y_diff * y_diff;

    sum.sqrt()
}

impl<T: CoolingMethod + 'static> TSA<T> {
    pub fn get_current_distance(&self) -> f64 {
        self.current_distance
    }

    pub fn get_current_temperature(&self) -> f64 {
        self.temperature
    }

    pub fn get_solution_distance(&self, solution: &Vec<usize>) -> f64 {
        return Self::_get_solution_distance(&self.distances, solution);
    }

    fn _get_solution_distance(distances: &Vec<Vec<f64>>, solution: &Vec<usize>) -> f64 {
        let mut dist = 0.0;
        let size = solution.len();
        for origem_index in 0..size {
            let origem = solution[origem_index];
            let destiny = solution[(origem_index + 1) % size]; // mod size to wrap to the first item
            dist += distances[origem][destiny];
        }
        return dist;
    }

    pub fn gen_next_solution(&mut self) {
        if self.current_iter >= self.config.qtd_iters {
            // dbg!(&self.solution);
            return;
        }
        // self.current_iter += 1;
        dbg!(self.temperature);
        // dbg!(self.current_distance);

        let mut rng = thread_rng();
        let qtd = rng.gen_range(1usize..=5);
        // println!("{}", qtd);
        let initial_size = self.solution.len();
        let new_solution = TSA::<T>::permute(&self.solution, qtd);
        assert_eq!(initial_size, self.solution.len());
        assert_eq!(initial_size, new_solution.len());

        let new_distance = self.get_solution_distance(&new_solution);

        if new_distance < self.current_distance || self.should_change(new_distance) {
            self.current_distance = new_distance;
            self.solution = new_solution;
            // return;
        }

        self.update_temperature();
        // let itera = self.current_iter * self.config.qtd_iters_on_temp + self.iters_on_temp;
        // println!("{} {} {}", itera, self.current_distance, self.temperature);
    }

    fn should_change(&self, new_distance: f64) -> bool {
        if self.temperature <= self.config.final_temperature {
            return false;
        }
        let mut rng = thread_rng();
        let value = rng.gen_range(0.0..=1.0);
        let e = std::f64::consts::E;
        let delta = new_distance - self.current_distance;
        let prob = e.powf(-delta / self.temperature);
        // println!("-------------------------------------");
        // println!("prob {}", prob);
        // println!("distance {}", delta);
        // println!("temp {}", self.temperature);
        // println!("-------------------------------------");
        assert!(0.0 <= prob && prob <= 1.0);
        return value <= prob;
    }

    fn update_temperature(&mut self) {
        self.iters_on_temp += 1;
        self.current_iter += 1;
        if self.iters_on_temp % self.config.qtd_iters_on_temp != 0 {
            return;
        }
        self.iters_on_temp = 0;

        // self.cooling_method

        self.temperature = self
            .config
            .cooling_method
            .get_next_temperature(self.current_iter);
    }

    fn permute(solution: &Vec<usize>, qtd: usize) -> Vec<usize> {
        let size = solution.len();
        let mut rng = thread_rng();
        let mut new_solution = solution.clone();

        for i in 0..solution.len() {
            assert_eq!(solution[i], new_solution[i]);
        }

        for _ in 0..qtd {
            let mut index_1: usize;
            let mut index_2: usize;
            loop {
                index_1 = rng.gen_range(0..size);
                index_2 = rng.gen_range(0..size);
                if index_1 != index_2 {
                    break;
                }
            }

            let value_1 = new_solution[index_1];
            let value_2 = new_solution[index_2];

            new_solution[index_1] = value_2;
            new_solution[index_2] = value_1;

            assert_eq!(value_1, new_solution[index_2]);
            assert_eq!(value_2, new_solution[index_1]);
            assert_ne!(value_1, value_2);
            assert_ne!(new_solution[index_1], new_solution[index_2]);
        }
        return new_solution;
    }
}

impl TSAConfig<SigmoidCooling> {
    pub fn create_default() -> Self {
        let final_temperature = 0.0001;
        let initial_temperature = 10.0;
        let qtd_iters = 1000000;
        let qtd_iters_on_temp = 10;
        Self::create(
            final_temperature,
            initial_temperature,
            qtd_iters,
            qtd_iters_on_temp,
        )
    }
}

impl<T: CoolingMethod + 'static> TSAConfig<T> {
    pub fn create(
        final_temperature: f64,
        initial_temperature: f64,
        qtd_iters: usize,
        qtd_iters_on_temp: usize,
    ) -> Self {
        Self {
            final_temperature,
            initial_temperature,
            qtd_iters,
            qtd_iters_on_temp,
            cooling_method: T::create(initial_temperature, final_temperature, qtd_iters),
        }
    }
}

impl<T: CoolingMethod + 'static> TSA<T> {
    pub fn create(data: Data, config: TSAConfig<T>) -> Self {
        let distances = Self::euclidian_distance_matrix(&data);
        let initial_solution = Self::get_initial_solution(data.len());
        let current_distance = Self::_get_solution_distance(&distances, &initial_solution);
        let initial_temperature = config.initial_temperature;
        Self {
            distances,
            data,
            solution: initial_solution,
            current_distance,
            temperature: initial_temperature,
            iters_on_temp: 0,
            current_iter: 0,
            config,
        }
    }
}

impl<T: CoolingMethod + 'static> TSA<T> {
    fn get_initial_solution(len: usize) -> Vec<usize> {
        let mut solution = (0..len).collect::<Vec<usize>>();
        solution.shuffle(&mut thread_rng());
        return solution;
    }

    fn euclidian_distance_matrix(data: &Data) -> Vec<Vec<f64>> {
        let mut matrix = vec![vec![0.0; data.len()]; data.len()];
        for item_1 in data {
            for item_2 in data {
                let index_1 = item_1.group as usize - 1;
                let index_2 = item_2.group as usize - 1;
                let distance = dist(&item_1.point, &item_2.point);
                matrix[index_1][index_2] = distance;
                matrix[index_2][index_1] = distance;
            }
        }
        return matrix;
    }

    pub fn get_current_iter(&self) -> usize {
        self.current_iter
    }
}

#[cfg(test)]
mod tests {
    use crate::{TSAConfig, TSA};

    #[test]
    fn can_create_with_51_items_as_default() {
        let data = data_retrieve::load("../data/inst_51.txt");
        let config = TSAConfig::create_default();
        let tsa = TSA::create(data, config);
        assert_eq!(tsa.distances.len(), 51);
    }
}
