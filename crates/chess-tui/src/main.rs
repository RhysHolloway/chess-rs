pub mod reader;

use chess_lib::{MoveError, ParsePositions};
use chess_lib::{BoardPiece, Side};
use chess_lib::{Board, Pos};
use reader::Reader;

fn main() {
    let mut args = std::env::args();
    args.next();
    run(Reader::new(args));
}

pub fn run(mut io: Reader) -> Vec<MoveError> {
    let mut board = Board::default();
    let mut errors = Vec::new();
    let mut input = String::new();
    println!("Chess engine running... Type \"help\" for commands");
    while io.read_line(&mut input) {
        match input.trim() {
            "exit" => break,
            "print" => self::print(&board),
            "taken" => {
                println!();
                Side::sides().into_iter().for_each(|side| {
                    print!("{:?}: ", side);
                    for piece in board.history.taken(side) {
                        print!("{} ", piece.kind.symbol(piece.side));
                    }
                    println!();
                });
            },
            "reset" => {
                board.reset();
            },
            "help" => {
                println!("Commands: exit, print, taken, reset, help");
                println!("To see the status of a piece, type its position (e.g. \"a1\")");
                println!("To move a piece, type the move (e.g. \"e2 e4\")");
            },
            line => {
                match ParsePositions::parse(line) {
                    ParsePositions::Move(mov) => match board.move_piece(mov) {
                        Ok(()) => {
                            // println!("{:?} moved {:?} to {}", piece.side, piece.kind, piece.position);
                            if io.print() {
                                self::print(&board);
                            }                        
                        },
                        Err(err) => {
                            errors.push(err);
                            println!("Could not perform move {mov} with error {err:?}");
                        },
                    },
                    ParsePositions::Pos(pos) => {
                        match board.pieces.at(&pos) {
                            Some(piece) => {
                                println!("{:?} at {}", piece.kind, pos);

                                let moves = piece.moves(&board, pos).collect::<Vec<_>>();

                                let mut targets = piece.targets(&board, pos).filter(|target| !moves.contains(target));

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
                    },
                    ParsePositions::Error(err) => println!("Invalid move command \"{line}\" with error {err}"),
                }
            },
        }
        input.clear();
    }
    errors
}

pub fn print(board: &Board) {
    println!();
    if let Some(moves) = board.state.check() {
        match moves.is_empty() {
            true => println!("{:?} is in checkmate!", board.state.turn.side),
            false => println!("{:?} is in check with available moves {moves:?}", board.state.turn.side),
        }
    }
    for y in (0..8).rev() {
        print!("{} ", y + 1);
        (0..8).for_each(|x| print!("{} ", board.pieces.at(&Pos { x, y }).map(BoardPiece::symbol).unwrap_or('_')));
        println!("| ");
    }
    println!("# a b c d e f g h |");

}

#[cfg(test)]
mod tests {

    macro_rules! case {
        ( $x : literal ) => {
            crate::run(crate::reader::Reader::literal(include_str!($x)))
        };
    }

    #[test]
    fn castle() {
        assert!(case!("tests/castle_test.txt").is_empty());
        assert!(case!("tests/castle_test_fail.txt").len() == 1);
    }

    #[test]
    fn en_passant() {
        assert!(case!("tests/en_passant.txt").is_empty());
    }

    #[test]
    fn check() {
        // assert!(case!("tests/check_tester.txt").is_empty());
    }
    

}