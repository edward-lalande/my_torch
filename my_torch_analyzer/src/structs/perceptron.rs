#![allow(unused)]
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::process;

#[derive(Deserialize)]
pub struct Perceptron {
    inputs: u32,
    hidden_layers: u32,
    hidden_neurons: u32,
    outputs: u32,
    epochs: u32,
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
}

impl Perceptron {
    pub fn perceptron_from_file(filepath: String) -> Perceptron {
        let mut content: Vec<u8> = Vec::new();
        let mut fs = File::open(filepath.clone()).unwrap_or_else(|err| {
            eprintln!(
                "Application Error: Failed to open config file named: ({})! Details: {}",
                filepath, err
            );
            process::exit(1);
        });
        fs.read_to_end(&mut content).unwrap_or_else(|size| {
            eprintln!("Impossible to read file of size: {}", size);
            process::exit(1);
        });
        let slice = content.as_slice();
        let mut perceptron: Perceptron = bincode::deserialize(slice).unwrap();
        perceptron.weights.append(&mut vec![0.0, 0.0]);
        perceptron.bias = 0.0;

        perceptron
    }

    fn calcul_z(&self, input: (f64, f64)) -> f64 {
        self.weights[0] * input.0 + self.weights[1] * input.1 + self.bias
    }

    fn activation_function(&self, z: f64) -> bool {
        z >= 0.0
    }

    fn update_weight_bias(&mut self, input: (f64, f64), output: f64) {
        let prediction = self.predict(input);
        let error_rate = output - prediction;

        self.bias = self.bias + (self.learning_rate * error_rate);
        self.weights[0] = self.weights[0] + (self.learning_rate * error_rate * input.0);
        self.weights[1] = self.weights[1] + (self.learning_rate * error_rate * input.1);
    }

    pub fn fit(&mut self, inputs: Vec<(f64, f64)>, outputs: Vec<f64>) {
        if inputs.len() != outputs.len() {
            eprintln!("Input and output don't have the same len.");
            process::exit(1);
        }
        for _ in 0..self.epochs {
            for (input, output) in inputs.iter().zip(outputs.iter()) {
                self.update_weight_bias(*input, *output);
            }
        }
    }

    pub fn predict(&self, input: (f64, f64)) -> f64 {
        let z = self.calcul_z(input);
        f64::from(self.activation_function(z))
    }
}

impl fmt::Debug for Perceptron {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Perceptron {{ inputs: {}, hidden_layers: {}, hidden_neurons: {}, outputs: {}, epochs: {}, learning_rate: {}, weights: [{}, {}], bias: {} }}",
            self.inputs,
            self.hidden_layers,
            self.hidden_neurons,
            self.outputs,
            self.epochs,
            self.learning_rate,
            self.weights[0],
            self.weights[1],
            self.bias
        )
    }
}
