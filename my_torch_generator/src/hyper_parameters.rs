use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HyperParameters {
    inputs: u32,
    hidden_layers: u32,
    hidden_neurons: u32,
    outputs: u32,
    epochs: u32,
    learning_rate: f64,
}

impl fmt::Debug for HyperParameters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HyperParameters {{ inputs: {}, hidden_layers: {}, hidden_neurons: {}, outputs: {}, epochs: {}, learning_rate: {} }}",
            self.inputs,
            self.hidden_layers,
            self.hidden_neurons,
            self.outputs,
            self.epochs,
            self.learning_rate
        )
    }
}
