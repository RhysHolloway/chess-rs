use core::fmt::{Display, Result as FmtResult, Formatter};
use core::ops::{Add, Mul, Sub};
use core::str::FromStr;

use std::error::Error;

pub type PosInt = i8;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: PosInt,
    pub y: PosInt,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

impl Pos {

    pub const CHAR_MIN: char = 'a';
    /// Inclusive
    pub const CHAR_MAX: char = 'h';

    pub const fn directions() -> [Self; 4] {
        [
            Self { x: 1, y: 0 },
            Self { x: -1, y: 0 },
            Self { x: 0, y: 1 },
            Self { x: 0, y: -1 },
        ]
    }

    pub const fn diagonals() -> [Self; 4] {
        [
            Self { x: 1, y: 1 },
            Self { x: -1, y: -1 },
            Self { x: -1, y: 1 },
            Self { x: 1, y: -1 },
        ]
    }
    
    pub const fn max(&self) -> PosInt  {
        if self.x > self.y { self.x } else { self.y }
    }
    
    pub const fn abs(self) -> Self {
        Self { x: self.x.abs(), y: self.y.abs() }
    }

}

impl Move {

    pub const fn new(from: Pos, to: Pos) -> Self {
        Self { from, to }
    }

}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Pos {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<i8> for Pos {
    type Output = Self;

    fn mul(self, scalar: i8) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}{}", ('a' as u8 + self.x as u8) as char, self.y + 1)
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {}", self.from, self.to)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParsePosError {
    Char,
    Digit,
    Length,
}

impl Display for ParsePosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Invalid {}", match self {
            Self::Char => "character",
            Self::Digit => "digit",
            Self::Length => "length",
        })
    }
}

impl Error for ParsePosError {}

impl FromStr for Pos {
    type Err = ParsePosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParsePosError::Length);
        }
        let mut chars = s.chars();
        let xchar = chars.next().unwrap();
        if xchar < Self::CHAR_MIN || xchar > Self::CHAR_MAX {
            return Err(ParsePosError::Char);
        }
        let x = xchar as PosInt - (Self::CHAR_MIN as PosInt);
        let y = chars.next().unwrap().to_digit(10).ok_or(ParsePosError::Digit)? as PosInt - 1;
        if !(0..8).contains(&y) {
            return Err(ParsePosError::Digit);
        }
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseMoveError {
    Pos(bool, ParsePosError),
    Length,
}

impl Display for ParseMoveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ParseMoveError::Pos(first, err) => write!(f, "Invalid {} position with error {}", if *first { "first" } else { "second" }, err),
            ParseMoveError::Length => write!(f, "Invalid length / components"),
        }
    }
}

impl Error for ParseMoveError {}

impl FromStr for Move {
    type Err = ParseMoveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let first = parts.next().ok_or(ParseMoveError::Length)?.parse::<Pos>().map_err(|e| ParseMoveError::Pos(true, e))?;
        let second = parts.next().ok_or(ParseMoveError::Length)?.parse::<Pos>().map_err(|e| ParseMoveError::Pos(false, e))?;
        if parts.next().is_some() {
            return Err(ParseMoveError::Length);
        } else {
            return Ok(Self { from: first, to: second });
        }
    }
}


pub enum ParsePositions {
    Pos(Pos),
    Move(Move),
    Error(ParsePosError),
}

impl ParsePositions {

    pub fn parse(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let from = match parts.next().ok_or(ParsePosError::Length).and_then(str::parse) {
            Ok(pos) => pos,
            Err(err) => return Self::Error(err),
        };
        match parts.next() {
            Some(pos) => match pos.parse() {
                Ok(to) => Self::Move(Move { from, to }),
                Err(err) => Self::Error(err),
            },
            None => Self::Pos(from),
        }
    }

}