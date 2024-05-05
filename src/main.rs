fn main() {
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
        pub fn init(&mut self) {}
    }
    Board::new();
}
