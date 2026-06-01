use std::env;
use std::process::exit;
mod structs;
use structs::args::*;

use crate::structs::chess_fen_notation::ChessFenNotation;
use crate::structs::hyper_parameters::HyperParameters;
use crate::structs::mlp::MLP;
pub mod utils;

fn my_torch_analyzer(
    chess_file: Vec<ChessFenNotation>,
    parameters: HyperParameters,
    torch_args: TorchArgs,
) {
    let mut mlp = MLP::new(parameters);

    if torch_args.is_train {
        mlp.fit(chess_file);
    } else if torch_args.is_predict {
        mlp.predict(chess_file);
    }
}

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
    let chess_file = ChessFenNotation::chess_fen_notation_from_file(torch_args.clone().input_file);
    my_torch_analyzer(chess_file, parameters, torch_args)
}
