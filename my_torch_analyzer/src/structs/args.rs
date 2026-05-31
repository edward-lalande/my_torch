use std::fmt;

pub struct TorchArgs {
    pub model: String,
    pub input: u64,
    pub save_file: String,
    pub input_file: String,
}

impl TorchArgs {
    pub fn new() -> Self {
        Self {
            model: String::new(),
            input: 0,
            save_file: String::new(),
            input_file: String::new(),
        }
    }
}

impl fmt::Debug for TorchArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TorchArgs {{ model: {}, input: {}, save_file: {}, input_file: {} }}",
            self.model, self.input, self.save_file, self.input_file
        )
    }
}
