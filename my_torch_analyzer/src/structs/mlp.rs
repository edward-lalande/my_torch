use crate::structs::{
    chess_fen_notation::ChessFenNotation, hyper_parameters::HyperParameters, layers::Layers,
};

pub struct MLP {
    learning_rate: f64,
    epoch: u32,
    layers: Vec<Layers>,
}

impl MLP {
    pub fn new(parameters: HyperParameters) -> MLP {
        let mut layers: Vec<Layers> = Vec::new();

        for i in 0..parameters.hidden_layers {
            layers.push(Layers::new(
                if i == 0 {
                    parameters.inputs
                } else {
                    parameters.hidden_neurons
                },
                parameters.learning_rate,
                parameters.hidden_neurons,
            ));
        }

        layers.push(Layers::new(
            parameters.hidden_neurons,
            parameters.learning_rate,
            parameters.outputs,
        ));

        MLP {
            learning_rate: parameters.learning_rate,
            epoch: parameters.epochs,
            layers,
        }
    }

    pub fn fit(&mut self, input_file: Vec<ChessFenNotation>) {}
    pub fn predict(&mut self, input_file: Vec<ChessFenNotation>) {}
}
