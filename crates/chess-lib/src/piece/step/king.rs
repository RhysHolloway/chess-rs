use crate::{Board, Move, Piece, Pos, Side};

use super::{occupied, Iter, PieceStep, QueenStep};

pub struct KingTarget;

impl PieceStep for KingTarget {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        QueenStep.directions()
    }
}

pub struct KingMove;

impl KingMove {

    pub fn safe(board: &Board, position: Pos, side: Side) -> bool {
        board.pieces.of(side.other()).flat_map(|(other, piece)| piece.targets(board, *other)).all(|target| position != target)
    }

}

impl PieceStep for KingMove {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        KingTarget.directions()
    }

    fn condition(&self, board: &Board, mov: Move, side: Side) -> bool {
        KingTarget.condition(board, mov, side) && Self::safe(board, mov.to, side)
    }
}

pub struct Castling;

impl PieceStep for Castling {

    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -3, y: 0 }, Pos { x: -2, y: 0 }, Pos { x: 2, y: 0 }].iter()
    }

    fn condition(&self, board: &Board, mov: Move, side: Side) -> bool {
        let king_offset = (mov.to - mov.from).x;
        let direction = king_offset / king_offset.abs();

        let rook = Pos { x: if direction == 1 { 7 } else { 0 },  y: side.origin() };
        let no_rook_move = || board.pieces.at(&rook).filter(|piece| board.history.of(rook).count() == 1 && piece.kind == Piece::Rook).is_some();
        let no_king_move = || board.history.of(mov.from).count() == 1;
        let not_in_check = || KingMove::safe(board, mov.from, side);
        let safe_between = || (1..=king_offset.abs()).into_iter().map(|i| mov.from + Pos { x: (direction * i), y: 0 }).all(|pos| !occupied(board, pos) && KingMove::safe(board, pos, side));

        // dbg!(rook, no_king_move, no_rook_move, not_in_check, safe_between);
        // println!();

        (no_king_move)() && (no_rook_move)() && (not_in_check)() && (safe_between)()
    }

    fn on_move(&self, board: &mut Board, mov: Move, side: Side) {
        let right = mov.to.x > 3;
        board.pieces.move_piece(Move { from: Pos { x: if right { 7 } else { 0 }, y: side.origin()}, to: mov.to + Pos { x: if right { -1 } else { 1 }, y: 0 } });
    }
}