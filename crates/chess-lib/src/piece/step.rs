use core::slice::Iter;

use crate::{Board, Move, Side, Pos};

pub mod pawn;
pub mod king;

pub trait PieceStep<S: Side> {

    fn once(&self) -> bool;

    /**
     * Forward is y-positive,
     * Right is x-positive
     */
    fn directions(&self) -> Iter<'_, Pos>;

    #[allow(unused_variables)]
    fn condition(&self, board: &Board<S>, mov: Move, side: &S) -> bool {
        true
    }

    #[allow(unused_variables)]
    fn on_move(&self, board: &mut Board<S>, mov: Move, side: &S) {

    }

}

pub fn occupied<S: Side>(board: &Board<S>, mov: Pos) -> bool {
    board.players.piece_at(&mov).is_some()
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

impl<const DIR: bool, const DIAG: bool, S: Side> PieceStep<S> for MultiStep<DIR, DIAG> {
    
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

impl<S: Side> PieceStep<S> for KnightStep {

    
    fn directions(&self) -> Iter<'static, Pos> {
        KNIGHT_STEPS.iter()
    }
    
    fn once(&self) -> bool {
        true
    }
}