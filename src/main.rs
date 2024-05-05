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
    // maybe implement display for board?
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

    loop {
        board.print();

        // pure coordinate notation for now
        println!("Your move (e.g. 'e2e4'): ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if input.len() != 4 {
            println!("Invalid input. Please enter your move in the format 'e2e4'.");
            continue;
        }

        // extract source and destination info
        let source_col = input.chars().nth(0).unwrap() as usize - 'a' as usize;
        let source_row = 8 - input.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
        let dest_col = input.chars().nth(2).unwrap() as usize - 'a' as usize;
        let dest_row = 8 - input.chars().nth(3).unwrap().to_digit(10).unwrap() as usize;

        println!("{}", source_col);
        println!("{}", source_row);
        println!("{}", dest_col);
        println!("{}", dest_row);
    }
}
