use chess_lib::Side;
use chess_lib::{Board, Move, MoveError, Pos};

use crate::reader::Reader;
use crate::side::SideTui;
use crate::Client;

pub struct LocalClient<S: Side> {
    board: Board<S>,
    size: Pos,
}

impl<S: Side + SideTui + 'static> LocalClient<S> {

    fn print(&self) {
        // println!("size: {:?}", self.size);
        println!();
        println!("{}'s turn", self.board.turn.side);
        // if let Some(moves) = self.board.state.check() {
        //     match moves.is_empty() {
        //         true => println!("{:?} is in checkmate!", self.board.state.turn.side),
        //         false => println!("{:?} is in check with available moves {moves:?}", self.board.state.turn.side),
        //     }
        // }
        for y in (0..self.size.y).rev() {
            print!("{:<3}", y + 1);
            (0..self.size.x).for_each(|x| {
                match S::on_board(&Pos { x, y }) {
                    true => {
                        match (x + y) % 2 == 0 {
                            false => print!("\x1b[107m"),
                            true => print!("\x1b[40m"),
                        }
                        match self.board.players.piece_at(&Pos { x, y }) {
                            Some((side, piece)) => side.print_symbol(piece),
                            None => print!(" "),
                        }
                    },
                    false => print!("#"),
                }
                print!(" \x1b[0m");
            });
            println!("| ");
        }
        print!("#  ");
        ('a'..).take(self.size.x as usize).for_each(|c| print!("{} ", c));
        println!("|");
    
    }
}

impl<S: Side + SideTui + 'static> Client for LocalClient<S>  {
    fn reset(&mut self) {
        self.board.reset();
    }

    fn move_piece(&mut self, _reader: &Reader, mov: Move) -> Result<(), MoveError> {
        self.board.move_piece(mov).inspect(|()| self.print()).inspect_err(|err| println!("Could not perform move {mov} with error {err:?}"))
    }
    
    fn print(&self) {
        self.print();
    }
    
    fn taken(&self) {
        println!();
        let mut side = S::default();
        loop {
            print!("{side}: ");
            for (side, piece) in self.board.history.taken(&side) {
                side.print_symbol(piece);
                print!(" ");
            }
            side.next();
            if side.first() {
                break;
            }
        }
    }
    
    fn position(&self, _reader: &Reader, pos: Pos) {
        match self.board.players.piece_at(&pos) {
            Some((side, piece)) => {
                println!("{:?} at {}", piece, pos);

                let moves = piece.moves(&self.board, pos, side).collect::<Vec<_>>();

                let mut targets = piece.targets(&self.board, pos, side).filter(|target| !moves.contains(target));

                match targets.next() {
                    Some(first) => {
                        print!("Targets: {first}");
                        for mov in targets {
                            print!(", {}", mov);
                        }
                        println!();
                    },
                    None => println!("No targets"),
                }

                match moves.get(0) {
                    Some(first) => {
                        print!("Moves: {first}");
                        for mov in &moves[1..] {
                            print!(", {}", mov);
                        }
                        println!();
                    },
                    None => println!("No moves"),
                }
            },
            None => println!("No piece at {}", pos),
        }
    }
}

impl<S: Side> Default for LocalClient<S> {
    fn default() -> Self {
        Self { 
            board: Default::default(), 
            size: Pos { 
                x: S::dimensions().into_iter().map(|p| p.max.x).max().unwrap() - S::dimensions().into_iter().map(|p| p.min.x).min().unwrap(), 
                y: S::dimensions().into_iter().map(|p| p.max.y).max().unwrap() - S::dimensions().into_iter().map(|p| p.min.y).min().unwrap(),
            } 
        }
    }
}