use crate::error::ShogiUtilError::InvalidMove;
use crate::piece::Piece;
use crate::{Color, Move, Result, Square};
use std::ops::BitAnd;

const fn bit(file: u8, rank: u8) -> u128 {
    1 << ((rank - 1) * 9 + 9 - file)
}

const fn bit_rank(rank: u8) -> u128 {
    bit(1, rank)
        | bit(2, rank)
        | bit(3, rank)
        | bit(4, rank)
        | bit(5, rank)
        | bit(6, rank)
        | bit(7, rank)
        | bit(8, rank)
        | bit(9, rank)
}

const PIECE_TYPES: usize = 15;

pub struct Board {
    pub piece_bb: [Bitboard; PIECE_TYPES],
    pub pieces_in_hand: [[u8; PIECE_TYPES]; 2],
    pub occupied: [Bitboard; 2],
    pub black_turn: bool,
}

impl Default for Board {
    fn default() -> Self {
        let piece_bb = [
            Bitboard(0),                                             // NONE
            Bitboard(bit_rank(3) | bit_rank(7)),                     // PAWN
            Bitboard(bit(1, 1) | bit(9, 1) | bit(1, 9) | bit(9, 9)), // LANCE
            Bitboard(bit(2, 1) | bit(8, 1) | bit(2, 9) | bit(8, 9)), // KNIGHT
            Bitboard(bit(3, 1) | bit(7, 1) | bit(3, 9) | bit(7, 9)), // SILVER
            Bitboard(bit(4, 1) | bit(6, 1) | bit(4, 9) | bit(6, 9)), // GOLD
            Bitboard(bit(2, 2) | bit(8, 8)),                         // BISHOP
            Bitboard(bit(2, 8) | bit(8, 2)),                         // ROOK
            Bitboard(bit(5, 1) | bit(5, 9)),                         // KING
            Bitboard(0),                                             // PROM_PAWN
            Bitboard(0),                                             // PROM_LANCE
            Bitboard(0),                                             // PROM_KNIGHT
            Bitboard(0),                                             // PROM_SILVER
            Bitboard(0),                                             // PROM_BISHOP
            Bitboard(0),                                             // PROM_ROOK
        ];
        let occupied = [
            Bitboard(bit_rank(9) | bit(8, 8) | bit(2, 8) | bit_rank(7)),
            Bitboard(bit_rank(3) | bit(8, 2) | bit(2, 2) | bit_rank(1)),
        ];
        let pieces_in_hand = [[0; 15]; 2];
        Self {
            piece_bb,
            pieces_in_hand,
            occupied,
            black_turn: true,
        }
    }
}

impl Board {
    pub fn push_move(&mut self, mv: Move) -> Result<()> {
        let color = mv.color;
        let piece = mv.piece;
        let to = mv.to;
        let _prev_piece: Result<Piece> = if let Some(from) = mv.from.as_ref() {
            let prev_piece = self.remove_piece(from, color)?;
            if prev_piece != piece {
                if !prev_piece.is_valid_promotion(&piece) {
                    return Err(InvalidMove(format!(
                        "Invalid promotion: {:?} => {:?}",
                        prev_piece, mv.piece
                    )));
                }
                assert!(mv.piece.is_promoted());

                let can_promote = match color {
                    Color::Black => from.rank <= 3 || to.rank <= 3,
                    Color::White => from.rank >= 7 || to.rank >= 7,
                };
                if !can_promote {
                    return Err(InvalidMove(format!(
                        "{:?} can not promote at {:?} nor {:?}",
                        color, from, to
                    )));
                }
            }
            Ok(prev_piece)
        } else {
            self.remove_hand(color, piece)?;
            assert!(!piece.is_promoted());
            Ok(piece)
        };

        // todo validate move

        let opponent = color.opponent();
        if self.occupied[opponent.to_usize()].is_filled(&to) {
            let opponent_piece = self.remove_piece(&to, opponent)?;
            self.push_hand(opponent_piece, color);
        }
        if self.occupied[color.to_usize()].is_filled(&to) {
            return Err(InvalidMove(format!(
                "{:?} already has a piece on {:?}",
                color, to
            )));
        }

        self.push_piece(&to, color, piece);
        Ok(())
    }

    fn push_hand(&mut self, piece: Piece, color: Color) {
        let piece = piece.revert_promotion().unwrap_or(piece);
        assert!(!piece.is_promoted());

        let piece = piece.to_usize();
        let color = color.to_usize();
        self.pieces_in_hand[color][piece] += 1;

        // todo validate hand limit
    }

    fn remove_hand(&mut self, color: Color, piece: Piece) -> Result<()> {
        let color = color.to_usize();
        let piece = piece.to_usize();
        if self.pieces_in_hand[color][piece] == 0 {
            Err(InvalidMove(format!("{:?} has no piece {:?}", color, piece)))
        } else {
            self.pieces_in_hand[color][piece] -= 1;
            Ok(())
        }
    }

    fn remove_piece(&mut self, sq: &Square, color: Color) -> Result<Piece> {
        let color = color.to_usize();
        if !self.occupied[color].is_filled(sq) {
            return Err(InvalidMove(format!(
                "There's no piece at {}{}",
                sq.file, sq.rank
            )));
        }

        let filled_piece_types = (0..PIECE_TYPES)
            .filter(|&i| self.piece_bb[i].is_filled(sq))
            .collect::<Vec<_>>();
        assert_eq!(filled_piece_types.len(), 1);
        let piece_type = filled_piece_types[0];

        self.piece_bb[piece_type].remove(sq);
        self.occupied[color].remove(sq);
        Ok(Piece::from(piece_type as u8))
    }

    fn push_piece(&mut self, sq: &Square, color: Color, piece: Piece) {
        assert!(!self.occupied[color.to_usize()].is_filled(sq));
        self.occupied[color.to_usize()].fill(sq);
        self.piece_bb[piece.to_usize()].fill(sq);
    }
}

#[derive(Clone, Copy)]
pub struct Bitboard(u128);

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl Bitboard {
    pub fn is_filled(&self, sq: &Square) -> bool {
        let pos = sq_to_pos(sq);
        self.0 & (1 << pos) != 0
    }
    pub fn fill(&mut self, sq: &Square) {
        let pos = sq_to_pos(sq);
        assert_eq!(self.0 & (1 << pos), 0);
        self.0 ^= 1 << pos;
    }
    pub fn remove(&mut self, sq: &Square) {
        let pos = sq_to_pos(sq);
        assert_ne!(self.0 & (1 << pos), 0);
        self.0 ^= 1 << pos;
    }

    pub fn rotate180(&self) -> Self {
        let mut result = Bitboard(0);
        for file in 1..=9 {
            for rank in 1..=9 {
                if self.is_filled(&Square { file, rank }) {
                    result.fill(&Square {
                        file: 10 - file,
                        rank: 10 - rank,
                    });
                }
            }
        }
        result
    }
}

fn sq_to_pos(sq: &Square) -> u8 {
    (sq.rank - 1) * 9 + 9 - sq.file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_board() {
        let board = Board::default();
        assert_eq!(
            board.occupied[0].0,
            0b111111111_010000010_111111111_000000000_000000000_000000000_000000000_000000000_000000000
        );
        assert_eq!(
            board.occupied[1].0,
            0b000000000_000000000_000000000_000000000_000000000_000000000_111111111_010000010_111111111
        );
        assert_eq!(
            board.piece_bb[1].0,
            0b000000000_000000000_111111111_000000000_000000000_000000000_111111111_000000000_000000000
        );
        assert_eq!(
            board.piece_bb[2].0,
            0b100000001_000000000_000000000_000000000_000000000_000000000_000000000_000000000_100000001
        );
        assert_eq!(
            board.piece_bb[3].0,
            0b010000010_000000000_000000000_000000000_000000000_000000000_000000000_000000000_010000010
        );
        assert_eq!(
            board.piece_bb[4].0,
            0b001000100_000000000_000000000_000000000_000000000_000000000_000000000_000000000_001000100
        );
        assert_eq!(
            board.piece_bb[5].0,
            0b000101000_000000000_000000000_000000000_000000000_000000000_000000000_000000000_000101000
        );
        assert_eq!(
            board.piece_bb[6].0,
            0b000000000_000000010_000000000_000000000_000000000_000000000_000000000_010000000_000000000
        );
        assert_eq!(
            board.piece_bb[7].0,
            0b000000000_010000000_000000000_000000000_000000000_000000000_000000000_000000010_000000000
        );
        assert_eq!(
            board.piece_bb[8].0,
            0b000010000_000000000_000000000_000000000_000000000_000000000_000000000_000000000_000010000
        );
    }

    #[test]
    fn test_rotate180() {
        let b = Bitboard(0b_111111111_100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000);
        let rotated = b.rotate180();
        assert_eq!(rotated.0, 0b_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001_111111111);
    }
}
