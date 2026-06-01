use std::fmt;

pub enum ChessPrediction {
    Init,
    Nothing,
    CheckWhite,
    CheckBlack,
    CheckMateWhite,
    CheckMateBlack,
}

impl ChessPrediction {
    pub fn string_to_chess_prediction(value: &str) -> ChessPrediction {
        println!("value: {}", value);
        match value {
            "Nothing" => ChessPrediction::Nothing,
            "Check Black" => ChessPrediction::CheckBlack,
            "Check White" => ChessPrediction::CheckWhite,
            "Checkmate Black" => ChessPrediction::CheckMateBlack,
            "Checkmate White" => ChessPrediction::CheckMateWhite,
            _ => ChessPrediction::Init,
        }
    }
}

impl fmt::Debug for ChessPrediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChessPrediction: {}",
            match self {
                ChessPrediction::Nothing => "Nothing",
                ChessPrediction::CheckWhite => "CheckWhite",
                ChessPrediction::CheckBlack => "CheckBlack",
                ChessPrediction::CheckMateWhite => "CheckMateWhite",
                ChessPrediction::CheckMateBlack => "CheckMateBlack",
                _ => "Init",
            }
        )
    }
}

impl fmt::Display for ChessPrediction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ChessPrediction::Nothing => "Nothing",
                ChessPrediction::CheckWhite => "CheckWhite",
                ChessPrediction::CheckBlack => "CheckBlack",
                ChessPrediction::CheckMateWhite => "CheckMateWhite",
                ChessPrediction::CheckMateBlack => "CheckMateBlack",
                _ => "Init",
            }
        )
    }
}
