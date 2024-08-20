mod step;

// use alloc::vec::Vec;

use core::fmt::{Display, Formatter, Result as FmtResult};
use core::slice::Iter;

use king::*;
use step::pawn::*;
use step::*;

use crate::{Board, Move, Pos, PosInt, Side};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoardPiece {
    pub kind: Piece,
    pub side: Side,
}

impl BoardPiece {

    pub const fn symbol(&self) -> char {
        self.kind.symbol(self.side)
    }

    fn with<'a>(step: &'a dyn PieceStep, position: Pos, side: Side, predicate: impl Fn(Pos, PosInt) -> bool + 'a + Copy) -> impl Iterator<Item = Pos> + 'a {
        step.directions().copied().map(move |d| d * side.forward()).flat_map(move |direction| {
            (1..=match step.once() {
                false => (1..).take_while(move |i| predicate(direction, *i)).count() as PosInt + 1,
                true => predicate(direction, 1) as PosInt,
            }).map(move |i| position + (direction * i))
        })
    }
    
    fn step_targets<'a>(board: &'a Board, step: &'a &'static dyn PieceStep, position: Pos, side: Side) -> impl Iterator<Item = Pos> + 'a {
        Self::with(*step, position, side, move |direction, num| {
            let mov = position + (direction * num);
            Self::on(mov) && Self::previous_unoccupied(board, position, direction, num) && step.condition(board, Move { from: position, to: position + (direction * num) }, side)
        })
    }

    fn step_moves<'a>(board: &'a Board, step: &'a &'static dyn PieceStep, position: Pos, side: Side) -> impl Iterator<Item = Pos> + 'a {
        Self::step_targets(board, step, position, side).filter(move |mov| board.pieces.at(mov).filter(|piece| piece.side == side || piece.kind == Piece::King).is_none() && Self::prevents_check(board, &Move { from: position, to: *mov }, side))
    }

    pub fn targets<'a>(&'a self, board: &'a Board, position: Pos) -> impl Iterator<Item = Pos> + 'a {
        self.kind.targets().flat_map(move |step| Self::step_targets(board, step, position, self.side))
    }

    pub fn can_move(&self, board: &Board, mov: Move) -> Option<&dyn PieceStep> {
        self.kind.moves().find(move |step| Self::step_moves(board, step, mov.from, self.side).any(|target| target == mov.to)).map(|s| *s)
    }

    pub fn moves<'a>(&'a self, board: &'a Board, position: Pos) -> impl Iterator<Item = Pos> + 'a {
        self.kind.moves().flat_map(move |step| Self::step_moves(board, step, position, self.side))
    }

    fn on(pos: Pos) -> bool {
        pos.x >= 0 && pos.x < 8 && pos.y >= 0 && pos.y < 8
    }

    fn previous_unoccupied(board: &Board, position: Pos, direction: Pos, num: PosInt) -> bool {
        num <= 1 || !step::occupied(board, position + (direction * (num - 1)))
    }

    fn prevents_check(board: &Board, mov: &Move, side: Side) -> bool {
        let kings = board.pieces.iter_with_move(mov).filter(|(.., piece)| piece.is_king(side)).map(|(pos, ..)| pos).collect::<Vec<_>>();
        board.pieces.iter_with_move(mov).filter(|(.., piece)| piece.side == side.other()).flat_map(|(pos, piece)| piece.targets(board, *pos)).all(|target| !kings.contains(&&target))
    }

    pub fn is_king(&self, side: Side) -> bool {
        self.kind == Piece::King && self.side == side
    }

}

impl Display for BoardPiece {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.kind.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

macro_rules! step {
    ( $($x:expr), * ) => {
        [$(& $x as &dyn PieceStep, )*].iter()
    };
}

impl Piece {

    pub const fn symbol(&self, side: Side) -> char {
        match side {
            Side::White => match self {
                Self::Pawn => '♟',
                Self::Rook => '♜',
                Self::Knight => '♞',
                Self::Bishop => '♝',
                Self::Queen => '♛',
                Self::King => '♚',
            },
            Side::Black => match self {
                Self::Pawn => '♙',
                Self::Rook => '♖',
                Self::Knight => '♘',
                Self::Bishop => '♗',
                Self::Queen => '♕',
                Self::King => '♔',
            },
        }
    }

    pub fn targets(&self) -> Iter<'static, &'static dyn PieceStep> {
        match self {
            Self::Pawn => step!(PawnTake),
            Self::Rook => step!(RookStep),
            Self::Knight => step!(KnightStep),
            Self::Bishop => step!(BishopStep),
            Self::Queen => step!(QueenStep),
            Self::King => step!(KingTarget),
        }
    }

    pub fn moves(&self) -> Iter<'static, &'static dyn PieceStep> {
        match self {
            Self::Pawn => step!(SingleMove, DoubleMove, PawnTake, EnPassant),
            Self::Rook => step!(RookStep),
            Self::Knight => step!(KnightStep),
            Self::Bishop => step!(BishopStep),
            Self::Queen => step!(QueenStep),
            Self::King => step!(KingMove, Castling),
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