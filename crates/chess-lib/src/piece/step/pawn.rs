use crate::{Board, Move, Piece, Pos, Side};

use super::{occupied, Iter, PieceStep};

fn pawn_promotion(board: &mut Board, mov: Move, side: Side) {
    if mov.to.y == side.origin() {
        board.pieces.at_mut(&mov.to).expect("Could not get pawn to promote!").kind = Piece::Queen;
    }
}

pub struct SingleMove;

impl PieceStep for SingleMove {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: 0, y: 1 }].iter()
    }

    fn condition(&self, board: &Board, mov: Move, _side: Side) -> bool {
        !occupied(board, mov.to)
    }
    
    fn on_move(&self, board: &mut Board, mov: Move, side: Side) {
        pawn_promotion(board, mov, side);
    }
    
    fn once(&self) -> bool {
        true
    }

}

pub struct DoubleMove;

impl PieceStep for DoubleMove {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: 0, y: 2 }].iter()
    }

    fn condition(&self, board: &Board, mov: Move, side: Side) -> bool {
        mov.from.y == side.offset(1) && !occupied(board, mov.from + Pos { x: 0, y: side.forward() }) && !occupied(board, mov.to)
    }
    
    fn once(&self) -> bool {
        true
    }
}

pub struct PawnTake;


impl PieceStep for PawnTake {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -1, y: 1 }, Pos { x: 1, y: 1 }].iter()
    }

    fn condition(&self, board: &Board, mov: Move, _side: Side) -> bool {
        occupied(board, mov.to)
    }
    
    fn on_move(&self, board: &mut Board, mov: Move, side: Side) {
        pawn_promotion(board, mov, side);
    }
    
    fn once(&self) -> bool {
        true
    }

}

pub struct EnPassant;

impl PieceStep for EnPassant {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -1, y: 1 }, Pos { x: 1, y: 1 }].iter()
    }

    fn condition(&self, board: &Board, mov: Move, side: Side) -> bool {
        let pos = mov.to + Pos { x: 0, y: -side.forward() };
        board.pieces.at(&pos).filter(|piece| piece.side == side.other() && piece.kind == Piece::Pawn && board.history.of(pos).count() == 2).is_some()
    }
    
    fn on_move(&self, board: &mut Board, mov: Move, side: Side) {
        board.pieces.take(&(mov.to - Pos { x: 0, y: side.forward() }));
    }
    
}