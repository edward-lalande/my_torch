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
