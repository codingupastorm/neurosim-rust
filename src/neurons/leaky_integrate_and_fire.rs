// Configurable components:
// - Spiking + recharge strategy
// - Injected randomness strategy

// Future Questions
// - Should the neurons store their own history?

use super::Neuron;

pub struct LeakyIntegrateAndFireNeuron {
    pub membrane_potential: f64,
    resting_potential: f64,
    membrane_resistance: f64,
    membrane_capacity: f64,
    spike_threshold: f64,
    reset_potential: f64,
    reset_strategy: ResetStrategy,
}

pub enum ResetStrategy {
    INSTANT,
}

impl LeakyIntegrateAndFireNeuron {
    pub fn new_at_rest(
        resting_potential: f64,
        membrane_resistance: f64,
        membrane_capacity: f64,
        spike_threshold: f64,
        reset_potential: f64,
        reset_strategy: ResetStrategy
    ) -> LeakyIntegrateAndFireNeuron {
        LeakyIntegrateAndFireNeuron {
            membrane_potential: resting_potential,
            resting_potential,
            membrane_resistance,
            membrane_capacity,
            spike_threshold,
            reset_potential,
            reset_strategy
        }
    }

    fn check_for_reset(&mut self) {
        match self.reset_strategy {
            ResetStrategy::INSTANT => {
                if self.membrane_potential > self.spike_threshold {
                    self.membrane_potential = self.reset_potential;
                }
            }
        }
    }
}

impl Neuron for LeakyIntegrateAndFireNeuron {
    fn simulate(&mut self, dt: f64, input_current: f64) -> f64 {
        let prev = self.membrane_potential;
        let dv_dt = (((self.resting_potential - prev) / self.membrane_resistance) + input_current)
            / self.membrane_capacity;
        self.membrane_potential = prev + dv_dt * dt;

        self.check_for_reset();

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
            reset_potential,
            ResetStrategy::INSTANT
        );

        let dt = 0.001;
        let tmax = 100.0;
        let t = (0..=(tmax / dt) as usize)
            .map(|i| i as f64 * dt)
            .collect::<Vec<f64>>();

        let mut membrane_potential_result = vec![0.0; t.len()];
        membrane_potential_result[0] = neuron_under_test.membrane_potential;

        for i in 1..t.len() {
            membrane_potential_result[i] = neuron_under_test.simulate(dt, input_current);
        }
    }
}
