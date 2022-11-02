pub trait CoolingMethod {
    fn get_next_temperature(&self, current_iter: usize) -> f64;
}

pub struct SigmoidCooling {
    initial_temperature: f64,
    final_temperature: f64,
    qtd_iters: usize,
}

impl CoolingMethod for SigmoidCooling {
    fn get_next_temperature(&self, current_iter: usize) -> f64 {
        let delta_temp = self.initial_temperature - self.final_temperature;
        let n = self.qtd_iters as f64;
        let a = delta_temp * (n + 1.0) / n;
        let b = self.initial_temperature - a;
        let new_temp = a / (current_iter as f64 + 1.0) + b;
        return new_temp;
    }
}

impl SigmoidCooling {
    pub fn create(initial_temperature: f64, final_temperature: f64, qtd_iters: usize) -> Self {
        SigmoidCooling {
            initial_temperature,
            final_temperature,
            qtd_iters,
        }
    }
}
