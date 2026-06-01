use std::env;
use std::process::exit;
mod structs;
use structs::args::*;
pub mod utils;

fn parse_args(args: Vec<String>) -> TorchArgs {
    let mut torch_args = TorchArgs::new();
    let mut iter = args.into_iter();

    iter.next(); // to jump the binary file

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--predict" => {
                torch_args.is_predict = true;
            }
            "--train" => {
                torch_args.is_train = true;
            }
            "--save" => {
                if let Some(val) = iter.next() {
                    torch_args.save_file = val;
                }
            }

            _ => {
                torch_args.load_file = arg;
                if let Some(val) = iter.next() {
                    torch_args.input_file = val;
                }
            }
        }
    }

    torch_args
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let torch_args = parse_args(args);
    println!("is_predict: {}", torch_args.is_predict);
    println!("is_train: {}", torch_args.is_train);
    println!("save_file.is_empty(): {}", torch_args.save_file.is_empty());
    println!("load_file.is_empty(): {}", torch_args.load_file.is_empty());
    println!(
        "input_file.is_empty(): {}",
        torch_args.input_file.is_empty()
    );
    if ((torch_args.is_predict && torch_args.is_train)
        || (!torch_args.is_predict && !torch_args.is_train))
        || (torch_args.is_train && torch_args.save_file.is_empty())
        || (torch_args.load_file.is_empty() || torch_args.input_file.is_empty())
    {
        println!("USAGE");
        println!(
            "\t./my_torch_analyzer [--predict | --train [--save SAVEFILE]] LOADFILE CHESSFILE\n"
        );
        println!("DESCRIPTION");
        println!(
            "\t--train\tLaunch the neural network in training mode. Each chessboard in FILE must"
        );
        println!(
            "\t\tcontain inputs to send to the neural network in FEN notation and the expected output"
        );
        println!(
            "\t\tseparated by space. If specified, the newly trained neural network will be saved in"
        );
        println!("\t\tSAVEFILE. Otherwise, it will be saved in the original LOADFILE.");
        println!(
            "\t--predict\tLaunch the neural network in prediction mode. Each chessboard in FILE must"
        );
        println!(
            "\t\tcontain inputs to send to the neural network in FEN notation, and optionally an expected"
        );
        println!("\t\toutput.");
        println!("\t--save\tSave neural network into SAVEFILE. Only works in train mode.\n");
        println!("\tLOADFILE\tFile containing an artificial neural network");
        println!("\tCHESSFILE\tFile containing chessboards");

        exit(1);
    }
    println!("{:?}", torch_args);
}
