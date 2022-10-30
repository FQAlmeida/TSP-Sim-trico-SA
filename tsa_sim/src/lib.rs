use data_retrieve::{load, Data, DataNode, Point};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct TSA {
    pub distances: Vec<Vec<f64>>,
    pub data: Vec<DataNode>,
    pub solution: Vec<usize>,
    current_distance: f64,
}

fn dist(a: &Point, b: &Point) -> f64 {
    let x_diff: f64 = b.x as f64 - a.x as f64;
    let y_diff: f64 = b.y as f64 - a.y as f64;
    let sum = x_diff * x_diff + y_diff * y_diff;

    sum.sqrt()
}

impl TSA {
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
        let mut rng = thread_rng();
        let qtd = rng.gen_range(1usize..=5);
        let initial_size = self.solution.len();
        let new_solution = TSA::permute(&mut self.solution.clone(), qtd);
        assert_eq!(initial_size, self.solution.len());

        let new_distance = self.get_solution_distance(&new_solution);

        if new_distance < self.current_distance {
            self.current_distance = new_distance;
            self.solution = new_solution;
            return;
        }

        // let value = rng.gen_range(0.0..=1.0);
        // if value <= 0.005 {
        //     self.current_distance = new_distance;
        //     self.solution = new_solution;
        // }
    }
    fn permute(solution: &mut Vec<usize>, qtd: usize) -> Vec<usize> {
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

impl TSA {
    fn create(data: Data) -> Self {
        let distances = Self::euclidian_distance_matrix(&data);
        let initial_solution = Self::get_initial_solution(data.len());
        let current_distance = Self::_get_solution_distance(&distances, &initial_solution);
        Self {
            distances,
            data,
            solution: initial_solution,
            current_distance,
        }
    }

    pub fn create_with_data() -> Self {
        let data = load("data/inst_51.txt");
        Self::create(data)
    }

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
}

#[cfg(test)]
mod tests {
    use crate::TSA;

    #[test]
    fn can_create_with_51_items_as_default() {
        let tsa = TSA::create_with_data();
        assert_eq!(tsa.distances.len(), 51);
    }
}
