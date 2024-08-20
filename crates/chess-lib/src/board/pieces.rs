// use alloc::vec::Vec;
use std::collections::HashMap;

use crate::piece::{BoardPiece, Piece};
use crate::{Move, Pos, PosInt, Side};


pub struct Pieces(HashMap<Pos, BoardPiece>, Vec<PieceUpdate>);

pub enum PieceUpdate {
    Update(Pos, Option<BoardPiece>),
    Modify(Pos, Piece),
}

impl Pieces {

    pub fn iter(&self) -> impl Iterator<Item = (&Pos, &BoardPiece)> {
        self.0.iter()
    }

    pub fn iter_with_move<'a>(&'a self, mov: &'a Move) -> impl Iterator<Item = (&'a Pos, &'a BoardPiece)> + 'a {
        let copy = self.at(&mov.from).expect("Could not get piece to copy for iter_with_move!");
        self.0.iter().filter(|(pos, ..)| *pos != &mov.from && *pos != &mov.to).chain(std::iter::once((&mov.to, copy)))
    }
    
    pub fn of(&self, side: Side) -> impl Iterator<Item=(&Pos, &BoardPiece)> {
        self.0.iter().filter(move |(.., piece)| piece.side == side)
    }

    pub fn at(&self, pos: &Pos) -> Option<&BoardPiece> {
        self.0.get(pos)
    }

    pub fn at_mut(&mut self, pos: &Pos) -> Option<&mut BoardPiece> {
        self.0.get_mut(pos)
    }

    pub fn take(&mut self, pos: &Pos) -> Option<BoardPiece> {
        self.0.remove(pos)
    }

    pub fn move_piece(&mut self, mov: Move) -> Option<BoardPiece> {
        let piece = self.take(&mov.from).expect("Could not get moved piece!");
        let taken = self.0.insert(mov.to, piece);
        taken
    }

    pub fn events(&mut self) -> impl Iterator<Item = PieceUpdate> + '_ {
        self.1.drain(..)
    }

    pub fn clear(&mut self) {
        self.0.drain().for_each(|(pos, ..)| {
            self.1.push(PieceUpdate::Update(pos, None));
        });
    }
    
    pub fn fill(&mut self) {
        self.0.extend(Self::default_board().inspect(|(pos, piece)| self.1.push(PieceUpdate::Update(*pos, Some(*piece)))));
    }

    pub fn reset(&mut self) {
        self.clear();
        self.fill();
    }

    fn default_board() -> impl Iterator<Item = (Pos, BoardPiece)> {

        fn mirror(x: PosInt, side: Side, kind: Piece) -> impl IntoIterator<Item = (Pos, BoardPiece)> {
            [(Pos { x, y: side.origin() }, BoardPiece {
                kind,
                side,
            }), (Pos { x: 7 - x, y: side.origin() }, BoardPiece {
                kind,
                side,
            })]
        }

        fn side(side: Side) -> impl Iterator<Item = (Pos, BoardPiece)> {
                (0..8).into_iter().map(move |x| (Pos { x, y: side.offset(1) }, BoardPiece {
                    kind: Piece::Pawn,
                    side,
                }))
                .chain(mirror(0, side, Piece::Rook))
                .chain(mirror(1, side, Piece::Knight))
                .chain(mirror(2, side, Piece::Bishop))
                .chain(std::iter::once((Pos { x: 3, y: side.origin() }, BoardPiece {
                    kind: Piece::Queen,
                    side,
                })))
                .chain(std::iter::once((Pos { x: 4, y: side.origin() }, BoardPiece {
                    kind: Piece::King,
                    side,
                })))
        }
        
        side(Side::White).chain(side(Side::Black))
    }
    
    
}

impl Default for Pieces {
    fn default() -> Self {
        Self(Self::default_board().collect(), Vec::new())
    }
}