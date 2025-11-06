use std::fmt::Display;
use std::hash::Hash;

use crate::{Direction, Piece, Pos, PosBox, Rectangle};

pub trait Side: Eq + Hash + Display + Default + Clone {

    fn dimensions() -> impl IntoIterator<Item = &'static PosBox>;

    fn on_board(pos: &Pos) -> bool {
        Self::dimensions().into_iter().any(|dim| dim.contains(pos))
    }

    // todo impl rectangle
    fn origin(&self) -> &(impl Rectangle + '_);

    fn forward(&self) -> Direction;

    // fn file(&self, no: PosInt) -> Pos;
    
    // fn offset<M>(&self, offset: PosInt) -> Pos {
    //     self.origin() + (offset * self.forward())
    // }

    fn first(&self) -> bool;

    fn next(&mut self);

    fn pieces(&self) -> impl Iterator<Item = (Pos, Piece)>;

}