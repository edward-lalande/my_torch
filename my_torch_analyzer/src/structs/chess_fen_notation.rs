use std::fmt;

use crate::structs::enum_chess_prediction::ChessPrediction;

pub struct ChessFenNotation {
    board: String,
    actual_turn: char,
    nb_semi_turn: u32,
    nb_turn: u32,
    prediction: ChessPrediction,
}

impl ChessFenNotation {
    pub fn new() -> ChessFenNotation {
        return ChessFenNotation {
            board: "".to_string(),
            actual_turn: '\0',
            nb_semi_turn: 0,
            nb_turn: 0,
            prediction: ChessPrediction::Init,
        };
    }

    // Example of line
    // 8/8/R2k4/4r1p1/8/5K2/5P2/8 b - - 7 59 Check White
    pub fn chess_fen_notation_from_line(line: String) -> ChessFenNotation {
        let mut chess_line_notation = ChessFenNotation::new();
        let mut arr = line.split(" ");

        chess_line_notation.board = arr.next().unwrap().to_string();
        chess_line_notation.actual_turn = arr.next().unwrap().to_string().chars().next().unwrap();

        arr.next(); // to pass out the '-' index
        arr.next(); // to pass out the '-' index

        chess_line_notation.nb_semi_turn = arr.next().unwrap().to_string().parse::<u32>().unwrap();
        chess_line_notation.nb_turn = arr.next().unwrap().to_string().parse::<u32>().unwrap();
        chess_line_notation.prediction = ChessPrediction::string_to_chess_prediction(
            (arr.next().unwrap().to_string() + " " + arr.next().unwrap_or_else(|| ""))
                .as_str()
                .trim(),
        );
        chess_line_notation
    }
}

impl fmt::Debug for ChessFenNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChessFenNotation {{ board: ({}), actual_turn: ({}), nb_semi_turn: {}, nb_turn: {}, prediction: ({}) }}",
            self.board, self.actual_turn, self.nb_semi_turn, self.nb_turn, self.prediction
        )
    }
}
