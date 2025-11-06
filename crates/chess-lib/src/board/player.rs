use std::ops::{Deref, Index, IndexMut};
use std::time::Duration;

use crate::{Move, Piece, Side, Pos};

use super::pieces::PlayerPieces;

pub enum EndRequest {
    Draw,
    Resign,
}

pub struct BoardPlayers<S: Side> {
    players: Vec<BoardPlayer<S>>,
}

impl<S: Side> BoardPlayers<S> {

    fn getter<P: Deref<Target = BoardPlayer<S>>>(mut iter: impl Iterator<Item = P>, side: &S) -> P {
        iter.find(|p| &p.side == side).unwrap_or_else(|| panic!("Could not find player {side}!"))
    }

    pub fn get(&self, side: &S) -> &BoardPlayer<S> {
        Self::getter(self.players.iter(), side)
    }

    pub fn get_mut(&mut self, side: &S) -> &mut BoardPlayer<S> {
        Self::getter(self.players.iter_mut(), side)
    }

    pub fn iter(&self) -> impl Iterator<Item = &BoardPlayer<S>> {
        self.players.iter()
    }

    pub fn not<'a>(&'a self, side: &'a S) -> impl Iterator<Item = &'a BoardPlayer<S>> + 'a {
        self.players.iter().filter(move |player| &player.side != side)
    }

    pub fn piece_at(&self, pos: &Pos) -> Option<(&S, &Piece)> {
        self.players.iter().find_map(|player| player.pieces.at(pos).map(|piece| (&player.side, piece)))
    }
    
    pub fn piece_at_mut(&mut self, pos: &Pos) -> Option<(&S, &mut Piece)> {
        self.players.iter_mut().find_map(|player| player.pieces.at_mut(pos).map(|piece| (&player.side, piece)))
    }

    pub fn piece_iter(&self) -> impl Iterator<Item = (&Pos, &S, &Piece)> {
        self.players.iter().flat_map(|player| player.pieces.iter().map(|(pos, piece)| (pos, &player.side, piece)))
    }

    pub fn piece_iter_with_move<'a>(&'a self, mov: &'a Move) -> impl Iterator<Item = (&'a Pos, &'a S, &'a Piece)> + 'a {
        let (id, copy) = self.piece_at(&mov.from).expect("Could not get piece to copy for iter_with_move!");
        self.piece_iter().filter(|(pos, ..)| *pos != &mov.from && *pos != &mov.to).chain(std::iter::once((&mov.to, id, copy)))
    }

    pub fn take(&mut self, pos: &Pos) -> Option<Piece> {
        self.players.iter_mut().find_map(|player| player.pieces.take(pos))
    }

    pub fn move_piece(&mut self, mov: Move) -> Option<(S, Piece)> {
        let player = self.players.iter_mut().find(|player| player.pieces.at(&mov.from).is_some()).expect("Could not find moved piece!");
        player.pieces.move_piece(mov).map(|p| (player.side.clone(), p))
    }

    pub fn reset(&mut self) {
        self.players.iter_mut().for_each(BoardPlayer::reset);
    }

}

impl<S: Side> Index<&S> for BoardPlayers<S> {
    type Output = BoardPlayer<S>;

    fn index(&self, side: &S) -> &Self::Output {
        self.get(side)
    }
}

impl<S: Side> IndexMut<&S> for BoardPlayers<S> {
    fn index_mut(&mut self, side: &S) -> &mut Self::Output {
        self.get_mut(side)
    }
}

// #[derive(Default)]
pub struct BoardPlayer<S: Side> {
    pub side: S,
    pieces: PlayerPieces,

    /// Time remaining
    time: Duration,

    // flags
    pub resign: bool,
    pub draw: bool,
    // pub check: Option<Vec<Move>>,
}

impl<S: Side> BoardPlayer<S> {

    pub fn should_move(&self) -> bool {
        !self.resign //&& self.check.as_ref().map(Vec::is_empty).unwrap_or_default()
    }

    pub fn pieces(&self) -> impl Iterator<Item = (&Pos, &Piece)> {
        self.pieces.iter()
    }
    
    pub fn fill(&mut self) {
        self.pieces.fill_with(self.side.pieces());
    }

    pub fn reset(&mut self) {
        self.pieces.clear();
        self.fill();
    }

}

impl<S: Side> Default for BoardPlayers<S> {
    fn default() -> Self {
        let mut side = S::default();
        let mut players = Vec::new();
        loop {
            players.push(BoardPlayer { side: side.clone(), pieces: side.pieces().collect(), time: Default::default(), resign: false, draw: false });
            side.next();
            if side.first() {
                break;
            }
        }
        Self { players }
    }
}