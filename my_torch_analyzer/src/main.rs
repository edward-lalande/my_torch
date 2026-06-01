use std::env;
use std::process::exit;
mod structs;
use structs::args::*;

use crate::structs::hyper_parameters::HyperParameters;
pub mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let torch_args = TorchArgs::parse_args(args);

    if torch_args.is_valid_args() {
        torch_args.print_usage();
        exit(1);
    }
    let parameters: HyperParameters =
        bincode::deserialize(&std::fs::read(&torch_args.load_file).unwrap_or_else(|err| {
            eprintln!("{}", err);
            exit(1)
        }))
        .unwrap();

    println!("{:?}", parameters);
}
