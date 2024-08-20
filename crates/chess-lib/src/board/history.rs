// use alloc::vec::Vec;

use crate::{BoardPiece, Move, Pos, Side};


#[derive(Default)]
pub struct BoardHistory {
    moves: Vec<PreviousMove>,
}

pub struct PreviousMove {
    pub mov: Move,
    pub taken: Option<BoardPiece>,
}

impl BoardHistory {
    pub fn add(&mut self, mov: Move, taken: Option<BoardPiece>) {
        self.moves.push(PreviousMove { mov, taken });
    }

    // pub fn undo(&mut self) -> Option<PreviousMove> {
    //     self.moves.pop().map(f)
    // }

    pub fn taken(&self, side: Side) -> impl Iterator<Item = &BoardPiece> {
        self.moves.iter().filter_map(move |piece| piece.taken.as_ref().filter(|piece| piece.side == side))
    }

    pub fn of(&self, pos: Pos) -> impl Iterator<Item = Pos> + '_ {
        use core::sync::atomic::{AtomicI8, Ordering::Relaxed};
        let x = AtomicI8::new(pos.x);
        let y = AtomicI8::new(pos.y);
        self.moves.iter().rev().filter_map(move |prev| {
            (prev.mov.to.x == x.load(Relaxed) && prev.mov.to.y == y.load(Relaxed)).then(|| {
                x.store(prev.mov.from.x, Relaxed);
                y.store(prev.mov.from.y, Relaxed);
                prev.mov.from
            })
        }).rev().chain(core::iter::once(pos))
    }

    // pub fn of(&self, piece: &BoardPiece) -> impl Iterator<Item = &Move> {
        // self.moves.iter().rev().fold(piece.position, |current, mov| )
    // }
}
