use crate::structs::enum_chess_prediction::ChessPrediction;
use crate::utils::file_to_vec_string::read_lines_to_vec;
use std::fmt;

pub struct ChessFenNotation {
    board: Vec<f64>,
    actual_turn: char,
    nb_semi_turn: u32,
    nb_turn: u32,
    prediction: ChessPrediction,
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
    fn board_to_float_vector(&self, board_line: String) -> Vec<f64> {
        let mut board: Vec<f64> = Vec::new();

        for c in board_line.chars() {
            match c {
                // white pieces
                'P' => board.push(-0.1),
                'N' => board.push(-0.3),
                'B' => board.push(-0.35),
                'R' => board.push(-0.5),
                'Q' => board.push(-0.9),
                'K' => board.push(-1.0),
                // black pieces
                'p' => board.push(0.1),
                'n' => board.push(0.3),
                'b' => board.push(0.35),
                'r' => board.push(0.5),
                'q' => board.push(0.9),
                'k' => board.push(1.0),
                // no pieces on case
                '1'..'8' => {
                    for _ in 0..u32::from(c) - 48 {
                        board.push(0.0);
                    }
                }
                _ => {}
            }
        }

        board
    }

    // Example of line
    // 8/8/R2k4/4r1p1/8/5K2/5P2/8 b - - 7 59 Check White
    fn chess_fen_notation_from_line(&mut self, line: String) -> ChessFenNotation {
        let mut chess_line_notation = ChessFenNotation::new();
        let mut arr = line.split(" ");

        chess_line_notation.board = self.board_to_float_vector(arr.next().unwrap().to_string());
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

    pub fn chess_fen_notation_from_file(&mut self, filepath: String) -> Vec<ChessFenNotation> {
        let mut chess_vec: Vec<ChessFenNotation> = Vec::new();
        let lines = read_lines_to_vec(filepath);

        for line in lines {
            chess_vec.push(self.chess_fen_notation_from_line(line));
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
