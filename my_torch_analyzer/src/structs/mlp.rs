use serde::{Deserialize, Serialize};
use std::fs;

use crate::structs::{
    chess_fen_notation::ChessFenNotation, enum_chess_prediction::ChessPrediction,
    hyper_parameters::HyperParameters, layers::Layers,
};

#[derive(Serialize, Deserialize)]
pub struct MLP {
    epochs: u32,
    layers: Vec<Layers>,
}

impl MLP {
    pub fn new(parameters: HyperParameters) -> MLP {
        let mut layers: Vec<Layers> = Vec::new();

        let mut current_input_size = parameters.inputs;

        for _ in 0..parameters.hidden_layers {
            layers.push(Layers::new(
                current_input_size,
                parameters.learning_rate,
                parameters.hidden_neurons,
            ));
            current_input_size = parameters.hidden_neurons;
        }

        layers.push(Layers::new(
            current_input_size,
            parameters.learning_rate,
            parameters.outputs,
        ));

        MLP {
            epochs: parameters.epochs,
            layers,
        }
    }

    pub fn forward(&mut self, initial_input: &Vec<f64>) -> Vec<Vec<f64>> {
        let mut history: Vec<Vec<f64>> = Vec::new();

        history.push(initial_input.clone());
        for layer in self.layers.iter_mut() {
            let current_input = history.last().unwrap();
            let layer_output = layer.forward_layer(current_input);
            history.push(layer_output);
        }

        history
    }

    pub fn backpropagation(&mut self, target_vector: Vec<f64>) {
        let len = self.layers.len();

        self.layers[len - 1].compute_output_deltas(&target_vector);
        for i in (0..len - 1).rev() {
            let (left, right) = self.layers.split_at_mut(i + 1);

            let current_layer = &mut left[i];
            let next_layer = &right[0];

            current_layer.compute_hidden_deltas(next_layer);
        }
    }

    pub fn update_weights(&mut self, history: &Vec<Vec<f64>>) {
        for i in 0..self.layers.len() {
            self.layers[i].update_layer_weights(&history[i]);
        }
    }

    pub fn fit(&mut self, input_file: &mut Vec<ChessFenNotation>) -> &mut Self {
        let total_lines = input_file.len() as f64;

        for epoch in 1..=self.epochs {
            let mut total_loss = 0.0;
            let mut correct_predictions = 0;

            for line in input_file.iter_mut() {
                let history = self.forward(&line.board);
                let outputs = history.last().unwrap();
                let targets = ChessPrediction::get_prediction_to_vec_f64(line.prediction.clone());

                let mut local_loss = 0.0;
                for i in 0..outputs.len() {
                    local_loss += (targets[i] - outputs[i]).powi(2);
                }
                total_loss += local_loss / outputs.len() as f64;

                let max_output_index = outputs
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(idx, _)| idx)
                    .unwrap();
                let max_target_index = targets
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(idx, _)| idx)
                    .unwrap();

                if max_output_index == max_target_index {
                    correct_predictions += 1;
                }
                self.backpropagation(targets);
                self.update_weights(&history);
            }

            let average_loss = total_loss / total_lines;
            let accuracy = (correct_predictions as f64 / total_lines) * 100.0;

            println!(
                "-> Epoch [{:02}/{:02}] | Loss: {:.4} | Accuracy: {:.2}%",
                epoch, self.epochs, average_loss, accuracy
            );
        }

        self
    }

    pub fn predict(&mut self, board: &Vec<f64>) {
        let history = self.forward(board);
        let outputs = history.last().unwrap();
        let max_index = outputs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        println!(
            "I found: {}",
            ChessPrediction::index_to_chess_prediction(max_index)
        )
    }

    pub fn save(&self, filename: String) {
        let encoded = bincode::serialize(self).unwrap();
        let res = fs::write(filename, encoded);
        match res {
            Ok(_) => {}
            Err(e) => eprint!("{}", e),
        }
    }
}
