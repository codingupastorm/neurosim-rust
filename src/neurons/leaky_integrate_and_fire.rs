// Need to create a single Neuron for which we are able to simulate over time

// Configurable components:
// - Spiking + recharge strategy
// - Injected randomness strategy

use super::Neuron;


pub struct LeakyIntegrateAndFireNeuron {
    pub membrane_potential: f64,
    resting_potential: f64,
    membrane_resistance: f64,
    membrane_capacity: f64,
    spike_threshold: f64,
    reset_potential: f64
}

impl LeakyIntegrateAndFireNeuron {

    fn new_at_rest(resting_potential: f64, membrane_resistance: f64, membrane_capacity: f64, spike_threshold: f64, reset_potential: f64) 
    -> LeakyIntegrateAndFireNeuron {
        LeakyIntegrateAndFireNeuron { 
            membrane_potential: resting_potential,
            resting_potential, 
            membrane_resistance,
            membrane_capacity,
            spike_threshold,
            reset_potential
        }
    }
}

impl Neuron for LeakyIntegrateAndFireNeuron {

    fn simulate(&mut self, dt: f64, input_current: f64) -> f64 {
        let prev = self.membrane_potential;
        let dv_dt = (((self.resting_potential - prev) / self.membrane_resistance) + input_current)/ self.membrane_capacity;
        self.membrane_potential = prev + dv_dt * dt;
        return self.membrane_potential;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_simulate_neuron() {
        let resting_potential = -70.0;
        let membrane_resistance = 5.0;
        let membrane_capacity = 2.0;
        let input_current = 4.1;

        let spike_threshold = -50.0;
        let reset_potential = -65.0;

        let mut neuron_under_test = LeakyIntegrateAndFireNeuron::new_at_rest(
            resting_potential,
            membrane_resistance,
            membrane_capacity,
            spike_threshold,
            reset_potential
        );

        let dt = 0.001;
        let tmax = 100.0;
        let t = (0..=(tmax / dt) as usize)
            .map(|i| i as f64 * dt)
            .collect::<Vec<f64>>();

        let mut membrane_potential_result = vec![0.0; t.len()];

        for i in 1..t.len() {
            membrane_potential_result[i] = neuron_under_test.simulate(dt, input_current);
        }

        println!("{:?}", membrane_potential_result);

        // For some time, simulate
    }
}