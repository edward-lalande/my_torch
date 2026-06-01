pub struct Neuron {
    learning_rate: f64,
    output: f64,
    delta: f64,
    bias: f64,
    weights: Vec<f64>,
}

impl Neuron {
    pub fn new(weights_size: u32, learning_rate: f64) -> Neuron {
        Neuron {
            learning_rate: learning_rate,
            output: 0.0,
            delta: 0.0,
            bias: 0.0,
            weights: vec![rand::random(); weights_size as usize],
        }
    }
    pub fn calcul_delta(&mut self) {}
    pub fn update_weight_and_bias(&mut self) {}
}
