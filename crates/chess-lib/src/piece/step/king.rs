use crate::{Board, Move, Piece, Side, Pos, Rectangle};

use super::{occupied, Iter, MultiStep, PieceStep, QueenStep};

pub struct KingTarget;

impl<S: Side> PieceStep<S> for KingTarget {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        <MultiStep<true, true> as PieceStep<S>>::directions(&QueenStep)
    }
}

pub struct KingMove;

impl KingMove {

    pub fn safe<S: Side + 'static>(board: &Board<S>, position: Pos, side: &S) -> bool {
        board.players.not(side).flat_map(|side| side.pieces().flat_map(|(other, piece)| piece.targets(board, *other, &side.side))).all(|target| position != target)
    }

}

impl<S: Side + 'static> PieceStep<S> for KingMove {
    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        <KingTarget as PieceStep<S>>::directions(&KingTarget)
    }

    fn condition(&self, board: &Board<S>, mov: Move, side: &S) -> bool {
        KingTarget.condition(board, mov, side) && Self::safe(board, mov.to, side)
    }
}

pub struct Castling;

impl<S: Side + 'static> PieceStep<S> for Castling {

    fn once(&self) -> bool {
        true
    }

    fn directions(&self) -> Iter<'static, Pos> {
        [Pos { x: -3, y: 0 }, Pos { x: -2, y: 0 }, Pos { x: 2, y: 0 }].iter()
    }

    fn condition(&self, board: &Board<S>, mov: Move, side: &S) -> bool {
        let king_offset = (mov.to - mov.from).x;
        let direction = king_offset / king_offset.abs();

        let rook = side.origin().enumerate().find(|(i, ..)| i == if direction == 1 { &7 } else { &0 }).map(|(_, pos)| pos).expect("Could not get default rook position!");
        let no_rook_move = || board.players.piece_at(&rook).filter(|(other, piece)| board.history.of(rook).count() == 1 && matches!(piece, Piece::Rook) && *other == side).is_some();
        let no_king_move = || board.history.of(mov.from).count() == 1;
        let not_in_check = || KingMove::safe(board, mov.from, side);
        let safe_between = || (1..=king_offset.abs()).into_iter().map(|i| mov.from + Pos { x: (direction * i), y: 0 }).all(|pos| !occupied(board, pos) && KingMove::safe(board, pos, side));

        // dbg!(rook, no_king_move, no_rook_move, not_in_check, safe_between);
        // println!();

        (no_king_move)() && (no_rook_move)() && (not_in_check)() && (safe_between)()
    }

    fn on_move(&self, board: &mut Board<S>, mov: Move, side: &S) {
        let right = mov.to.x > 3;
        board.players.move_piece(Move { from: side.origin().enumerate().find(|(i, ..)| i == if right { &7 } else { &0 }).expect("Could not find rook!").1, to: mov.to + Pos { x: if right { -1 } else { 1 }, y: 0 } });
    }
}