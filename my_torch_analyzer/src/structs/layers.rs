use crate::structs::neuron::Neuron;

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

    pub fn activation_function(&mut self) {}
}
