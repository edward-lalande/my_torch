use serde::{Deserialize, Serialize};
use std::process::exit;

#[derive(Serialize, Deserialize)]
pub struct Neuron {
    learning_rate: f64,
    pub output: f64,
    pub delta: f64,
    bias: f64,
    pub weights: Vec<f64>,
}

impl Neuron {
    pub fn new(weights_size: u32, learning_rate: f64) -> Neuron {
        let bounds = (6.0 / (weights_size as f64)).sqrt();
        let mut weights = Vec::with_capacity(weights_size as usize);

        for _ in 0..weights_size {
            let random_value: f64 = rand::random::<f64>() * (2.0 * bounds) - bounds;
            weights.push(random_value);
        }

        Neuron {
            learning_rate,
            output: 0.0,
            delta: 0.0,
            bias: 0.0,
            weights,
        }
    }

    pub fn calcul_output(&mut self, inputs: &Vec<f64>) {
        if inputs.len() != self.weights.len() {
            eprintln!("Inputs and weights don't have the same len in calcul_output");
            eprintln!(
                "Inputs len: {}, self.weights.len: {}.",
                inputs.len(),
                self.weights.len()
            );
            exit(1);
        }

        let mut x = 0.0;
        for i in 0..self.weights.len() {
            x += inputs[i] * self.weights[i];
        }
        x += self.bias;

        self.output = if x > 0.0 { x } else { 0.01 * x };
    }

    pub fn calcul_delta_output(&mut self, target: &f64) {
        self.delta = (target - self.output) * self.activation_derivative();
    }

    pub fn calcul_delta_hidden(&mut self, upstream_error: f64) {
        self.delta = upstream_error * self.activation_derivative();
    }

    pub fn update_weight_and_bias(&mut self, inputs: &Vec<f64>) {
        if inputs.len() != self.weights.len() {
            eprintln!("Inputs and weights don't have the same len in update_weight_and_bias");
            exit(1);
        }

        self.bias += self.learning_rate * self.delta;

        for (weight, input) in self.weights.iter_mut().zip(inputs.iter()) {
            *weight += self.learning_rate * self.delta * *input;
        }
    }

    fn activation_derivative(&self) -> f64 {
        if self.output > 0.0 { 1.0 } else { 0.01 }
    }
}
