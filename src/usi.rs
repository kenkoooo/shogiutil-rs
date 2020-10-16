use crate::error::ShogiUtilError::UsiParseError;
use crate::{Board, Color, Move, Piece};
use crate::{Result, Square};

pub struct SfenBoard {
    pub board: Board,
    pub next_turn: Color,
}

impl SfenBoard {
    pub fn parse(sfen_state: &str) -> Result<Self> {
        let e = || UsiParseError(format!("Invalid sfen format: {}", sfen_state));
        let sfen_parts = sfen_state.split(' ').collect::<Vec<_>>();
        if sfen_parts.len() != 4 {
            return Err(e());
        }

        let rows = sfen_parts[0].split('/').collect::<Vec<_>>();
        if rows.len() != 9 {
            return Err(e());
        }

        let mut board = Board::empty();
        for (rank, row) in rows.into_iter().enumerate() {
            let rank = (rank + 1) as u8;
            let mut row = row.chars();
            let mut file = 9;
            while let Some(c) = row.next() {
                if c.is_numeric() {
                    let c = c as u8 - '0' as u8;
                    file -= c - 1;
                } else if c == '+' {
                    let c = row.next().ok_or_else(e)?;
                    let (piece, color) = parse_piece(c)
                        .and_then(|(p, c)| p.promote().map(|p| (p, c)))
                        .ok_or_else(e)?;
                    let pos = Square { rank, file };
                    board.push_piece(&pos, color, piece);
                } else if let Some((piece, color)) = parse_piece(c) {
                    let pos = Square { rank, file };
                    board.push_piece(&pos, color, piece);
                } else {
                    return Err(e());
                }
                file -= 1;
            }
            if file != 0 {
                return Err(e());
            }
        }

        let next_turn;
        match sfen_parts[1] {
            "b" => next_turn = Color::Black,
            "w" => next_turn = Color::White,
            _ => return Err(e()),
        }

        if sfen_parts[2] != "-" {
            let hands = parse_hand(sfen_parts[2]).ok_or_else(e)?;
            for &piece in hands[0].iter() {
                board.push_hand(piece, Color::Black);
            }
            for &piece in hands[1].iter() {
                board.push_hand(piece, Color::White);
            }
        }

        Ok(SfenBoard { board, next_turn })
    }
}

fn parse_hand(hand: &str) -> Option<[Vec<Piece>; 2]> {
    let mut hand = hand.chars();
    let mut stack = String::new();
    let mut result = [vec![], vec![]];
    while let Some(c) = hand.next() {
        if c.is_numeric() {
            stack.push(c);
        } else {
            let (piece, color) = parse_piece(c)?;
            let count = if stack.is_empty() {
                Some(1)
            } else {
                stack.parse::<usize>().ok()
            }?;
            for _ in 0..count {
                result[color.to_usize()].push(piece);
            }
            stack.clear();
        }
    }
    Some(result)
}

fn parse_piece(c: char) -> Option<(Piece, Color)> {
    let color = if c.is_ascii_uppercase() {
        Color::Black
    } else {
        Color::White
    };
    match c.to_ascii_uppercase() {
        'K' => Some((Piece::King, color)),
        'R' => Some((Piece::Rook, color)),
        'B' => Some((Piece::Bishop, color)),
        'G' => Some((Piece::Gold, color)),
        'S' => Some((Piece::Silver, color)),
        'N' => Some((Piece::Knight, color)),
        'L' => Some((Piece::Lance, color)),
        'P' => Some((Piece::Pawn, color)),
        _ => None,
    }
}

pub enum SfenMove {
    DropMove {
        to: Square,
        piece: Piece,
    },
    Travel {
        from: Square,
        to: Square,
        promoted: bool,
    },
}

impl SfenMove {
    pub fn parse(sfen_move: &str) -> Result<SfenMove> {
        let e = || UsiParseError(format!("Invalid sfen move: '{}'", sfen_move));
        if sfen_move.len() < 4 || 5 < sfen_move.len() {
            return Err(e());
        }

        let from = parse_sfen_square(&sfen_move[0..2]);
        let to = parse_sfen_square(&sfen_move[2..4]).ok_or_else(e)?;

        if let Some(from) = from {
            if sfen_move.len() == 5 {
                if &sfen_move[4..5] != "+" {
                    Err(e())
                } else {
                    Ok(SfenMove::Travel {
                        from,
                        to,
                        promoted: true,
                    })
                }
            } else {
                Ok(SfenMove::Travel {
                    from,
                    to,
                    promoted: false,
                })
            }
        } else if &sfen_move[1..2] != "*" {
            Err(e())
        } else {
            let c = sfen_move.chars().next().unwrap();
            let (piece, color) = parse_piece(c).ok_or_else(e)?;
            if color != Color::Black {
                Err(e())
            } else {
                Ok(SfenMove::DropMove { piece, to })
            }
        }
    }
}

fn parse_sfen_square(s: &str) -> Option<Square> {
    assert_eq!(s.len(), 2);
    let mut iter = s.chars();
    let file = iter.next().unwrap();
    let rank = iter.next().unwrap();
    if file < '0' || rank < 'a' {
        return None;
    }
    let file = file as u8 - '0' as u8;
    let rank = rank as u8 - 'a' as u8 + 1;
    if rank > 9 || file > 9 {
        None
    } else {
        Some(Square { rank, file })
    }
}

pub enum UsiRequest {
    Usi,
    IsReady,
    SetOption { id: String, value: String },
    NewGame,
    Position { board: Board, next_turn: Color },
    Go,
    Quit,
}

impl UsiRequest {
    pub fn parse(input: &str) -> Result<UsiRequest> {
        let command = input.split(' ').collect::<Vec<_>>();
        match command[0].trim() {
            "usi" => Ok(UsiRequest::Usi),
            "isready" => Ok(UsiRequest::IsReady),
            "setoption" => {
                if command[1] != "name" {
                    Err(UsiParseError(format!("Invalid command: {}", input)))
                } else {
                    Ok(UsiRequest::SetOption {
                        id: command[2].to_string(),
                        value: command[4].to_string(),
                    })
                }
            }
            "usinewgame" => Ok(UsiRequest::NewGame),
            "position" => match command[1] {
                "sfen" => {
                    let board_sfen = command[2];
                    let next_turn = command[3];
                    let hand_sfen = command[4];
                    let sfen_string = vec![board_sfen, next_turn, hand_sfen, "1"].join(" ");
                    let sfen_board = SfenBoard::parse(&sfen_string)?;
                    let cur_turn = sfen_board.next_turn;
                    let mut board = sfen_board.board;
                    if command[6] != "moves" {
                        return Err(UsiParseError(format!("Invalid command: {}", input)));
                    }
                    let cur_turn = push_move_commands(&mut board, &command[7..], cur_turn)?;
                    Ok(UsiRequest::Position {
                        board,
                        next_turn: cur_turn,
                    })
                }
                "startpos" => {
                    if command.len() != 2 && command[2] != "moves" {
                        return Err(UsiParseError(format!("Invalid command: {}", input)));
                    }
                    let mut cur_turn = Color::Black;
                    let mut board = Board::default();

                    if command.len() >= 4 {
                        cur_turn = push_move_commands(&mut board, &command[3..], cur_turn)?;
                    }

                    Ok(UsiRequest::Position {
                        board,
                        next_turn: cur_turn,
                    })
                }
                _ => Err(UsiParseError(format!("Invalid format: {}", input))),
            },
            "go" => Ok(UsiRequest::Go),
            "quit" => Ok(UsiRequest::Quit),
            _ => Err(UsiParseError(format!("Unsupported option: {}", input))),
        }
    }
}

fn push_move_commands(board: &mut Board, command: &[&str], mut cur_turn: Color) -> Result<Color> {
    for &command in command.iter() {
        match SfenMove::parse(command)? {
            SfenMove::DropMove { to, piece } => {
                board.push_move(Move {
                    from: None,
                    to,
                    piece,
                    color: cur_turn,
                })?;
            }
            SfenMove::Travel { from, to, promoted } => {
                board.move_between(&from, &to, promoted, cur_turn)?;
            }
        }

        cur_turn = cur_turn.opponent();
    }
    Ok(cur_turn)
}

pub enum UsiResponse {
    Id {
        name: String,
    },
    UsiOk,
    ReadyOk,
    TravelMove {
        from: Square,
        to: Square,
        promoted: bool,
    },
    DropMove {
        piece: Piece,
        to: Square,
    },
}

impl ToString for UsiResponse {
    fn to_string(&self) -> String {
        use UsiResponse::*;
        match self {
            Id { name } => format!("id name {}", name),
            UsiOk => "usiok".to_string(),
            ReadyOk => "readyok".to_string(),
            TravelMove { from, to, promoted } => {
                let mut response = "bestmove ".to_string();
                to_sfen_square(&from, &mut response);
                to_sfen_square(&to, &mut response);
                if *promoted {
                    response.push('+');
                }
                response
            }
            DropMove { piece, to } => {
                let mut response = "bestmove ".to_string();
                response.push(piece.to_sfen());
                response.push('*');
                to_sfen_square(&to, &mut response);
                response
            }
        }
    }
}

fn to_sfen_square(sq: &Square, s: &mut String) {
    s.push((sq.file + '0' as u8) as char);
    let rank = (sq.rank - 1 + 'a' as u8) as char;
    s.push(rank);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::dump_board;

    #[test]
    fn test_parse_hand() {
        let result = parse_hand("S2Pb3p").unwrap();
        assert_eq!(
            result,
            [
                vec![Piece::Silver, Piece::Pawn, Piece::Pawn],
                vec![Piece::Bishop, Piece::Pawn, Piece::Pawn, Piece::Pawn]
            ]
        );
    }

    #[test]
    fn test_parse_board() {
        let board =
            SfenBoard::parse("lnsgkgsn1/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL w - 1")
                .unwrap()
                .board;
        assert_eq!(
            dump_board(&board),
            r"P1-KY-KE-GI-KI-OU-KI-GI-KE * 
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
"
        );
    }

    #[test]
    fn test_parse_sfen_move() {
        if let SfenMove::Travel { from, to, promoted } = SfenMove::parse("8h2b+").unwrap() {
            assert!(promoted);
            assert_eq!(from, Square { file: 8, rank: 8 });
            assert_eq!(to, Square { file: 2, rank: 2 });
        } else {
            unreachable!()
        }

        if let SfenMove::Travel { from, to, promoted } = SfenMove::parse("7g7f").unwrap() {
            assert!(!promoted);
            assert_eq!(from, Square { file: 7, rank: 7 });
            assert_eq!(to, Square { file: 7, rank: 6 });
        } else {
            unreachable!()
        }

        if let SfenMove::DropMove { to, piece } = SfenMove::parse("S*5b").unwrap() {
            assert_eq!(piece, Piece::Silver);
            assert_eq!(to, Square { file: 5, rank: 2 });
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_parse_usi_board() {
        let input = "position startpos moves 7g7f 3c3d 2g2f 8c8d 8g8f 2c2d 2f2e 8d8e 2e2d 8e8f 2h2f 8b8d 2f2e 8d8e 3g3f 7c7d 6g6f 4c4d 4g4f 6c6d 5g5f 5c5d 3f3e 7d7e 7f7e 3d3e 4f4e 6d6e 6f6e 4d4e 3i3h 7a7b 5f5e 5d5e 7e7d 3e3f 7i7h 3a3b 7h7g 3b3c 7g7f 3c3d 7f7e 3d3e 6e6d 5e5f 3h3g 4e4f 3g3f 3e3f 5i5h 7b7c 4i4h 7c7d 4h4g 7d7e 4g4f 7e7f 4f4e 5a5b 4e4d";
        match UsiRequest::parse(input).unwrap() {
            UsiRequest::Position { board, next_turn } => {
                assert_eq!(next_turn, Color::White);
                assert_eq!(
                    r"P1-KY-KE * -KI * -KI * -KE-KY
P2 *  *  *  * -OU *  * -KA * 
P3-FU *  *  *  *  *  *  * -FU
P4 *  *  * +FU * +KI * +FU * 
P5 * -HI *  *  *  *  * +HI * 
P6 * -FU-GI * -FU * -GI *  * 
P7+FU *  *  *  *  *  *  * +FU
P8 * +KA *  * +OU *  *  *  * 
P9+KY+KE * +KI *  *  * +KE+KY
P+00FU00FU00FU00FU00FU
P-00FU00FU00FU00FU00FU00GI00GI
",
                    dump_board(&board)
                );
            }
            _ => unreachable!(),
        };
    }
}
