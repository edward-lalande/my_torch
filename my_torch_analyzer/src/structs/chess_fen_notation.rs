use crate::structs::enum_chess_prediction::ChessPrediction;
use crate::utils::file_to_vec_string::read_lines_to_vec;
use std::fmt;

#[derive(Clone)]
pub struct ChessFenNotation {
    pub board: Vec<f64>,
    pub actual_turn: char,
    pub nb_semi_turn: u32,
    pub nb_turn: u32,
    pub prediction: ChessPrediction,
}

impl ChessFenNotation {
    pub fn new() -> ChessFenNotation {
        return ChessFenNotation {
            board: Vec::new(),
            actual_turn: '\0',
            nb_semi_turn: 0,
            nb_turn: 0,
            prediction: ChessPrediction::Init,
        };
    }

    // To apply a MLP to the chess fen notation the board have to be translated
    // into a vec of f64 where each peaces have a numerical value
    fn board_to_float_vector(board_line: &str) -> Vec<f64> {
        let mut board: Vec<f64> = Vec::with_capacity(64);

        for c in board_line.chars() {
            match c {
                // White pieces values
                'P' => board.push(-0.1),
                'N' => board.push(-0.3),
                'B' => board.push(-0.35),
                'R' => board.push(-0.5),
                'Q' => board.push(-0.9),
                'K' => board.push(-1.0),
                // Black pieces values
                'p' => board.push(0.1),
                'n' => board.push(0.3),
                'b' => board.push(0.35),
                'r' => board.push(0.5),
                'q' => board.push(0.9),
                'k' => board.push(1.0),
                // Empty cases
                '1'..='8' => {
                    if let Some(count) = c.to_digit(10) {
                        for _ in 0..count {
                            board.push(0.0);
                        }
                    }
                }
                // Ignore FEN separator
                '/' => {}
                _ => {}
            }
        }

        board
    }

    // Example of line
    // 8/8/R2k4/4r1p1/8/5K2/5P2/8 b - - 7 59 Check White (Check white is ignored when we are in prediction mode)
    fn chess_fen_notation_from_line(line: String, is_predict: bool) -> ChessFenNotation {
        let mut chess_line_notation = ChessFenNotation::new();
        let mut arr = line.split(' ');

        chess_line_notation.board = Self::board_to_float_vector(arr.next().unwrap());
        chess_line_notation.actual_turn = arr.next().unwrap().chars().next().unwrap();

        arr.next(); // pass '-'
        arr.next(); // pass '-'

        chess_line_notation.nb_semi_turn = arr.next().unwrap().parse::<u32>().unwrap();
        chess_line_notation.nb_turn = arr.next().unwrap().parse::<u32>().unwrap();

        if !is_predict {
            let pred_str = format!("{} {}", arr.next().unwrap(), arr.next().unwrap_or(""));
            chess_line_notation.prediction =
                ChessPrediction::string_to_chess_prediction(pred_str.trim());
        } else {
            chess_line_notation.prediction = ChessPrediction::Init;
        }

        chess_line_notation
    }

    pub fn chess_fen_notation_from_file(
        filepath: String,
        is_predict: bool,
    ) -> Vec<ChessFenNotation> {
        let mut chess_vec: Vec<ChessFenNotation> = Vec::new();
        let lines = read_lines_to_vec(filepath);

        for line in lines {
            chess_vec.push(Self::chess_fen_notation_from_line(line, is_predict));
        }

        chess_vec
    }
}

impl fmt::Debug for ChessFenNotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ChessFenNotation {{ board_vector: {:?}, actual_turn: '{}', nb_semi_turn: {}, nb_turn: {}, prediction: {:?} }}",
            self.board, self.actual_turn, self.nb_semi_turn, self.nb_turn, self.prediction
        )
    }
}
