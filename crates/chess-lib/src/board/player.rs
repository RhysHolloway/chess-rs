use std::collections::{BTreeMap, HashMap};
use std::ops::{Deref, Index, IndexMut};
use std::time::Duration;

use crate::{Move, Piece, Pos, Side};

pub enum EndRequest {
    Draw,
    Resign,
}
// #[derive(Default)]
pub struct BoardPlayer {
    // pieces: 

    // /// Time remaining
    // time: Option<Duration>,

    // // flags
    // resign: bool,
    // draw: bool,
    // // pub check: Option<Vec<Move>>,
}

impl BoardPlayer {

    pub fn pieces(&self) -> impl Iterator<Item = (&Pos, &Piece)> {
        self.pieces.iter()
    }

    pub fn fill(&mut self, side: &S) {
        self.pieces = side.pieces().collect();
    }

    pub fn piece_at(&self, pos: &Pos) -> Option<&Piece> {
        self.pieces.get(pos)
    }

    pub fn take(&mut self, pos: &Pos) -> Option<Piece> {
        self.pieces.remove(pos)
    }

    pub fn pieces_with_move<'a>(&'a self, mov: &'a Move) -> impl Iterator<Item = (&'a Pos, &'a Piece)> + 'a {
        let copy = self.pieces.get(&mov.from).expect("Could not get piece to copy for iter_with_move!");
        self.pieces.iter().filter(|(pos, ..)| *pos != &mov.from && *pos != &mov.to).chain(std::iter::once((&mov.to, copy)))
    }

    pub fn piece_history(&self, pos: &Pos) -> impl Iterator<Item = Pos> {
        let mut pos = *pos;
        self.pieces.contains_key(&pos).then(|| std::iter::once(pos).chain(self.moves.iter().rev().filter_map(move |mov| {
            (mov.mov.to == pos).then(|| {
                pos = mov.mov.from;
                mov.mov.from
            })
        }).rev())).into_iter().flatten()
    }
    
    // pub fn at_mut(&mut self, pos: &Pos) -> Option<&mut Piece> {
    //     self.pieces.get_mut(pos).inspect(|_| self.modified.push(*pos))
    // }

    // pub fn take(&mut self, pos: &Pos) -> Option<Piece> {
    //     self.pieces.remove(pos).inspect(|_| self.modified.push(*pos))
    // }
}

impl<S: Side> From<&S> for BoardPlayer<S> {
    fn from(side: &S) -> Self {
        Self {
            pieces: side.pieces().collect(),
            time: Default::default(),
            moves: Default::default(),
            resign: false,
            draw: false,
        }
    }
}

impl<S: Side> Default for BoardPlayers<S> {
    fn default() -> Self {
        let mut side = S::default();
        let mut players = Vec::new();
        loop {
            players.push(BoardPlayer::from(&side));
            side.next();
            if side.first() {
                break;
            }
        }
        Self { players }
    }
}
