use crate::error::ShogiUtilError::InvalidMove;
use crate::piece::Piece;
use crate::{Color, Move, Result, Square};

mod bitboard;
mod legal_move;
use legal_move::generate_legal_moves;

use crate::model::LegalMove;
pub use bitboard::Bitboard;

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

const fn bit_file(file: u8) -> u128 {
    bit(file, 1)
        | bit(file, 2)
        | bit(file, 3)
        | bit(file, 4)
        | bit(file, 5)
        | bit(file, 6)
        | bit(file, 7)
        | bit(file, 8)
        | bit(file, 9)
}

const PIECE_TYPES: usize = 15;

pub struct MoveResult {
    pub promoted: bool,
}

pub struct Board {
    pub piece_bb: [Bitboard; PIECE_TYPES],
    pub pieces_in_hand: [[u8; PIECE_TYPES]; 2],
    pub occupied: [Bitboard; 2],
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
        }
    }
}

impl Board {
    pub fn empty() -> Self {
        Self {
            piece_bb: [Bitboard(0); PIECE_TYPES],
            pieces_in_hand: [[0; PIECE_TYPES]; 2],
            occupied: [Bitboard(0); 2],
        }
    }

    pub fn move_between(
        &mut self,
        from: &Square,
        to: &Square,
        promote: bool,
        color: Color,
    ) -> Result<()> {
        let piece = self.remove_piece(from, color)?;
        if self.occupied[color.opponent().to_usize()].is_filled(to) {
            let caught = self.remove_piece(to, color.opponent())?;
            self.push_hand(caught, color);
        }

        if promote {
            let piece = piece
                .promote()
                .ok_or_else(|| InvalidMove(format!("{:?} can not promote", piece)))?;
            self.push_piece(to, color, piece);
        } else {
            self.push_piece(to, color, piece);
        }

        Ok(())
    }

    pub fn push_move(&mut self, mv: Move) -> Result<MoveResult> {
        let color = mv.color;
        let piece = mv.piece;
        let to = mv.to;

        let prev_piece;
        if let Some(from) = mv.from.as_ref() {
            let maybe_prev_piece = self.remove_piece(from, color)?;
            if maybe_prev_piece != piece {
                if !maybe_prev_piece.is_valid_promotion(&piece) {
                    return Err(InvalidMove(format!(
                        "Invalid promotion: {:?} => {:?}",
                        maybe_prev_piece, piece
                    )));
                }
                assert!(piece.is_promoted());
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
            prev_piece = maybe_prev_piece;
        } else {
            self.remove_hand(color, piece)?;
            assert!(!piece.is_promoted());
            prev_piece = piece;
        }

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
        Ok(MoveResult {
            promoted: piece != prev_piece,
        })
    }

    pub fn push_hand(&mut self, piece: Piece, color: Color) {
        let piece = piece.revert_promotion().unwrap_or(piece);
        assert!(!piece.is_promoted());

        let color = color.to_usize();
        assert!(self.pieces_in_hand[color][piece.to_usize()] < piece.max_piece_in_hand() as u8);
        self.pieces_in_hand[color][piece.to_usize()] += 1;
    }

    pub fn remove_hand(&mut self, color: Color, piece: Piece) -> Result<()> {
        let color = color.to_usize();
        let piece = piece.to_usize();
        if self.pieces_in_hand[color][piece] == 0 {
            Err(InvalidMove(format!("{:?} has no piece {:?}", color, piece)))
        } else {
            self.pieces_in_hand[color][piece] -= 1;
            Ok(())
        }
    }

    pub fn remove_piece(&mut self, sq: &Square, color: Color) -> Result<Piece> {
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

    pub fn push_piece(&mut self, sq: &Square, color: Color, piece: Piece) {
        assert!(!self.occupied[color.to_usize()].is_filled(sq));
        self.occupied[color.to_usize()].fill(sq);
        self.piece_bb[piece.to_usize()].fill(sq);
    }

    pub fn generate_legal_moves(&self) -> Vec<LegalMove> {
        generate_legal_moves(self)
    }

    pub fn rotate180(&self) -> Self {
        let mut piece_bb = [Bitboard(0); PIECE_TYPES];
        for (i, bb) in self.piece_bb.iter().enumerate() {
            piece_bb[i] = bb.rotate180();
        }
        let occupied = [self.occupied[1].rotate180(), self.occupied[0].rotate180()];
        let pieces_in_hand = [self.pieces_in_hand[1], self.pieces_in_hand[0]];
        Self {
            piece_bb,
            pieces_in_hand,
            occupied,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::dump_board;

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
    fn test_generate_legal_moves() {
        let board = Board::default();
        assert_eq!(
            r"P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
P2 * -HI *  *  *  *  * -KA * 
P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
P4 *  *  *  *  *  *  *  *  * 
P5 *  *  *  *  *  *  *  *  * 
P6 *  *  *  *  *  *  *  *  * 
P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
P8 * +KA *  *  *  *  * +HI * 
P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
P+
P-
",
            dump_board(&board)
        );
        assert_eq!(
            r"P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
P2 * -HI *  *  *  *  * -KA * 
P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
P4 *  *  *  *  *  *  *  *  * 
P5 *  *  *  *  *  *  *  *  * 
P6 *  *  *  *  *  *  *  *  * 
P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
P8 * +KA *  *  *  *  * +HI * 
P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
P+
P-
",
            dump_board(&board.rotate180())
        );
    }
}
