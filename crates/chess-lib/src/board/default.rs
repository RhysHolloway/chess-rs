use std::collections::{HashMap, HashSet};

use crate::{Board, BoardType, Direction, Line, Move, Piece, PlayerPiece, Pos, PosBox, Rectangle, Side, pos};

pub struct DefaultBoard;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all = "lowercase"))]
pub enum DefaultSides {
    #[default]
    White, 
    Black
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(rename_all = "lowercase"))]
pub enum DefaultPieces {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

impl BoardType for DefaultBoard {

    type Side = DefaultSides;
    type Piece = DefaultPieces;

    fn dimensions() -> impl IntoIterator<Item = &'static (impl Rectangle + 'static)> {
        [&PosBox { min: Pos { x: 0, y: 0 }, max: Pos { x: 8, y: 8 } }]
    }
    
    fn default(&self) -> HashMap<Pos, PlayerPiece<Self>> {
        self.origin().iter().flat_map(move |pos| [match pos.x {
            0 | 7 => (pos, Piece::Rook),
            1 | 6 => (pos, Piece::Knight),
            2 | 5 => (pos, Piece::Bishop),
            3 => (pos, Piece::Queen),
            4 => (pos, Piece::King),
            ..0 | 8.. => unreachable!(),
        }, (pos + self.forward(), Piece::Pawn)]).collect()
    }

}

impl Side for DefaultSides {
    fn origin(&self) -> &(impl Rectangle + '_) {
        match self {
            Self::White => &Line { start: Pos { x: 0, y: 0 }, direction: Direction::XPos, length: 8 },
            Self::Black => &Line { start: Pos { x: 0, y: 7 }, direction: Direction::XPos, length: 8 },
        }
    }

    fn forward(&self) -> Direction {
        match self {
            Self::White => Direction::YPos,
            Self::Black => Direction::YNeg,
        }
    }

    fn all(&self) -> impl IntoIterator<Item = Self> {
        [Self::White, Self::Black]
    }
}

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

impl<T: BoardType> Piece<T> for DefaultPieces {
    fn moves(&self, board: &Board<T>, side: &T::Side, pos: &Pos) -> impl Iterator<Item = (Pos, Option<fn(&mut Board<T>, &T::Side, &Move)>)> {
        match self {
            DefaultPieces::Pawn => PawnMove.moves(board, side, *pos),
            other => {
                let default = self.targets(board, side, pos).filter(move |pos| board.pieces.get(pos).map(|piece| &piece.side != side).unwrap_or(true)).map(|mov| (mov, None));
                match other {
                    DefaultPieces::Rook => default.chain(Castling::<{Self::King}, false>.moves(board, side, pos)),
                    DefaultPieces::King => default.chain(Castling::<{Self::Rook}, true>.moves(board, side, pos)),
                    _ => default.chain(std::iter::empty()),
                }
            },
        }
    }

    fn targets(&self, board: &Board<T>, side: &T::Side, pos: &Pos) -> impl Iterator<Item = Pos> {
        let iter = match self {
            DefaultPieces::Pawn => todo!(),
            DefaultPieces::Rook => todo!(),
            DefaultPieces::Knight => KNIGHT_STEPS.into_iter().map(|offset| *pos + offset),
            DefaultPieces::Bishop => todo!(),
            DefaultPieces::Queen => todo!(),
            DefaultPieces::King => Pos::directions().into_iter().chain(Pos::diagonals()).map(|direction| *pos + direction)
        };
        iter.filter(T::on_board)
    }

    fn safe_at(&self, board: &Board<T>, side: &T::Side, positions: &[Pos]) -> impl Iterator<Item = Pos> {
        let set = if matches!(self, DefaultPieces::King) {
            board.pieces.iter().flat_map(|(p, piece)| (&piece.side != side).then(|| piece.targets(board, p))).flatten().collect::<HashSet<Pos>>()
        } else {
            HashSet::new()
        };
        return positions.iter().filter(move |pos| !set.contains(pos));
    }
}

impl std::fmt::Display for DefaultSides {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self, f)
    }
}

impl std::fmt::Display for DefaultPieces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

pub struct MultiStep<const DIR: bool, const DIAG: bool>;

#[allow(non_upper_case_globals)]
pub const RookStep: MultiStep<true, false> = MultiStep;
#[allow(non_upper_case_globals)]
pub const BishopStep: MultiStep<false, true> = MultiStep;
#[allow(non_upper_case_globals)]
pub const QueenStep: MultiStep<true, true> = MultiStep;

impl<S: Side, const DIR: bool, const DIAG: bool> MoveType<S> for MultiStep<DIR, DIAG> {
    
    fn moves(&self, board: &Board<S>, side: S, pos: Pos) ->  impl Iterator<Item = (Pos, Option<fn(&mut Board<S>, S, Move)>)> {
        DIRECTIONS[if DIR { 0 } else { 4 }..if DIAG { 8 } else { 4 }].into_iter().flat_map(|offset| {
            let mut i = 0;
            let mut other;
            while let mov = pos + offset * (i + 1) && S::on_board(mov) {
                if let Some((s, ..)) = board.players.piece_at(&mov) {
                    other = s;
                    break;
                }
                i += 1;
            }
            (1..i + (&side != other) as i8).map(move |j| (pos + offset * j, None))
        })
    }
}

pub struct Castling<T: BoardType> { other: T::Piece, dominant: bool }

impl<T: BoardType> Castling<T> {

    fn moves(&self, board: &Board<T>, side: &T::Side, pos: Pos) -> impl IntoIterator<Item = (Pos, Option<fn(&mut Board<T>, Move, &T::Side)>)> {
        if board.history_of(&pos).count() != 1 {
            // return [].into_iter().map(|a| a);
            // return HashMap::new().into_iter();
        }

        let piece = board.pieces.get(&pos).unwrap();
        
        board.pieces.iter().filter(|(other_pos, other_piece)| other_piece.piece == self.other && board.history_of(other_pos).count() == 1).filter(|(opos, opiece)| {

            Rectangle::new(*pos, *opos).into_iter().all(|between| {
                board.players.piece_at(&between).is_none() && piece.piece.safe_at(board, side, between) && opiece.safe_at(board, side, between)
            })

        }).map(|(opos, _)| {
            let opos = *opos;
            let offset = (pos - opos).normalize() * if self.dominant { 1 } else { -1 };
            let new_pos = match self.dominant { true => pos + offset * 2, false => opos + offset * 2 };
            (new_pos, Some(|board: &mut Board<T>, mov: Move, side: &T::Side| {
                board.players.get_mut(side).move_piece(Move::new(opos, new_pos - offset)).expect("Failed to move piece during castling!");
            }))
        })

    }
}


fn pawn_promotion<T: BoardType>(board: &mut Board<T>, mov: Move, side: &T::Side) {
    if side.origin().iter().any(|p| p == mov.to) {
        *board.players.piece_at_mut(&mov.to).expect("Could not get pawn to promote!").1 = Piece::Queen;
    }
}

pub struct PawnMove;

impl PawnMove {

    fn moves<T: BoardType>(&self, board: &Board<T>, side: &T::Side, pos: &Pos) -> impl Iterator<Item = (Pos, Option<fn(&mut Board<T>, &T::Side, Move)>)> {    
        let forward = pos + side.forward();
        let single = board.pieces.get(&forward).is_none().then_some(forward);
        let mut history = board.history_of(&pos).take(2);
        let origin = history.next().unwrap();
        let first = history.next();
        let forward = forward + side.forward();
        let double = (board.players.piece_at(&forward).is_none() && first.is_none()).then_some(forward);

        let take = [-1, 1].into_iter().map(|offset| pos + Pos { x: offset, y: 1 }.rotate(side.forward())).filter(|mov| board.players.piece_at(mov).filter(|(other, piece)| other != &side && matches!(piece, Piece::Pawn)).is_some());

        let en_passant = first.iter().flat_map(|first| {
                (first - origin != side.forward() * 2).then(|| [-1, 1].into_iter().map(|offset| pos + Pos { x: offset, y: 0 }.rotate(side.forward()))
                    .filter(|pos| board.players.piece_at(&first)
                        .filter(|(other, piece)| other != side && matches!(piece, Piece::Pawn)).is_some()))
        });
        [single, double].into_iter().flatten().chain(en_passant)
    }

}