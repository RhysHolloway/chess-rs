use std::fs::File;
use std::io::{BufRead, BufReader, Cursor, Stdin};

pub enum Reader {
    File(BufReader<File>),
    Stdin(Stdin),
    Literal(Cursor<&'static str>),
}

impl Reader {

    pub fn new(mut args: impl Iterator<Item = String>) -> Self {
        match args.next() {
            Some(arg) => Self::File(BufReader::new(File::open(arg).expect("Could not read provided file!"))),
            None => Self::Stdin(std::io::stdin()),
        }
    }

    pub fn literal(s: &'static str) -> Self {
        Self::Literal(Cursor::new(s))
    }

    pub fn read_line(&mut self, buf: &mut String) -> bool {
        let cont = match self {
            Self::File(reader) => reader.read_line(buf),
            Self::Stdin(reader) => reader.read_line(buf),
            Self::Literal(reader) => reader.read_line(buf),
        }.inspect_err(|err| eprintln!("Error: {err}")).is_ok();
        if !self.print() && buf.is_empty() {
            *self = Self::Stdin(std::io::stdin());
        }
        cont
    }

    pub fn print(&self) -> bool {
        matches!(self, Reader::Stdin(..))
    }
}
