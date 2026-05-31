use std::env;
use std::process::exit;
mod structs;
use structs::args::*;

use crate::structs::perceptron::Perceptron;

fn parse_args(args: Vec<String>) -> TorchArgs {
    let mut torch_args = TorchArgs::new();
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--new" => {
                if let Some(val) = iter.next() {
                    torch_args.input = val.parse().unwrap_or(0);
                }
            }
            "--load" => {
                if let Some(val) = iter.next() {
                    torch_args.model = val;
                }
            }
            "--save" => {
                if let Some(val) = iter.next() {
                    torch_args.save_file = val;
                }
            }

            _ => {
                torch_args.input_file = arg;
            }
        }
    }

    torch_args
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let torch_args = parse_args(args);
    if !torch_args.model.is_empty() && torch_args.input != 0 {
        println!("Impossible to have a load file and create a new one");
        exit(1);
    }
    let inputs: Vec<(f64, f64)> = vec![(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)];
    let outputs: Vec<f64> = vec![0.0, 0.0, 0.0, 1.0];
    let mut perceptron: Perceptron = Perceptron::perceptron_from_file(torch_args.model);

    perceptron.fit(inputs, outputs);

    println!("Perceptron parameters: {:?}", perceptron);

    println!("0 & 0 = {}", perceptron.predict((0.0, 0.0))); // Must show 0
    println!("1 & 0 = {}", perceptron.predict((1.0, 0.0))); // Must show 0
    println!("0 & 1 = {}", perceptron.predict((0.0, 1.0))); // Must show 0
    println!("1 & 1 = {}", perceptron.predict((1.0, 1.0))); // Must show 1
}
