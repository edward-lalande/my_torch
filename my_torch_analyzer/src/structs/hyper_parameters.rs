use std::fmt;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct HyperParameters {
    pub inputs: u32,
    pub hidden_layers: u32,
    pub hidden_neurons: u32,
    pub outputs: u32,
    pub epochs: u32,
    pub learning_rate: f64,
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
