use neurosim_rust::neurons::{leaky_integrate_and_fire::{LeakyIntegrateAndFireNeuron, ResetStrategy }, Neuron};
use neurosim_rust::plotting_helpers::print_chart;

pub fn main(){
    let resting_potential = -70.0;
    let membrane_resistance = 5.0;
    let membrane_capacity = 2.0;
    let input_current = 8.0;

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
    let tmax = 10.0;
    let t = (0..=(tmax / dt) as usize)
        .map(|i| i as f64 * dt)
        .collect::<Vec<f64>>();

    let mut membrane_potential_result = vec![0.0; t.len()];
    membrane_potential_result[0] = neuron_under_test.membrane_potential;

    for i in 1..t.len() {
        membrane_potential_result[i] = neuron_under_test.simulate(dt, input_current);
    }

    print_chart(&t, membrane_potential_result, "neuron.png");

}