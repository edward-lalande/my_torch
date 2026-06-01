mod hyper_parameters;
use hyper_parameters::HyperParameters;
use std::env;
use std::fs;

fn parse_file(config_file: &str) -> HyperParameters {
    let content_file = fs::read_to_string(config_file).unwrap();
    let parameters_model: HyperParameters = serde_json::from_str(content_file.as_str()).unwrap();

    parameters_model
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
    let _ = fs::write("hyper_parameters.nn", encoded_vec);
}
