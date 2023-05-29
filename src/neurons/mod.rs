pub mod leaky_integrate_and_fire;

pub trait Neuron {
    fn simulate(&mut self, dt: f64, input_current: f64) -> f64;
}