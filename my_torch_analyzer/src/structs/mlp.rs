use crate::structs::{chess_fen_notation::ChessFenNotation, layers::Layers};

struct MLP {
    learning_rate: f64,
    layers: Vec<Layers>,
}

impl MLP {
    pub fn fit(&mut self, input_file: Vec<ChessFenNotation>) {}
    pub fn predict(&mut self, input_file: Vec<ChessFenNotation>) {}
}
