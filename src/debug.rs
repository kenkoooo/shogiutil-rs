use crate::board::Board;
use crate::piece::Piece;
use crate::Square;

#[allow(dead_code)]
pub(crate) fn dump_board(board: &Board) -> String {
    let mut p_board = vec![vec![Piece::None; 9]; 9];
    let mut color_board = vec![vec![None; 9]; 9];
    for (piece_id, piece_bb) in board.piece_bb.iter().enumerate() {
        for (color_id, occupied) in board.occupied.iter().enumerate() {
            let player_board = *piece_bb & *occupied;

            for file in 1..=9 {
                for rank in 1..=9 {
                    if player_board.is_filled(&Square { file, rank }) {
                        p_board[rank as usize - 1][9 - file as usize] = Piece::from(piece_id as u8);
                        color_board[rank as usize - 1][9 - file as usize] = Some(color_id);
                    }
                }
            }
        }
    }

    let mut hands = vec![vec![]; 2];
    for (color_id, hand) in board.pieces_in_hand.iter().enumerate() {
        for (piece_id, &count) in hand.iter().enumerate() {
            for _ in 0..count {
                hands[color_id].push(Piece::from(piece_id as u8));
            }
        }
    }

    let mut result = String::new();
    for rank in 0..9 {
        result += &format!("P{}", rank + 1);
        for c in 0..9 {
            if let Some(color) = color_board[rank][c] {
                match color {
                    0 => result += "+",
                    1 => result += "-",
                    _ => unreachable!(),
                }
            } else {
                result += " ";
            }
            result += &p_board[rank][c].to_csa();
        }
        result += "\n";
    }
    for (color, hands) in hands.iter().enumerate() {
        if color == 0 {
            result += "P+";
        } else {
            result += "P-";
        }
        for hand in hands.iter() {
            result += "00";
            result += &hand.to_csa();
        }
        result += "\n";
    }
    result
}
