// use alloc::vec::Vec;

mod pieces;
mod history;
mod player;

pub mod boards;

use std::hash::Hash;

use crate::{Move, Piece, Side};


#[derive(Default)]
pub struct Board<S: Side = boards::DefaultSides> {
    pub players: player::BoardPlayers<S>,
    pub turn: Turn<S>,
    #[deprecated(note = "move history into players")]
    pub history: history::BoardHistory<S>,
    // moves: Vec<Move>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Turn<S: Side> {
    #[cfg_attr(feature = "serde", serde(rename = "move"))]
    pub mov: usize,
    pub side: S,
}

impl<P: Side> Turn<P> {
    pub fn increment(&mut self) {
        self.side.next();
        if self.side.first() {
            self.mov += 1;
        }
    }
}


impl<S: Side + 'static> Board<S> {

    pub fn move_piece(&mut self, mov: Move) -> Result<(), MoveError> {

        let (side, piece) = self.players.piece_at(&mov.from).map(|(a, b)| (a.clone(), *b)).ok_or(MoveError::NoPiece)?;
            
        if side != self.turn.side {
            return Err(MoveError::WrongSide)
        }

        if !self.players.get(&side).should_move() {
            return Err(MoveError::GameOver);
        }

        let step = piece.can_move(self, mov, &side).ok_or(MoveError::InvalidMove)?;
        let taken = self.players.move_piece(mov);
        step.on_move(self, mov, &side);
        self.history.add(mov, taken);
        // self.players.get_mut(&self.turn.side).check = self.check(&self.turn.side);
        self.turn.increment();
        Ok(())
    }

    // pub fn 

    pub fn request_draw(&mut self, side: &S) {
        self.players.get_mut(side).draw = true;
    }

    pub fn resign(&mut self, side: &S) {
        self.players.get_mut(side).resign = true;
    }

    pub fn check(&self, side: &S) -> Option<Vec<Move>> {
        let kings = self.players.piece_iter().filter(|(.., other, piece)| matches!(piece, Piece::King) && *other == side).map(|(pos, ..)| pos).collect::<Vec<_>>();
        self.players.not(side).flat_map(|player| player.pieces().flat_map(|(pos, piece)| piece.targets(self, *pos, &player.side))).any(|target| kings.contains(&&target)).then(|| {
            self.players.get(side).pieces().flat_map(|(pos, piece)| piece.moves(self, *pos, side).map(|to| Move::new(*pos, to))).collect()
        })
    }

    pub fn reset(&mut self) {
        self.turn = Default::default();
        self.players.reset();
        self.history.reset();
    }

    // /// Check if someone has won the current game
    // pub fn winner(&self) -> Option<Option<DefaultSides>> {
    //     if self.players.iter().all(|p| p.draw) {
    //         Some(None)
    //     // } else if self.players.iter().filter(|p| !p.resign && !p.check.map_or(|m| m.is_empty(), false)) || self.players.iter().any(|p| p.check.is_some()) {
    //     //     Some(Some(self.turn.side.other()))
    //     } else {
    //         None
    //     }
    // }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MoveError {
    WrongSide,
    NoPiece,
    InvalidMove,
    Check,
    GameOver,
}