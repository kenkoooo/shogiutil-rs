use crate::{Bitboard, Piece, Square};

fn generate_moves(
    from: &Square,
    occupied: &[Bitboard],
    result: &mut Vec<Square>,
    d_rank: i8,
    d_file: i8,
    one: bool,
) {
    let mut next_rank = from.rank as i8 + d_rank;
    let mut next_file = from.file as i8 + d_file;
    while 1 <= next_rank && next_rank <= 9 && 1 <= next_file && next_file <= 9 {
        let next = Square {
            rank: next_rank as u8,
            file: next_file as u8,
        };
        if occupied[0].is_filled(&next) {
            break;
        }
        let done = occupied[1].is_filled(&next);
        result.push(next);
        if done || one {
            break;
        }

        next_rank += d_rank;
        next_file += d_file;
    }
}

const ROOK_DIR: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const BISHOP_DIR: [(i8, i8); 4] = [(1, 1), (-1, 1), (1, -1), (-1, -1)];

pub(super) fn piece_moves(
    piece: Piece,
    from: &Square,
    occupied: &[Bitboard],
    result: &mut Vec<Square>,
) {
    use Piece::*;
    match piece {
        Pawn => generate_moves(from, occupied, result, -1, 0, true),
        Lance => generate_moves(from, occupied, result, -1, 0, false),
        Knight => {
            generate_moves(from, occupied, result, -2, -1, true);
            generate_moves(from, occupied, result, -2, 1, true);
        }
        Silver => {
            for &(d_rank, d_file) in BISHOP_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
            generate_moves(from, occupied, result, -1, 0, true);
        }
        Gold | ProPawn | ProLance | ProKnight | ProSilver => {
            for &(d_rank, d_file) in ROOK_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
            generate_moves(from, occupied, result, -1, -1, true);
            generate_moves(from, occupied, result, -1, 1, true);
        }
        King => {
            for &(d_rank, d_file) in BISHOP_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
            for &(d_rank, d_file) in ROOK_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
        }
        Rook => {
            for &(d_rank, d_file) in ROOK_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, false);
            }
        }
        Bishop => {
            for &(d_rank, d_file) in BISHOP_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, false);
            }
        }
        ProRook => {
            for &(d_rank, d_file) in ROOK_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, false);
            }
            for &(d_rank, d_file) in BISHOP_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
        }
        ProBishop => {
            for &(d_rank, d_file) in BISHOP_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, false);
            }
            for &(d_rank, d_file) in ROOK_DIR.iter() {
                generate_moves(from, occupied, result, d_rank, d_file, true);
            }
        }
        None => {}
    }
}
