use crate::error::ShogiUtilError::CsaParseError;
use crate::piece::Piece;
use crate::{Result, ShogiUtilError};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Color {
    Black,
    White,
}

impl Color {
    pub fn to_byte(&self) -> u8 {
        match self {
            Color::Black => 0,
            Color::White => 1,
        }
    }
    pub fn to_usize(&self) -> usize {
        self.to_byte() as usize
    }

    pub fn opponent(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

impl FromStr for Color {
    type Err = ShogiUtilError;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(Color::Black),
            "-" => Ok(Color::White),
            _ => Err(CsaParseError(format!("Invalid color symbol: {}", s))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Square {
    pub file: u8,
    pub rank: u8,
}

impl Square {
    pub fn from_pos(i: usize, j: usize) -> Square {
        assert!(i < 9 && j < 9);
        Square {
            rank: (i + 1) as u8,
            file: (9 - j) as u8,
        }
    }
    pub fn to_pos(&self) -> (usize, usize) {
        assert!(self.is_valid());
        let i = self.rank as usize - 1;
        let j = 9 - self.file as usize;
        (i, j)
    }

    pub fn is_valid(&self) -> bool {
        1 <= self.rank && self.rank <= 9 && 1 <= self.file && self.file <= 9
    }
}

impl FromStr for Square {
    type Err = ShogiUtilError;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 {
            Err(CsaParseError(format!("Invalid square: {}", s)))
        } else {
            let file = s[0..1].parse::<u8>();
            let rank = s[1..2].parse::<u8>();
            match (file, rank) {
                (Ok(file), Ok(rank)) if file > 0 && rank > 0 => Ok(Square { file, rank }),
                _ => Err(CsaParseError(format!("Invalid square {}", s))),
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Move {
    pub color: Color,
    pub from: Option<Square>,
    pub to: Square,
    pub piece: Piece,
}
