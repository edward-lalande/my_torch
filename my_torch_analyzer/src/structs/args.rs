use std::fmt;

pub struct TorchArgs {
    pub is_predict: bool,
    pub is_train: bool,
    pub save_file: String,
    pub load_file: String,
    pub input_file: String,
}

impl TorchArgs {
    pub fn new() -> Self {
        Self {
            is_predict: false,
            is_train: false,
            save_file: String::new(),
            load_file: String::new(),
            input_file: String::new(),
        }
    }

    pub fn parse_args(args: Vec<String>) -> TorchArgs {
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

    pub fn is_valid_args(&self) -> bool {
        ((self.is_predict && self.is_train) || (!self.is_predict && !self.is_train))
            || (self.is_train && self.save_file.is_empty())
            || (self.load_file.is_empty() || self.input_file.is_empty())
    }

    pub fn print_usage(&self) {
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
    }
}

impl fmt::Debug for TorchArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TorchArgs {{ is_predict: {}, is_train: {}, save_file: {}, load_file: {}, input_file: {} }}",
            self.is_predict, self.is_train, self.save_file, self.load_file, self.input_file
        )
    }
}
