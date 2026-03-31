use core::fmt::{Display, Result as FmtResult, Formatter};
use core::ops::{Add, Mul, Sub};
use core::str::FromStr;

use std::error::Error;
use std::ops::Neg;

pub type PosInt = i8;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: PosInt,
    pub y: PosInt,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
}

pub trait Rectangle {
    fn contains(&self, pos: &Pos) -> bool;

    fn iter(&self) -> impl DoubleEndedIterator<Item = Pos> + '_;

    fn enumerate(&self) -> impl DoubleEndedIterator<Item = (PosInt, Pos)> + '_ {
        self.iter().enumerate().map(|(i, pos)| (i as PosInt, pos))
    }
}

pub struct Line {
    pub start: Pos,
    pub direction: Direction,
    pub length: usize,
}

pub struct PosBox {
    pub min: Pos,
    pub max: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub from: Pos,
    pub to: Pos,
}

// pub struct BoardPiece()

impl Pos {

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

    pub const fn is_diagonal(self) -> bool {
        self.x.abs() == self.y.abs()
    }

    pub const fn normalize(self) -> Self {
        let max = self.max().abs();
        match max == 0 {
            true => Self { x: 0, y: 0 },
            false => Self { x: self.x / max, y: self.y / max },
        }
    }

    pub fn rotate(&self, forward: Direction) -> Pos {
        match forward {
            Direction::XPos => Pos { x: self.y, y: -self.x },
            Direction::XNeg => Pos { x: -self.y, y: self.x },
            Direction::YPos => *self,
            Direction::YNeg => Pos { x: -self.x, y: -self.y },
        }
    }

}

impl Rectangle for Line {

    fn contains(&self, pos: &Pos) -> bool {
        self.start.x <= pos.x && pos.x < self.start.x + match self.direction {
            Direction::XPos | Direction::XNeg => self.length as PosInt,
            Direction::YPos | Direction::YNeg => 1,
        } && self.start.y <= pos.y && pos.y < self.start.y + match self.direction {
            Direction::YPos | Direction::YNeg => self.length as PosInt,
            Direction::XPos | Direction::XNeg => 1,
        }
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = Pos> + '_ {
        (0..self.length as PosInt).map(move |i| self.start + self.direction * i)
    }

    fn enumerate(&self) -> impl DoubleEndedIterator<Item = (PosInt, Pos)> + '_ {
        (0..self.length as PosInt).map(move |i| (i, self.start + self.direction * i))
    }

}

impl Rectangle for PosBox {

    fn contains(&self, pos: &Pos) -> bool {
        pos.x >= self.min.x && pos.x < self.max.x && pos.y >= self.min.y && pos.y < self.max.y
    }
    
    fn iter(&self) -> impl DoubleEndedIterator<Item = Pos> + '_ {
        (self.min.x..self.max.x).flat_map(move |x| (self.min.y..self.max.y).map(move |y| Pos { x, y }))
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

impl Mul<PosInt> for Pos {
    type Output = Self;

    fn mul(self, rhs: PosInt) -> Self::Output {
        Self { x: rhs, y :rhs} * self
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

impl From<Direction> for Pos {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::XPos => Pos { x: 1, y: 0 },
            Direction::XNeg => Pos { x: -1, y: 0 },
            Direction::YPos => Pos { x: 0, y: 1 },
            Direction::YNeg => Pos { x: 0, y: -1 },
        }
    }
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, other: Direction) -> Pos {
        self + Pos::from(other)
    }
}

impl Mul<i8> for Direction {
    type Output = Pos;

    fn mul(self, other: i8) -> Pos {
        Pos::from(self) * other
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Direction::XPos => Direction::XNeg,
            Direction::XNeg => Direction::XPos,
            Direction::YPos => Direction::YNeg,
            Direction::YNeg => Direction::YPos,
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
        let mut chars = s.chars();
        let xchar = chars.next().unwrap();
        if !('a'..='z').contains(&xchar) {
            return Err(ParsePosError::Char);
        }
        let x = xchar as PosInt - ('a' as PosInt);
        let y =  chars.as_str().parse::<PosInt>().map_err(|_| ParsePosError::Digit)? - 1;
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
}

impl ParsePositions {

    pub fn parse(line: &str) -> Result<Self, ParsePosError> {
        let mut parts = line.split_whitespace();
        let from = match parts.next().ok_or(ParsePosError::Length).and_then(str::parse) {
            Ok(pos) => pos,
            Err(err) => return Err(err),
        };
        match parts.next() {
            Some(pos) => match pos.parse() {
                Ok(to) => Ok(Self::Move(Move { from, to })),
                Err(err) => Err(err),
            },
            None => Ok(Self::Pos(from)),
        }
    }

}

#[cfg(feature = "serde")]
mod serde {

    use super::*;
    use ::serde::de::Visitor;
    use ::serde::{Deserialize, Serialize, Serializer, Deserializer, de::Error};
    
    impl <'d> Deserialize<'d> for Pos {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'d> {
            deserializer.deserialize_string(PosVisitor)
        }
    }

    impl Serialize for Pos {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.collect_str(self)
        }
    }

    impl<'d> Deserialize<'d> for Move {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'d> {
            deserializer.deserialize_string(MoveVisitor)
        }
    }

    impl Serialize for Move {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
            serializer.collect_str(self)
        }
    }

    struct PosVisitor;

    impl<'de> Visitor<'de> for PosVisitor {
        type Value = Pos;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a position in the format 'a1' to 'h8'")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
            value.parse().map_err(Error::custom)
        }
    }

    struct MoveVisitor;

    impl<'de> Visitor<'de> for MoveVisitor {
        type Value = Move;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "a move in the format 'a1 b2'")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> where E: Error {
            value.parse().map_err(Error::custom)
        }
    }

}