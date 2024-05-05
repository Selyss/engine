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
        for col in 0..8 {
            self.0[1][col] = Square {
                piece_type: Some(PieceType::Pawn),
                piece_color: Some(PieceColor::White),
            };
        }

        for col in 0..8 {
            self.0[6][col] = Square {
                piece_type: Some(PieceType::Pawn),
                piece_color: Some(PieceColor::Black),
            };
        }
    }
    pub fn print(&self) {
        for row in 0..8 {
            for col in 0..8 {
                print!(
                    "{}",
                    match self.0[row][col].piece_type {
                        Some(PieceType::Pawn) => 'P',
                        Some(PieceType::Knight) => 'N',
                        Some(PieceType::Bishop) => 'B',
                        Some(PieceType::Rook) => 'R',
                        Some(PieceType::Queen) => 'Q',
                        Some(PieceType::King) => 'K',
                        None => '.',
                    }
                );
            }
            println!();
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.init();
    board.print();
}
