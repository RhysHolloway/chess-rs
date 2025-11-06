use chess_lib::boards::{Chess360, DefaultSides};
use chess_lib::Piece;

pub trait SideTui {

    fn print_symbol(&self, piece: &Piece);

}

impl SideTui for DefaultSides {
    
    fn print_symbol(&self, piece: &Piece) {
        print!("{}", match self {
            Self::White => match piece {
                Piece::Pawn => '♟',
                Piece::Rook => '♜',
                Piece::Knight => '♞',
                Piece::Bishop => '♝',
                Piece::Queen => '♛',
                Piece::King => '♚',
            },
            Self::Black => match piece {
                Piece::Pawn => '♙',
                Piece::Rook => '♖',
                Piece::Knight => '♘',
                Piece::Bishop => '♗',
                Piece::Queen => '♕',
                Piece::King => '♔',
            },
        });
    }
}

impl SideTui for Chess360 {
    fn print_symbol(&self, piece: &Piece) {
        print!("\x1b[3{}m", match self {
            Chess360::Red => '1',
            Chess360::Blue => '4',
            Chess360::Yellow => '3',
            Chess360::Green => '2',
        });
        DefaultSides::White.print_symbol(piece);
        print!("\x1b[0m");
    }
}