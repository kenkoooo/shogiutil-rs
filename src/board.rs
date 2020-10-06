use crate::Move;

const BB_VOID: u128 = 0;

const BB_11: u128 = 1 << ((1 - 1) * 9 + 9 - 1);
const BB_12: u128 = 1 << ((2 - 1) * 9 + 9 - 1);
const BB_13: u128 = 1 << ((3 - 1) * 9 + 9 - 1);
const BB_14: u128 = 1 << ((4 - 1) * 9 + 9 - 1);
const BB_15: u128 = 1 << ((5 - 1) * 9 + 9 - 1);
const BB_16: u128 = 1 << ((6 - 1) * 9 + 9 - 1);
const BB_17: u128 = 1 << ((7 - 1) * 9 + 9 - 1);
const BB_18: u128 = 1 << ((8 - 1) * 9 + 9 - 1);
const BB_19: u128 = 1 << ((9 - 1) * 9 + 9 - 1);
const BB_21: u128 = 1 << ((1 - 1) * 9 + 9 - 2);
const BB_22: u128 = 1 << ((2 - 1) * 9 + 9 - 2);
const BB_23: u128 = 1 << ((3 - 1) * 9 + 9 - 2);
const BB_24: u128 = 1 << ((4 - 1) * 9 + 9 - 2);
const BB_25: u128 = 1 << ((5 - 1) * 9 + 9 - 2);
const BB_26: u128 = 1 << ((6 - 1) * 9 + 9 - 2);
const BB_27: u128 = 1 << ((7 - 1) * 9 + 9 - 2);
const BB_28: u128 = 1 << ((8 - 1) * 9 + 9 - 2);
const BB_29: u128 = 1 << ((9 - 1) * 9 + 9 - 2);
const BB_31: u128 = 1 << ((1 - 1) * 9 + 9 - 3);
const BB_32: u128 = 1 << ((2 - 1) * 9 + 9 - 3);
const BB_33: u128 = 1 << ((3 - 1) * 9 + 9 - 3);
const BB_34: u128 = 1 << ((4 - 1) * 9 + 9 - 3);
const BB_35: u128 = 1 << ((5 - 1) * 9 + 9 - 3);
const BB_36: u128 = 1 << ((6 - 1) * 9 + 9 - 3);
const BB_37: u128 = 1 << ((7 - 1) * 9 + 9 - 3);
const BB_38: u128 = 1 << ((8 - 1) * 9 + 9 - 3);
const BB_39: u128 = 1 << ((9 - 1) * 9 + 9 - 3);
const BB_41: u128 = 1 << ((1 - 1) * 9 + 9 - 4);
const BB_42: u128 = 1 << ((2 - 1) * 9 + 9 - 4);
const BB_43: u128 = 1 << ((3 - 1) * 9 + 9 - 4);
const BB_44: u128 = 1 << ((4 - 1) * 9 + 9 - 4);
const BB_45: u128 = 1 << ((5 - 1) * 9 + 9 - 4);
const BB_46: u128 = 1 << ((6 - 1) * 9 + 9 - 4);
const BB_47: u128 = 1 << ((7 - 1) * 9 + 9 - 4);
const BB_48: u128 = 1 << ((8 - 1) * 9 + 9 - 4);
const BB_49: u128 = 1 << ((9 - 1) * 9 + 9 - 4);
const BB_51: u128 = 1 << ((1 - 1) * 9 + 9 - 5);
const BB_52: u128 = 1 << ((2 - 1) * 9 + 9 - 5);
const BB_53: u128 = 1 << ((3 - 1) * 9 + 9 - 5);
const BB_54: u128 = 1 << ((4 - 1) * 9 + 9 - 5);
const BB_55: u128 = 1 << ((5 - 1) * 9 + 9 - 5);
const BB_56: u128 = 1 << ((6 - 1) * 9 + 9 - 5);
const BB_57: u128 = 1 << ((7 - 1) * 9 + 9 - 5);
const BB_58: u128 = 1 << ((8 - 1) * 9 + 9 - 5);
const BB_59: u128 = 1 << ((9 - 1) * 9 + 9 - 5);
const BB_61: u128 = 1 << ((1 - 1) * 9 + 9 - 6);
const BB_62: u128 = 1 << ((2 - 1) * 9 + 9 - 6);
const BB_63: u128 = 1 << ((3 - 1) * 9 + 9 - 6);
const BB_64: u128 = 1 << ((4 - 1) * 9 + 9 - 6);
const BB_65: u128 = 1 << ((5 - 1) * 9 + 9 - 6);
const BB_66: u128 = 1 << ((6 - 1) * 9 + 9 - 6);
const BB_67: u128 = 1 << ((7 - 1) * 9 + 9 - 6);
const BB_68: u128 = 1 << ((8 - 1) * 9 + 9 - 6);
const BB_69: u128 = 1 << ((9 - 1) * 9 + 9 - 6);
const BB_71: u128 = 1 << ((1 - 1) * 9 + 9 - 7);
const BB_72: u128 = 1 << ((2 - 1) * 9 + 9 - 7);
const BB_73: u128 = 1 << ((3 - 1) * 9 + 9 - 7);
const BB_74: u128 = 1 << ((4 - 1) * 9 + 9 - 7);
const BB_75: u128 = 1 << ((5 - 1) * 9 + 9 - 7);
const BB_76: u128 = 1 << ((6 - 1) * 9 + 9 - 7);
const BB_77: u128 = 1 << ((7 - 1) * 9 + 9 - 7);
const BB_78: u128 = 1 << ((8 - 1) * 9 + 9 - 7);
const BB_79: u128 = 1 << ((9 - 1) * 9 + 9 - 7);
const BB_81: u128 = 1 << ((1 - 1) * 9 + 9 - 8);
const BB_82: u128 = 1 << ((2 - 1) * 9 + 9 - 8);
const BB_83: u128 = 1 << ((3 - 1) * 9 + 9 - 8);
const BB_84: u128 = 1 << ((4 - 1) * 9 + 9 - 8);
const BB_85: u128 = 1 << ((5 - 1) * 9 + 9 - 8);
const BB_86: u128 = 1 << ((6 - 1) * 9 + 9 - 8);
const BB_87: u128 = 1 << ((7 - 1) * 9 + 9 - 8);
const BB_88: u128 = 1 << ((8 - 1) * 9 + 9 - 8);
const BB_89: u128 = 1 << ((9 - 1) * 9 + 9 - 8);
const BB_91: u128 = 1 << ((1 - 1) * 9 + 9 - 9);
const BB_92: u128 = 1 << ((2 - 1) * 9 + 9 - 9);
const BB_93: u128 = 1 << ((3 - 1) * 9 + 9 - 9);
const BB_94: u128 = 1 << ((4 - 1) * 9 + 9 - 9);
const BB_95: u128 = 1 << ((5 - 1) * 9 + 9 - 9);
const BB_96: u128 = 1 << ((6 - 1) * 9 + 9 - 9);
const BB_97: u128 = 1 << ((7 - 1) * 9 + 9 - 9);
const BB_98: u128 = 1 << ((8 - 1) * 9 + 9 - 9);
const BB_99: u128 = 1 << ((9 - 1) * 9 + 9 - 9);

const BB_RANK_1: u128 = BB_11 | BB_21 | BB_31 | BB_41 | BB_51 | BB_61 | BB_71 | BB_81 | BB_91;
const BB_RANK_2: u128 = BB_12 | BB_22 | BB_32 | BB_42 | BB_52 | BB_62 | BB_72 | BB_82 | BB_92;
const BB_RANK_3: u128 = BB_13 | BB_23 | BB_33 | BB_43 | BB_53 | BB_63 | BB_73 | BB_83 | BB_93;
const BB_RANK_4: u128 = BB_14 | BB_24 | BB_34 | BB_44 | BB_54 | BB_64 | BB_74 | BB_84 | BB_94;
const BB_RANK_5: u128 = BB_15 | BB_25 | BB_35 | BB_45 | BB_55 | BB_65 | BB_75 | BB_85 | BB_95;
const BB_RANK_6: u128 = BB_16 | BB_26 | BB_36 | BB_46 | BB_56 | BB_66 | BB_76 | BB_86 | BB_96;
const BB_RANK_7: u128 = BB_17 | BB_27 | BB_37 | BB_47 | BB_57 | BB_67 | BB_77 | BB_87 | BB_97;
const BB_RANK_8: u128 = BB_18 | BB_28 | BB_38 | BB_48 | BB_58 | BB_68 | BB_78 | BB_88 | BB_98;
const BB_RANK_9: u128 = BB_19 | BB_29 | BB_39 | BB_49 | BB_59 | BB_69 | BB_79 | BB_89 | BB_99;

pub struct Board {
    piece_bb: [Bitboard; 15],
    pieces_in_hand: [[u8; 15]; 2],
    occupied: [Bitboard; 2],
    black_turn: bool,
}

impl Board {
    pub fn new() -> Self {
        let piece_bb = [
            Bitboard(BB_VOID),                       // NONE
            Bitboard(BB_RANK_3 | BB_RANK_7),         // PAWN
            Bitboard(BB_11 | BB_91 | BB_19 | BB_99), // LANCE
            Bitboard(BB_21 | BB_81 | BB_29 | BB_89), // KNIGHT
            Bitboard(BB_31 | BB_71 | BB_39 | BB_79), // SILVER
            Bitboard(BB_41 | BB_61 | BB_49 | BB_69), // GOLD
            Bitboard(BB_22 | BB_88),                 // BISHOP
            Bitboard(BB_28 | BB_82),                 // ROOK
            Bitboard(BB_51 | BB_59),                 // KING
            Bitboard(BB_VOID),                       // PROM_PAWN
            Bitboard(BB_VOID),                       // PROM_LANCE
            Bitboard(BB_VOID),                       // PROM_KNIGHT
            Bitboard(BB_VOID),                       // PROM_SILVER
            Bitboard(BB_VOID),                       // PROM_BISHOP
            Bitboard(BB_VOID),                       // PROM_ROOK
        ];
        let occupied = [
            Bitboard(BB_RANK_9 | BB_88 | BB_28 | BB_RANK_7),
            Bitboard(BB_RANK_3 | BB_82 | BB_22 | BB_RANK_1),
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

pub struct Bitboard(u128);

impl Bitboard {
    pub fn push_move(&mut self, mv: Move) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_init_board() {
        let board = Board::new();
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
}
