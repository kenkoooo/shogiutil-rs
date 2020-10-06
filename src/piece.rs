use crate::error::ShogiUtilError::CsaParseError;
use crate::ShogiUtilError;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Piece {
    None,
    Pawn,
    Lance,
    Knight,
    Silver,
    Gold,
    Bishop,
    Rook,
    King,
    ProPawn,
    ProLance,
    ProKnight,
    ProSilver,
    ProBishop,
    ProRook,
}

impl FromStr for Piece {
    type Err = ShogiUtilError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "* " => Ok(Piece::None),
            "FU" => Ok(Piece::Pawn),
            "KY" => Ok(Piece::Lance),
            "KE" => Ok(Piece::Knight),
            "GI" => Ok(Piece::Silver),
            "KI" => Ok(Piece::Gold),
            "KA" => Ok(Piece::Bishop),
            "HI" => Ok(Piece::Rook),
            "OU" => Ok(Piece::King),
            "TO" => Ok(Piece::ProPawn),
            "NY" => Ok(Piece::ProLance),
            "NK" => Ok(Piece::ProKnight),
            "NG" => Ok(Piece::ProSilver),
            "UM" => Ok(Piece::ProBishop),
            "RY" => Ok(Piece::ProRook),
            _ => Err(CsaParseError(format!("Invalid piece type: {}", s))),
        }
    }
}
