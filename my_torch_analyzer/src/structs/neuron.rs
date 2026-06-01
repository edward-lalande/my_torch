pub struct Neuron {
    learning_rate: f64,
    weights: Vec<f64>,
    bias: f64,
    output: f64,
    delta: f64,
}

impl Neuron {
    pub fn calcul_delta(&mut self) {}
    pub fn update_weight_and_bias(&mut self) {}
}
