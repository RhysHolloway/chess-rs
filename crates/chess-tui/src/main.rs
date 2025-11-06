pub mod local;
pub mod reader;
mod side;

use chess_lib::boards::{Chess360, DefaultSides};
use chess_lib::{Move, MoveError, ParsePositions, Pos};
use local::LocalClient;
use reader::Reader;

fn main() {
    let args = std::env::args()
    .skip(1)
        .collect::<Vec<String>>();
    if args.contains(&"--360".to_string()) {
        run(LocalClient::<Chess360>::default(), Reader::new(&args));
    } else {
        run(LocalClient::<DefaultSides>::default(), Reader::new(&args));
    }
}

pub trait Client {
    fn print(&self);

    fn taken(&self);

    fn reset(&mut self);

    fn move_piece(&mut self, reader: &Reader, mov: Move) -> Result<(), MoveError>;

    fn position(&self, reader: &Reader, pos: Pos);
}

pub fn run(mut client: impl Client, mut io: Reader) -> Vec<MoveError> {
    let mut input = String::new();
    let mut errors = Vec::new();
    println!("Chess engine running... Type \"help\" for commands");
    while io.read_line(&mut input) {
        match input.trim() {
            "exit" => break,
            "print" => client.print(),
            "taken" => client.taken(),
            "reset" => client.reset(),
            "help" => {
                println!("Commands: exit, print, taken, reset, help");
                println!("To see the status of a piece, type its position (e.g. \"a1\")");
                println!("To move a piece, type the move (e.g. \"e2 e4\")");
            }
            line => match ParsePositions::parse(line) {
                Ok(ParsePositions::Move(mov)) => {
                    if let Err(err) = client.move_piece(&io, mov) {
                        errors.push(err);
                    }
                }
                Ok(ParsePositions::Pos(pos)) => {
                    client.position(&io, pos);
                }
                Err(err) => println!("Invalid move command \"{line}\" with error {err}"),
            },
        }
        input.clear();
    }
    errors
}

#[cfg(test)]
mod tests {

    macro_rules! case {
        ( $x : literal ) => {
            crate::run(
                crate::local::LocalClient::<crate::DefaultSides>::default(),
                crate::reader::Reader::literal(include_str!($x)),
            )
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
