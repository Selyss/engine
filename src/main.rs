use std::io;

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    piece_type: Option<PieceType>,
    piece_color: Option<PieceColor>,
}

#[derive(Debug, Clone, Copy)]
pub struct Board(pub [[Square; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let squares = Board(
            [[Square {
                piece_type: None,
                piece_color: None,
            }; 8]; 8],
        );
        squares
    }
    pub fn init(&mut self) {
        let white_back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        let black_back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        for col in 0..8 {
            self.0[7][col] = Square {
                piece_type: Some(white_back_rank[col]),
                piece_color: Some(PieceColor::White),
            };

            self.0[6][col] = Square {
                piece_type: Some(PieceType::Pawn),
                piece_color: Some(PieceColor::White),
            };

            self.0[1][col] = Square {
                piece_type: Some(PieceType::Pawn),
                piece_color: Some(PieceColor::Black),
            };

            self.0[0][col] = Square {
                piece_type: Some(black_back_rank[col]),
                piece_color: Some(PieceColor::Black),
            };
        }
    }
    pub fn parse_move(&self, input: &str) -> Option<((usize, usize), (usize, usize))> {
        if input.len() != 4 {
            return None;
        }

        let source_col = input.chars().nth(0)?;
        let source_row = input.chars().nth(1)?;
        let dest_col = input.chars().nth(2)?;
        let dest_row = input.chars().nth(3)?;

        let source_col_index = (source_col as usize) - ('a' as usize);
        let source_row_index = 8 - source_row.to_digit(10)? as usize;
        let dest_col_index = (dest_col as usize) - ('a' as usize);
        let dest_row_index = 8 - dest_row.to_digit(10)? as usize;

        if source_col_index >= 8
            || source_row_index >= 8
            || dest_col_index >= 8
            || dest_row_index >= 8
        {
            return None;
        }

        Some((
            (source_row_index, source_col_index),
            (dest_row_index, dest_col_index),
        ))
    }
    pub fn perform_move(&mut self, source: (usize, usize), destination: (usize, usize)) {
        let piece = self.0[source.0][source.1];
        self.0[destination.0][destination.1] = piece;
        self.0[source.0][source.1] = Square {
            piece_type: None,
            piece_color: None,
        };
    }
}

fn main() {
    let mut board = Board::new();
    board.init();

    loop {}
}
