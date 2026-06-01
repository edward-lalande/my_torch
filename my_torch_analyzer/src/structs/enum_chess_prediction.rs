use std::fmt;

#[derive(Clone)]
pub enum ChessPrediction {
    Init,
    Nothing,
    CheckWhite,
    CheckBlack,
    CheckMateWhite,
    CheckMateBlack,
}

impl ChessPrediction {
    pub fn get_prediction_to_vec_f64(chess_prediction: ChessPrediction) -> Vec<f64> {
        match chess_prediction {
            ChessPrediction::Nothing => vec![1.0, 0.0, 0.0, 0.0, 0.0],
            ChessPrediction::CheckWhite => vec![0.0, 1.0, 0.0, 0.0, 0.0],
            ChessPrediction::CheckBlack => vec![0.0, 0.0, 1.0, 0.0, 0.0],
            ChessPrediction::CheckMateWhite => vec![0.0, 0.0, 0.0, 1.0, 0.0],
            ChessPrediction::CheckMateBlack => vec![0.0, 0.0, 0.0, 0.0, 1.0],
            _ => {
                vec![]
            }
        }
    }

    pub fn index_to_chess_prediction(idx: usize) -> ChessPrediction {
        match idx {
            0 => ChessPrediction::Nothing,
            1 => ChessPrediction::CheckWhite,
            2 => ChessPrediction::CheckBlack,
            3 => ChessPrediction::CheckMateWhite,
            4 => ChessPrediction::CheckMateBlack,
            _ => ChessPrediction::Init,
        }
    }

    pub fn string_to_chess_prediction(value: &str) -> ChessPrediction {
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
                ChessPrediction::CheckWhite => "Check White",
                ChessPrediction::CheckBlack => "Check Black",
                ChessPrediction::CheckMateWhite => "CheckMate White",
                ChessPrediction::CheckMateBlack => "CheckMate Black",
                _ => "Init",
            }
        )
    }
}
