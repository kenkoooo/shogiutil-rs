use crate::model::LegalMove;
use crate::{Bitboard, Board, Color, Move, Piece, Square};

pub(crate) fn generate_legal_moves(board: &Board) -> Vec<LegalMove> {
    let mut moves = vec![];
    for (piece_id, &piece_bb) in board.piece_bb.iter().enumerate() {
        let piece = Piece::from(piece_id as u8);
        let bb = piece_bb & board.occupied[0];
        for pos in bb.iter() {
            let pos_i = pos / 9;
            let pos_j = pos % 9;
            let from = Square::from_pos(pos_i as usize, pos_j as usize);
            let mut destinations = vec![];
            piece.generate_moves(&from, &board.occupied, &mut destinations);
            for to in destinations {
                if from.rank <= 3 || to.rank <= 3 {
                    if let Some(piece) = piece.promote() {
                        moves.push(LegalMove {
                            mv: Move {
                                color: Color::Black,
                                from: Some(from.clone()),
                                to: to.clone(),
                                piece,
                            },
                            promoted: true,
                        });
                    }
                }
                moves.push(LegalMove {
                    mv: Move {
                        color: Color::Black,
                        from: Some(from.clone()),
                        to,
                        piece,
                    },
                    promoted: false,
                });
            }
        }
    }

    let occupied = board.occupied[0] | board.occupied[1];
    let unoccupied = Bitboard::full() ^ occupied;
    for (piece_id, &count) in board.pieces_in_hand[0].iter().enumerate() {
        if count == 0 {
            continue;
        }
        let piece = Piece::from(piece_id as u8);
        for pos in unoccupied.iter() {
            let pos_i = pos / 9;
            let pos_j = pos % 9;
            let to = Square::from_pos(pos_i as usize, pos_j as usize);
            if piece == Piece::Pawn && board.piece_bb[piece_id].file_count_ones(to.file) > 0 {
                continue;
            }
            moves.push(LegalMove {
                mv: Move {
                    color: Color::Black,
                    from: None,
                    to,
                    piece,
                },
                promoted: false,
            })
        }
    }
    moves
}
