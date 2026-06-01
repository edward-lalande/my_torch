use std::process;

use serde::{Deserialize, Serialize};

use crate::structs::neuron::Neuron;

#[derive(Serialize, Deserialize)]
pub struct Layers {
    neurons: Vec<Neuron>,
}

impl Layers {
    pub fn new(weights_size: u32, learning_rate: f64, hidden_neurons: u32) -> Layers {
        let mut neurons: Vec<Neuron> = Vec::new();

        for _ in 0..hidden_neurons {
            neurons.push(Neuron::new(weights_size, learning_rate))
        }

        Layers { neurons }
    }

    pub fn forward_layer(&mut self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut sum_output: Vec<f64> = Vec::with_capacity(self.neurons.len());

        for neuron in self.neurons.iter_mut() {
            neuron.calcul_output(inputs);
            sum_output.push(neuron.output);
        }

        sum_output
    }

    pub fn compute_output_deltas(&mut self, targets: &Vec<f64>) {
        if self.neurons.len() != targets.len() {
            eprintln!("Neurons and target output don't have the same len.");
            process::exit(1);
        }

        for (neuron, target) in self.neurons.iter_mut().zip(targets.iter()) {
            neuron.calcul_delta_output(target);
        }
    }

    pub fn compute_hidden_deltas(&mut self, next_layer: &Layers) {
        for i in 0..self.neurons.len() {
            let mut upstream_error: f64 = 0.0;

            for j in 0..next_layer.neurons.len() {
                let next_neuron = &next_layer.neurons[j];
                upstream_error += next_neuron.weights[i] * next_neuron.delta;
            }
            self.neurons[i].calcul_delta_hidden(upstream_error);
        }
    }

    pub fn update_layer_weights(&mut self, inputs: &Vec<f64>) {
        for neuron in self.neurons.iter_mut() {
            neuron.update_weight_and_bias(inputs);
        }
    }
}
