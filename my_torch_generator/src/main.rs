mod perceptron;
use perceptron::Perceptron;
use std::env;
use std::fs;

fn parse_file(config_file: &str) -> Perceptron {
    let content_file = fs::read_to_string(config_file).unwrap();
    let perceptron_model: Perceptron = serde_json::from_str(content_file.as_str()).unwrap();

    perceptron_model
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./my_torch_generator config_file_1");
        return;
    }

    let config_file = &args[1];
    let perceptron = parse_file(config_file);
    let encoded_vec: Vec<u8> = bincode::serialize(&perceptron).unwrap();
    let _ = fs::write("perceptron.nn", encoded_vec);
}
