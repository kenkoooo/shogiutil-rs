use crate::error::ShogiUtilError::CsaParseError;
use crate::{Bitboard, ShogiUtilError, Square};
use std::str::FromStr;

mod moves;
use moves::piece_moves;

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

impl From<u8> for Piece {
    fn from(b: u8) -> Self {
        match b {
            0 => Piece::None,
            1 => Piece::Pawn,
            2 => Piece::Lance,
            3 => Piece::Knight,
            4 => Piece::Silver,
            5 => Piece::Gold,
            6 => Piece::Bishop,
            7 => Piece::Rook,
            8 => Piece::King,
            9 => Piece::ProPawn,
            10 => Piece::ProLance,
            11 => Piece::ProKnight,
            12 => Piece::ProSilver,
            13 => Piece::ProBishop,
            14 => Piece::ProRook,
            _ => unreachable!(),
        }
    }
}

impl Piece {
    pub fn to_byte(&self) -> u8 {
        match self {
            Piece::None => 0,
            Piece::Pawn => 1,
            Piece::Lance => 2,
            Piece::Knight => 3,
            Piece::Silver => 4,
            Piece::Gold => 5,
            Piece::Bishop => 6,
            Piece::Rook => 7,
            Piece::King => 8,
            Piece::ProPawn => 9,
            Piece::ProLance => 10,
            Piece::ProKnight => 11,
            Piece::ProSilver => 12,
            Piece::ProBishop => 13,
            Piece::ProRook => 14,
        }
    }

    pub fn to_usize(&self) -> usize {
        self.to_byte() as usize
    }

    pub fn promote(&self) -> Option<Piece> {
        match self {
            Piece::Pawn => Some(Piece::ProPawn),
            Piece::Lance => Some(Piece::ProLance),
            Piece::Knight => Some(Piece::ProKnight),
            Piece::Silver => Some(Piece::ProSilver),
            Piece::Bishop => Some(Piece::ProBishop),
            Piece::Rook => Some(Piece::ProRook),
            _ => None,
        }
    }

    pub fn to_csa(&self) -> String {
        match self {
            Piece::None => "* ".to_string(),
            Piece::Pawn => "FU".to_string(),
            Piece::Lance => "KY".to_string(),
            Piece::Knight => "KE".to_string(),
            Piece::Silver => "GI".to_string(),
            Piece::Gold => "KI".to_string(),
            Piece::Bishop => "KA".to_string(),
            Piece::Rook => "HI".to_string(),
            Piece::King => "OU".to_string(),
            Piece::ProPawn => "TO".to_string(),
            Piece::ProLance => "NY".to_string(),
            Piece::ProKnight => "NK".to_string(),
            Piece::ProSilver => "NG".to_string(),
            Piece::ProBishop => "UM".to_string(),
            Piece::ProRook => "RY".to_string(),
        }
    }

    pub fn to_sfen(&self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Lance => 'L',
            Piece::Knight => 'N',
            Piece::Silver => 'S',
            Piece::Gold => 'G',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::King => 'K',
            _ => unreachable!(),
        }
    }

    pub fn revert_promotion(&self) -> Option<Piece> {
        match self {
            Piece::ProPawn => Some(Piece::Pawn),
            Piece::ProLance => Some(Piece::Lance),
            Piece::ProKnight => Some(Piece::Knight),
            Piece::ProSilver => Some(Piece::Silver),
            Piece::ProBishop => Some(Piece::Bishop),
            Piece::ProRook => Some(Piece::Rook),
            _ => None,
        }
    }

    pub fn is_valid_promotion(&self, promoted: &Piece) -> bool {
        if let Some(valid_promotion) = self.promote() {
            &valid_promotion == promoted
        } else {
            false
        }
    }

    pub fn is_promoted(&self) -> bool {
        self == &Piece::ProPawn
            || self == &Piece::ProLance
            || self == &Piece::ProKnight
            || self == &Piece::ProSilver
            || self == &Piece::ProBishop
            || self == &Piece::ProRook
    }

    pub const fn max_piece_in_hand(&self) -> usize {
        match self {
            Piece::Pawn => 18,
            Piece::Lance => 4,
            Piece::Knight => 4,
            Piece::Silver => 4,
            Piece::Gold => 4,
            Piece::Bishop => 2,
            Piece::Rook => 2,
            _ => 0,
        }
    }

    pub fn generate_moves(&self, from: &Square, occupied: &[Bitboard; 2], moves: &mut Vec<Square>) {
        piece_moves(*self, from, occupied, moves)
    }
}

#[cfg(test)]
mod tests {
    use crate::debug::generate_bitboard;
    use crate::piece::Piece;
    use crate::Square;

    #[test]
    fn test_from_to_byte() {
        for i in 0..15 {
            let piece = Piece::from(i);
            assert_eq!(piece.to_byte(), i);
        }
    }

    #[test]
    fn test_pawn_generate_moves() {
        let occupied = [
            generate_bitboard(
                r"
            .........
            .........
            .........
            ....#....
            .........
            .........
            .........
            .........
            .........
        ",
            ),
            generate_bitboard(
                r"
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
        ];
        let mut moves = vec![];
        Piece::Pawn.generate_moves(&Square { file: 5, rank: 5 }, &occupied, &mut moves);
        assert!(moves.is_empty());

        let occupied = [
            generate_bitboard(
                r"
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
            generate_bitboard(
                r"
            .........
            .........
            .........
            ....#....
            .........
            .........
            .........
            .........
            .........
        ",
            ),
        ];
        let mut moves = vec![];
        Piece::Pawn.generate_moves(&Square { file: 5, rank: 5 }, &occupied, &mut moves);
        assert_eq!(moves, [Square { file: 5, rank: 4 }]);
    }
    #[test]
    fn test_lance_generate_moves() {
        let occupied = [
            generate_bitboard(
                r"
            .........
            ....#....
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
            generate_bitboard(
                r"
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
        ];
        let mut moves = vec![];
        Piece::Lance.generate_moves(&Square { file: 5, rank: 5 }, &occupied, &mut moves);
        assert_eq!(
            moves,
            [Square { file: 5, rank: 4 }, Square { file: 5, rank: 3 }]
        );

        let occupied = [
            generate_bitboard(
                r"
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
            generate_bitboard(
                r"
            .........
            ....#....
            .........
            .........
            .........
            .........
            .........
            .........
            .........
        ",
            ),
        ];
        let mut moves = vec![];
        Piece::Lance.generate_moves(&Square { file: 5, rank: 5 }, &occupied, &mut moves);
        assert_eq!(
            moves,
            [
                Square { file: 5, rank: 4 },
                Square { file: 5, rank: 3 },
                Square { file: 5, rank: 2 }
            ]
        );
    }
}
