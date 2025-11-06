mod step;

// use alloc::vec::Vec;

use core::fmt::{Display, Formatter, Result as FmtResult};
use core::slice::Iter;

use king::*;
use step::pawn::*;
use step::*;

use crate::{Board, Move, Side, Pos, PosInt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all = "lowercase"))]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

macro_rules! step {
    ( $id:ty, $($x:expr), * ) => {
        [$(& $x as &dyn PieceStep<$id>, )*].iter()
    };
}

impl Piece {


    fn with<'a, S: Side>(step: &'a dyn PieceStep<S>, position: Pos, side: &'a S, predicate: impl Fn(Pos, PosInt) -> bool + 'a + Copy) -> impl Iterator<Item = Pos> + 'a {
        step.directions().copied().map(move |d| d.rotate(side.forward())).flat_map(move |direction| {
            (1..=match step.once() {
                false => (1..).take_while(move |i| predicate(direction, *i)).count() as PosInt + 1,
                true => predicate(direction, 1) as PosInt,
            }).map(move |i| position + (direction * i))
        })
    }
    
    fn step_targets<'a, S: Side>(board: &'a Board<S>, step: &'a &'static dyn PieceStep<S>, position: Pos, side: &'a S) -> impl Iterator<Item = Pos> + 'a {
        Self::with(*step, position, side, move |direction, num| {
            let mov = position + (direction * num);
            S::on_board(&mov) && Self::previous_unoccupied(board, position, direction, num) && step.condition(board, Move { from: position, to: position + (direction * num) }, side)
        })
    }

    fn step_moves<'a, S: Side>(board: &'a Board<S>, step: &'a &'static dyn PieceStep<S>, position: Pos, side: &'a S) -> impl Iterator<Item = Pos> + 'a {
        Self::step_targets(board, step, position, side).filter(move |mov| board.players.piece_at(mov).filter(|(other, piece)| side == *other || piece == &&Piece::King).is_none() && Self::prevents_check(board, &Move { from: position, to: *mov }, side))
    }

    pub fn targets<'a, S: Side + 'static>(&'a self, board: &'a Board<S>, position: Pos, side: &'a S) -> impl Iterator<Item = Pos> + 'a {
        self.target_steps().flat_map(move |step| Self::step_targets(board, step, position, side))
    }

    pub fn can_move<S: Side + 'static>(&self, board: &Board<S>, mov: Move, side: &S) -> Option<&dyn PieceStep<S>> {
        self.move_steps().find(move |step| Self::step_moves(board, step, mov.from, side).any(|target| target == mov.to)).map(|s| *s)
    }

    pub fn moves<'a, S: Side + 'static>(&'a self, board: &'a Board<S>, position: Pos, side: &'a S) -> impl Iterator<Item = Pos> + 'a {
        self.move_steps().flat_map(move |step| Self::step_moves(board, step, position, side))
    }

    fn previous_unoccupied<S: Side>(board: &Board<S>, position: Pos, direction: Pos, num: PosInt) -> bool {
        num <= 1 || !step::occupied(board, position + (direction * (num - 1)))
    }

    fn prevents_check<S: Side + 'static>(board: &Board<S>, mov: &Move, side: &S) -> bool {
        let kings = board.players.piece_iter_with_move(mov).filter(|(.., other, piece)| matches!(piece, Piece::King) && side == *other).map(|(pos, ..)| pos).collect::<Vec<_>>();
        board.players.piece_iter_with_move(mov).filter(|(_, other, _)| *other != side).flat_map(|(pos, id, piece)| piece.targets(board, *pos, id)).all(|target| !kings.contains(&&target))
    }

    fn target_steps<S: Side>(&self) -> Iter<'static, &'static dyn PieceStep<S>> {
        match self {
            Self::Pawn => step!(S, PawnTake),
            Self::Rook => step!(S, RookStep),
            Self::Knight => step!(S, KnightStep),
            Self::Bishop => step!(S, BishopStep),
            Self::Queen => step!(S, QueenStep),
            Self::King => step!(S, KingTarget),
        }
    }

    fn move_steps<S: Side>(&self) -> Iter<'static, &'static dyn PieceStep<S>> {
        match self {
            Self::Pawn => step!(S, SingleMove, DoubleMove, PawnTake, EnPassant),
            Self::Rook => step!(S, RookStep),
            Self::Knight => step!(S, KnightStep),
            Self::Bishop => step!(S, BishopStep),
            Self::Queen => step!(S, QueenStep),
            Self::King => step!(S, KingMove, Castling),
        }
    }

}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", match self {
            Self::Pawn => "",
            Self::Rook => "R",
            Self::Knight => "N",
            Self::Bishop => "B",
            Self::Queen => "Q",
            Self::King => "K",
        })
    }
}