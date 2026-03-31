use crate::{Direction, Line, Piece, Pos, PosBox, Rectangle, Side};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Chess360 {
    #[default]
    Red,
    Blue,
    Yellow,
    Green,
}

impl Side for Chess360 {
    fn dimensions() -> impl IntoIterator<Item = &'static (impl Rectangle + 'static)> {
        [
            &PosBox { min: Pos { x: 4, y: 4 }, max: Pos { x: 12, y: 12 } }, // center
            &PosBox { min: Pos { x: 4, y: 0 }, max: Pos { x: 12, y: 4 } }, // red
            &PosBox { min: Pos { x: 4, y: 12 }, max: Pos { x: 12, y: 16 } }, // blue
            &PosBox { min: Pos { x: 0, y: 4 }, max: Pos { x: 4, y: 12 } }, // green
            &PosBox { min: Pos { x: 12, y: 4 }, max: Pos { x: 16, y: 12 } }, // yellow
        ]
    }

    fn origin(&self) -> &(impl Rectangle + '_) {
        match self {
            Self::Red => &Line { start: Pos { x: 4, y: 0 }, direction: Direction::XPos, length: 8 },
            Self::Blue => &Line { start: Pos { x: 12, y: 15 }, direction: Direction::XNeg, length: 8 },
            Self::Green => &Line { start: Pos { x: 0, y: 4 }, direction: Direction::YPos, length: 8 },
            Self::Yellow => &Line { start: Pos { x: 15, y: 12 }, direction: Direction::YNeg, length: 8 },
        }
    }

    fn forward(&self) -> Direction {
        match self {
            Self::Red => Direction::YPos,
            Self::Blue => Direction::YNeg, 
            Self::Green => Direction::XPos, 
            Self::Yellow => Direction::XNeg, 
        }
    }

    fn first(&self) -> bool {
        matches!(self, Self::Red)
    }

    fn next(&mut self) {
        *self = match self {
            Self::Red => Self::Blue,
            Self::Blue => Self::Yellow,
            Self::Yellow => Self::Green,
            Self::Green => Self::Red,
        }
    }

    fn pieces(&self) -> impl Iterator<Item = (Pos, Piece)> {
        self.origin().enumerate().flat_map(move |(i, pos)| [match i {
            0 | 7 => (pos, Piece::Rook),
            1 | 6 => (pos, Piece::Knight),
            2 | 5 => (pos, Piece::Bishop),
            3 => (pos, Piece::Queen),
            4 => (pos, Piece::King),
            ..0 | 8.. => unreachable!(),
        }, (pos + self.forward(), Piece::Pawn)])
    }
}

impl std::fmt::Display for Chess360 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}