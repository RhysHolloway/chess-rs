use core::slice::Iter;

use crate::{Board, Move, Pos, Side};

pub mod pawn;
pub mod king;

pub trait PieceStep {

    fn once(&self) -> bool;

    fn directions(&self) -> Iter<'_, Pos>;

    #[allow(unused_variables)]
    fn condition(&self, board: &Board, mov: Move, side: Side) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn on_move(&self, board: &mut Board, mov: Move, side: Side) {

    }

}

pub fn occupied(board: &Board, mov: Pos) -> bool {
    board.pieces.at(&mov).is_some()
}

pub struct MultiStep<const DIR: bool, const DIAG: bool>;

#[allow(non_upper_case_globals)]
pub const RookStep: MultiStep<true, false> = MultiStep;
#[allow(non_upper_case_globals)]
pub const BishopStep: MultiStep<false, true> = MultiStep;
#[allow(non_upper_case_globals)]
pub const QueenStep: MultiStep<true, true> = MultiStep;


const DIRECTIONS: [Pos; 8] = [
    Pos { x: 0, y: 1 },
    Pos { x: 1, y: 0 },
    Pos { x: 0, y: -1 },
    Pos { x: -1, y: 0 },
    Pos { x: 1, y: 1 },
    Pos { x: 1, y: -1 },
    Pos { x: -1, y: -1 },
    Pos { x: -1, y: 1 }
];

impl<const DIR: bool, const DIAG: bool> PieceStep for MultiStep<DIR, DIAG> {
    
    fn once(&self) -> bool {
        false
    }
    
    fn directions(&self) -> Iter<'static, Pos> {
        DIRECTIONS[if DIR { 0 } else { 4 }..if DIAG { 8 } else { 4 }].iter()
    }
}

pub struct KnightStep;

const KNIGHT_STEPS: [Pos; 8] = [
    Pos { x: 1, y: 2 },
    Pos { x: 2, y: 1 },
    Pos { x: 2, y: -1 },
    Pos { x: 1, y: -2 },
    Pos { x: -1, y: -2 },
    Pos { x: -2, y: -1 },
    Pos { x: -2, y: 1 },
    Pos { x: -1, y: 2 }
];

impl PieceStep for KnightStep {

    
    fn directions(&self) -> Iter<'static, Pos> {
        KNIGHT_STEPS.iter()
    }
    
    fn once(&self) -> bool {
        true
    }
}