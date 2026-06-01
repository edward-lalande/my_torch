use std::env;
use std::process::exit;
mod structs;
use structs::args::*;

use crate::structs::chess_fen_notation::ChessFenNotation;
use crate::structs::hyper_parameters::HyperParameters;
use crate::structs::mlp::MLP;
pub mod utils;

fn my_torch_analyzer_updated(
    chess_file: &mut Vec<ChessFenNotation>,
    parameters: HyperParameters,
    existing_mlp: Option<MLP>,
    torch_args: TorchArgs,
) {
    if torch_args.is_train {
        let mut mlp = match existing_mlp {
            Some(loaded_mlp) => {
                println!("-> Get back to train, NO PAIN NO GAIN.");
                loaded_mlp
            }
            None => {
                println!("-> Initialization of a new MLP.");
                MLP::new(parameters)
            }
        };

        mlp.fit(chess_file);
        mlp.save(torch_args.save_file.to_string());
        println!("-> Progess saved in: ({}).", torch_args.save_file);
    } else if torch_args.is_predict {
        let mut mlp: MLP =
            bincode::deserialize(&std::fs::read(&torch_args.load_file).unwrap_or_else(|err| {
                eprintln!("{}", err);
                exit(1)
            }))
            .unwrap();
        for line in chess_file.iter() {
            mlp.predict(&line.board);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let torch_args = TorchArgs::parse_args(args);

    if torch_args.is_valid_args() {
        torch_args.print_usage();
        exit(1);
    }

    let file_bytes = std::fs::read(&torch_args.load_file).unwrap_or_else(|err| {
        eprintln!("Cannot read file {}: {}", torch_args.load_file, err);
        exit(1);
    });

    let mlp: Option<MLP> = bincode::deserialize(&file_bytes).ok();
    let parameters = if mlp.is_none() {
        match bincode::deserialize::<HyperParameters>(&file_bytes) {
            Ok(params) => params,
            Err(err) => {
                eprintln!(
                    "The specified files is not a valid model, nor a file of hyper parameters.\nError: {}",
                    err
                );
                exit(1);
            }
        }
    } else {
        HyperParameters {
            inputs: 0,
            hidden_layers: 0,
            hidden_neurons: 0,
            outputs: 0,
            epochs: 0,
            learning_rate: 0.0,
        }
    };

    let mut chess_file = ChessFenNotation::chess_fen_notation_from_file(
        torch_args.clone().input_file,
        torch_args.is_predict,
    );

    my_torch_analyzer_updated(&mut chess_file, parameters, mlp, torch_args);
}
