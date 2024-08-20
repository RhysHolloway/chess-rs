mod pos;

pub use pos::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    #[default]
    White, 
    Black
}

impl Side {
    pub fn other(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
    
    pub fn origin(&self) -> PosInt {
        match self {
            Self::White => 0,
            Self::Black => 7,
        }
    }

    pub fn forward(&self) -> PosInt {
        match self {
            Self::White => 1,
            Self::Black => -1,
        }
    }

    pub fn offset(&self, offset: PosInt) -> PosInt {
        self.origin() + (offset * self.forward())
    }

    pub fn sides() -> [Self; 2] {
        [Self::White, Self::Black]
    }
}