use crate::{Board, Move, Piece, Side, Pos, Rectangle};

use super::{occupied, Iter, PieceStep};

fn pawn_promotion<S: Side>(board: &mut Board<S>, mov: Move, side: &S) {
    if side.origin().iter().any(|p| p == mov.to) {
        *board.players.piece_at_mut(&mov.to).expect("Could not get pawn to promote!").1 = Piece::Queen;
    }
}

pub struct SingleMove;

impl<S: Side> PieceStep<S> for SingleMove {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: 0, y: 1 }].iter()
    }

    fn condition(&self, board: &Board<S>, mov: Move, _player: &S) -> bool {
        !occupied(board, mov.to)
    }
    
    fn on_move(&self, board: &mut Board<S>, mov: Move, side: &S) {
        pawn_promotion(board, mov, side);
    }
    
    fn once(&self) -> bool {
        true
    }

}

pub struct DoubleMove;

impl<S: Side> PieceStep<S> for DoubleMove {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: 0, y: 2 }].iter()
    }

    fn condition(&self, board: &Board<S>, mov: Move, side: &S) -> bool {
        side.origin().iter().map(|p| p + side.forward()).any(|p| p == mov.from) && !occupied(board, mov.from + side.forward()) && !occupied(board, mov.to)
    }
    
    fn once(&self) -> bool {
        true
    }
}

pub struct PawnTake;


impl<S: Side> PieceStep<S> for PawnTake {
    // const LIMIT: Option<PosInt> = Some(1);

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -1, y: 1 }, Pos { x: 1, y: 1 }].iter()
    }

    fn condition(&self, board: &Board<S>, mov: Move, _player: &S) -> bool {
        occupied(board, mov.to)
    }
    
    fn on_move(&self, board: &mut Board<S>, mov: Move, side: &S) {
        pawn_promotion(board, mov, side);
    }
    
    fn once(&self) -> bool {
        true
    }

}

pub struct EnPassant;

impl<S: Side> PieceStep<S> for EnPassant {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -1, y: 1 }, Pos { x: 1, y: 1 }].iter()
    }

    fn condition(&self, board: &Board<S>, mov: Move, side: &S) -> bool {
        let pos = mov.to + side.forward() * -1i8;
        board.players.piece_at(&pos).filter(|(other, piece)| *other != side && matches!(piece, Piece::Pawn) && board.history.of(pos).count() == 2).is_some()
    }
    
    fn on_move(&self, board: &mut Board<S>, mov: Move, side: &S) {
        board.players.take(&(mov.to + (side.forward() * -1i8)));
    }
    
}