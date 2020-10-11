use crate::Square;
use std::ops::{BitAnd, BitOr, BitXor};

#[derive(Clone, Copy)]
pub struct Bitboard(pub u128);

impl BitAnd for Bitboard {
    type Output = Bitboard;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Bitboard {
    pub const fn empty() -> Self {
        Self(0)
    }
    pub const fn full() -> Self {
        Self(0b_111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111_111111111)
    }

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

    pub fn iter(&self) -> impl Iterator<Item = u32> {
        BitIterator(self.0)
    }

    pub fn file_count_ones(&self, file: u8) -> u32 {
        let file_only = super::bit_file(file) & self.0;
        file_only.count_ones()
    }
}

struct BitIterator(u128);
impl Iterator for BitIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let pos = self.0.trailing_zeros();
            self.0 ^= 1 << pos;
            Some(pos)
        }
    }
}

fn sq_to_pos(sq: &Square) -> u8 {
    (sq.rank - 1) * 9 + 9 - sq.file
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate180() {
        let b = Bitboard(0b_111111111_100000000_100000000_100000000_100000000_100000000_100000000_100000000_100000000);
        let rotated = b.rotate180();
        assert_eq!(rotated.0, 0b_000000001_000000001_000000001_000000001_000000001_000000001_000000001_000000001_111111111);
    }

    #[test]
    fn test_bit_iterator() {
        let v = Bitboard(0b1000100101).iter().collect::<Vec<_>>();
        assert_eq!(v, [0, 2, 5, 9]);
    }

    #[test]
    fn test_full_bit() {
        let full = Bitboard::full();
        assert_eq!(81, full.0.count_ones());
    }
}
