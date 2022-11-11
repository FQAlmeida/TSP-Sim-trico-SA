use std::f64::consts::PI;

pub trait CoolingMethod {
    fn get_next_temperature(&self, current_iter: usize) -> f64;
    fn create(initial_temperature: f64, final_temperature: f64, qtd_iters: usize) -> Self;
}

pub struct SigmoidCooling {
    initial_temperature: f64,
    final_temperature: f64,
    qtd_iters: usize,
}

pub struct ExpCooling {
    initial_temperature: f64,
    final_temperature: f64,
    qtd_iters: usize,
}

pub struct CosCooling {
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
    fn create(initial_temperature: f64, final_temperature: f64, qtd_iters: usize) -> Self {
        SigmoidCooling {
            initial_temperature,
            final_temperature,
            qtd_iters,
        }
    }
}

impl CoolingMethod for ExpCooling {
    fn get_next_temperature(&self, current_iter: usize) -> f64 {
        let exp = current_iter as f64 / self.qtd_iters as f64;
        let fraction = self.final_temperature / self.initial_temperature;
        let new_temp = self.initial_temperature * fraction.powf(exp);
        return new_temp;
    }

    fn create(initial_temperature: f64, final_temperature: f64, qtd_iters: usize) -> Self {
        ExpCooling {
            initial_temperature,
            final_temperature,
            qtd_iters,
        }
    }
}

impl CoolingMethod for CosCooling {
    fn get_next_temperature(&self, current_iter: usize) -> f64 {
        let p1 = 0.5 * (self.initial_temperature - self.final_temperature);
        let i = current_iter as f64;
        let pcos = i * PI / self.qtd_iters as f64;
        let p2 = 1.0 + pcos.cos();
        let p3 = self.final_temperature;
        let new_temp = p1 * p2 + p3;
        return new_temp;
    }

    fn create(initial_temperature: f64, final_temperature: f64, qtd_iters: usize) -> Self {
        CosCooling {
            initial_temperature,
            final_temperature,
            qtd_iters,
        }
    }
}
