// use alloc::vec::Vec;
use std::collections::HashMap;

use crate::Piece;
use crate::{Move, Pos};


pub struct PlayerPieces {
    pieces: HashMap<Pos, Piece>, 
    #[deprecated(note = "move to main piece holder")]
    modified: Vec<Pos>,
}

impl PlayerPieces {

    pub fn iter(&self) -> impl Iterator<Item = (&Pos, &Piece)> {
        self.pieces.iter()
    }

    pub fn iter_with_move<'a>(&'a self, mov: &'a Move) -> impl Iterator<Item = (&'a Pos, &'a Piece)> + 'a {
        let copy = self.at(&mov.from).expect("Could not get piece to copy for iter_with_move!");
        self.pieces.iter().filter(|(pos, ..)| *pos != &mov.from && *pos != &mov.to).chain(std::iter::once((&mov.to, copy)))
    }

    pub fn at(&self, pos: &Pos) -> Option<&Piece> {
        self.pieces.get(pos)
    }

    pub fn at_mut(&mut self, pos: &Pos) -> Option<&mut Piece> {
        self.pieces.get_mut(pos).inspect(|_| self.modified.push(*pos))
    }

    pub fn take(&mut self, pos: &Pos) -> Option<Piece> {
        self.pieces.remove(pos).inspect(|_| self.modified.push(*pos))
    }

    pub fn move_piece(&mut self, mov: Move) -> Option<Piece> {
        let piece = self.take(&mov.from).expect("Could not get moved piece!");
        self.modified.push(mov.to);
        let taken = self.pieces.insert(mov.to, piece);
        taken
    }

    pub fn drain_changes(&mut self) -> impl Iterator<Item = Pos> + '_ {
        self.modified.drain(..)
    }

    pub fn clear(&mut self) {
        self.pieces.drain().for_each(|(pos, ..)| {
            self.modified.push(pos);
        });
    }
    
    pub fn fill_with(&mut self, iter: impl Iterator<Item = (Pos, Piece)>) {
        self.pieces.extend(iter.inspect(|(pos, ..)| self.modified.push(*pos)));
    }
    
}

impl FromIterator<(Pos, Piece)> for PlayerPieces {
    fn from_iter<T: IntoIterator<Item = (Pos, Piece)>>(iter: T) -> Self {
        Self { pieces: iter.into_iter().collect(), modified: Vec::new() }
    }
}