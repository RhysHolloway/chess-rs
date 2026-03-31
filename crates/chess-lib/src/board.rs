// use alloc::vec::Vec;

// mod pieces;
// mod history;
// pub mod player;

pub mod default;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::fmt::Display;

use crate::{Move, Pos, Direction, Rectangle};


pub trait Piece<T: BoardType> where Self: Clone + Copy + Display + Eq + Hash {

    fn moves(&self, board: &Board<T>, side: &T::Side, pos: &Pos) -> impl Iterator<Item = (Pos, Option<fn(&mut Board<T>, &T::Side, &Move)>)>;
    
    fn targets(&self, board: &Board<T>, side: &T::Side, pos: &Pos) -> HashSet<Pos>;
    
    fn safe_at(&self, board: &Board<T>, side: &T::Side, positions: &[Pos]) -> HashSet<Pos> {
        positions.iter().collect()
    }

}

pub trait Side: Eq + Hash + Display + Default + Clone + Copy {

    fn origin(&self) -> &(impl Rectangle + '_);

    fn forward(&self) -> Direction;

    fn all() -> impl IntoIterator<Item = Self>;

}

pub trait BoardType {

    type Side: Side;
    type Piece: Piece<Self>;

    fn dimensions() -> impl IntoIterator<Item = &'static (impl Rectangle + 'static)>;
    
    fn on_board(pos: &Pos) -> bool {
        Self::dimensions().into_iter().any(|dim| dim.contains(pos))
    }

    fn default() -> HashMap<Pos, PlayerPiece<Self::Side>>;

    fn from_str(input: &str) -> Result<HashMap<Pos, PlayerPiece<Self>>, ()>;

}

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Board<T: BoardType> {
    pieces: HashMap<Pos, PlayerPiece<T>>,
    history: Vec<PreviousMove<T>>,
    players: Vec<T::Side>,
    turn: usize,
    move_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PreviousMove<T: BoardType> {
    pub mov: Move,
    pub taken: Option<PlayerPiece<T>>,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PlayerPiece<T: BoardType> {
    pub side: T::Side,
    pub piece: T::Piece,
}

impl<T: BoardType> PlayerPiece<T> {
    
    pub fn moves(&self, board: &Board<T>, pos: &Pos) -> impl Iterator<Item = (Pos, Option<fn(&mut Board<T>, &T::Side, &Move)>)> {
        self.piece.moves(board, &self.side, pos)
    }

    pub fn targets(&self, board: &Board<T>, pos: &Pos) -> impl Iterator<Item = Pos> {
        self.piece.targets(board, &self.side, pos)
    }

    pub fn safe_at(&self, board: &Board<T>, pos: impl IntoIterator<Item = Pos>) -> impl Iterator<Item = Pos> {
        self.piece.safe_at(board, &self.side, pos)
    }
    
}

pub struct MoveResult<T: BoardType> {
    pub mov: PreviousMove<T>,
    pub checks: HashMap<T::Side, Check>,
}

pub struct Check {
    mate: bool,
}

impl<T: BoardType> Board<T> {

    pub fn move_piece(&mut self, mov: Move) -> Result<MoveResult<T>, MoveError> {

        let piece = self.pieces.get(&mov.from).ok_or(MoveError::NoPiece)?;
            
        if piece.side != self.turn.side {
            return Err(MoveError::WrongSide);
        }

        if !self.should_move(&piece.side) {
            return Err(MoveError::GameOver);
        }

        let step = piece.can_move(self, mov, &side).ok_or(MoveError::InvalidMove)?;
        let taken = self.players.move_piece(mov);
        step.on_move(self, mov, &side);
        self.history.add(mov, taken);
        // self.players.get_mut(&self.turn.side).check = self.check(&self.turn.side);
        self.turn.increment();
        Ok(MoveResult {
            mov,
            taken,
            checks: HashMap::new(),
        })
    }

    pub fn request_draw(&mut self, side: &S) {
        self.players.get_mut(side).draw = true;
    }

    pub fn resign(&mut self, side: &S) {
        self.players.get_mut(side).resign = true;
    }

    pub fn check(&self, side: &S) -> Option<Vec<Move>> {
        let kings = self.players.piece_iter().filter(|(.., other, piece)| matches!(piece, Piece::King) && *other == side).map(|(pos, ..)| pos).collect::<Vec<_>>();
        self.players.not(side).flat_map(|player| player.pieces().flat_map(|(pos, piece)| piece.targets(self, *pos, &player.side))).any(|target| kings.contains(&&target)).then(|| {
            self.players.get(side).pieces().flat_map(|(pos, piece)| piece.moves(self, *pos, side).map(|to| Move::new(*pos, to))).collect()
        })
    }

    pub fn reset(&mut self) {
        self.turn = Default::default();
        self.pieces = S::sides().flat_map(|side| side.pieces().map(move |(pos, piece)| (pos, PlayerPiece { side, piece }))).collect();
        self.history.clear();
    }

    // /// Check if someone has won the current game
    // pub fn winner(&self) -> Option<Option<DefaultSides>> {
    //     if self.players.iter().all(|p| p.draw) {
    //         Some(None)
    //     // } else if self.players.iter().filter(|p| !p.resign && !p.check.map_or(|m| m.is_empty(), false)) || self.players.iter().any(|p| p.check.is_some()) {
    //     //     Some(Some(self.turn.side.other()))
    //     } else {
    //         None
    //     }
    // }

    pub fn not<'a>(&'a self, side: &'a T::Side) -> impl Iterator<Item = &'a Pos> + 'a {
        self.pieces
        .keys()
            .filter(move |other| *other != side)
    }

    pub fn piece_iter(&self) -> impl Iterator<Item = (&Pos, &PlayerPiece<T>)> {
        self.players.iter().flat_map(|(side, player)| {
            player
                .pieces
                .iter()
                .map(move |(pos, piece)| (pos, side, piece))
        })
    }

    pub fn history_of(
        &self,
        pos: &Pos,
    ) -> impl DoubleEndedIterator<Item = &PreviousMove<T>> {
        self.pieces.get(&pos).into_iter().map(move |player| {
            let mut pos = *pos;
            return self.history.iter().rev().filter(move |mov| {
                if mov.mov.to == pos {
                    pos = mov.mov.from;
                    true
                } else {
                    false
                }
            });
        })
    }

    fn piece_iter_with_move<'a>(
        &'a self,
        mov: &'a Move,
    ) -> impl Iterator<Item = (&'a Pos, &'a PlayerPiece<T>)> + 'a {
        let (id, copy) = self
            .piece_at(&mov.from)
            .expect("Could not get piece to copy for iter_with_move!");
        self.piece_iter()
            .filter(|(pos, ..)| *pos != &mov.from && *pos != &mov.to)
            .chain(std::iter::once((&mov.to, id, copy)))
    }

    pub fn take(&mut self, pos: &Pos) -> Option<PlayerPiece<T>> {
        self.players
            .iter_mut()
            .find_map(|player| player.take(pos))
    }

    pub fn move_piece(&mut self, mov: Move) -> Option<(S, Piece)> {

        let side = self.players.iter().position(|player| player.piece_at(&mov.from).is_some())?;
        let other = self.players.iter().position(|player| player.piece_at(&mov.to).is_some());


        if let Some(other) = other.filter(|other| other != side) {
            let ps = self.players.get_disjoint_mut([side, other]).unwrap();
            let side = ps[0];
            let other = ps[1];
            return side.move_piece(mov, Some(other));
        } else {
            return self.players[side].move_piece(mov, None);
        }
    }

    pub fn reset(&mut self) {
        self.players.iter_mut().for_each(BoardPlayer::reset);
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MoveError {
    WrongSide,
    NoPiece,
    InvalidMove,
    Check,
    GameOver,
}