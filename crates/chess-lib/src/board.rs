// use alloc::vec::Vec;

mod pieces;
mod history;

use crate::{Move, Side};

pub use pieces::Pieces;

#[derive(Default)]
pub struct Board {
    pub pieces: Pieces,
    pub state: State,
    pub history: history::BoardHistory,
    // moves: Vec<Move>,
}

impl Board {
    
    pub fn move_piece(&mut self, mov: Move) -> Result<(), MoveError> {
        
        if self.state.check.as_ref().map(Vec::is_empty).unwrap_or(false) {
            return Err(MoveError::Checkmate)
        }

        if let Some(piece) = self.pieces.at(&mov.from).copied() {
            
            if piece.side != self.state.turn.side {
                return Err(MoveError::WrongSide)
            }

            if let Some(step) = piece.can_move(self, mov) {
                let taken = self.pieces.move_piece(mov);
                step.on_move(self, mov, piece.side);
                

                // let taken = self.pieces.0.iter().position(|p| p.position == mov.to).map(|i| self.pieces.0.remove(i));
                // self.pieces.0.iter_mut().find(|p| p.position == mov.from).unwrap().position = mov.to;
                // self.state.check = self.pieces.check(self.state.turn.side.other(), None);

                self.history.add(mov, taken);

                self.state.check = self.check(piece.side.other());

                self.state.turn.increment();

                // let piece = self.pieces.at(&mov.to).unwrap();
                // Ok(piece)
                Ok(())
            } else {
                Err(MoveError::InvalidMove)
            }
        } else {
            Err(MoveError::NoPiece)
        }
    }

    pub fn check(&self, side: Side) -> Option<Vec<Move>> {
        let kings = self.pieces.iter().filter(|(.., piece)| piece.is_king(side)).map(|(pos, ..)| pos).collect::<Vec<_>>();
        self.pieces.of(side.other()).flat_map(|(pos, piece)| piece.targets(self, *pos)).any(|target| kings.contains(&&target)).then(|| {
            self.pieces.of(side).flat_map(|(pos, piece)| piece.moves(self, *pos).map(|to| Move::new(*pos, to))).collect()
        })
    }

    pub fn reset(&mut self) {
        self.state.reset();
        self.pieces.reset();
        self.history.reset();
    }

}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Turn {
    pub side: Side,
    pub no: usize,
}

impl Turn {
    pub fn increment(&mut self) {
        self.side = self.side.other();
        if self.side == Side::White {
            self.no += 1;
        }
    }
}

#[derive(Default)]
pub struct State {
    pub turn: Turn,
    check: Option<Vec<Move>>,
}

impl State {
    pub fn check(&self) -> Option<&Vec<Move>> {
        self.check.as_ref()
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MoveError {
    WrongSide,
    NoPiece,
    InvalidMove,
    Check,
    Checkmate,
}