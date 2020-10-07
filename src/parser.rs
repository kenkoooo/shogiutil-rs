use crate::error::ShogiUtilError::CsaParseError;
use crate::piece::Piece;
use crate::{Color, Move, Result, Square};
use std::str::FromStr;

pub struct ParsedCsa {
    pub names: [Option<String>; 2],
    pub winner: Option<Color>,
    pub moves: Vec<Move>,
}

pub fn parse_csa_string(csa_str: &str) -> Result<ParsedCsa> {
    let mut names = [None, None];
    let mut current_turn = None;
    let mut moves = vec![];
    let mut loser = None;
    for (line_number, line) in csa_str.split('\n').map(|l| l.trim()).enumerate() {
        if line.is_empty() {
            //ignore
        } else {
            match &line[..1] {
                "'" | "V" | "$" | "T" | "P" => {
                    //ignore
                }
                "N" => match &line[1..2] {
                    "+" => names[0] = Some(line[2..].to_string()),
                    "-" => names[1] = Some(line[2..].to_string()),
                    _ => {
                        return Err(CsaParseError(format!(
                            "Unknown symbol {}: {}",
                            line_number + 1,
                            line
                        )));
                    }
                },
                "+" | "-" => {
                    if line.len() == 1 {
                        if current_turn.is_some() {
                            return Err(CsaParseError(String::from(
                                "This file has multiple starting information.",
                            )));
                        }
                        current_turn = Some(Color::from_str(line)?);
                    } else {
                        let mv = parse_move_str(line)?;
                        moves.push(mv);
                        let current = ensure_initialized(current_turn)?;
                        current_turn = Some(current.opponent());
                    }
                }
                "%" => {
                    // end of game
                    match line {
                        "%TORYO" | "%TIME_UP" | "%ILLEGAL_MOVE" => {
                            loser = Some(ensure_initialized(current_turn)?);
                        }
                        "%+ILLEGAL_ACTION" => {
                            loser = Some(Color::Black);
                        }
                        "%-ILLEGAL_ACTION" => {
                            loser = Some(Color::White);
                        }
                        _ => {
                            return Err(CsaParseError(format!("{} is not supported.", line)));
                        }
                    }
                }
                "/" => {
                    return Err(CsaParseError("Separator is not supported.".to_string()));
                }
                _ => {
                    return Err(CsaParseError(format!(
                        "Invalid line {}: {}",
                        line_number + 1,
                        line
                    )));
                }
            }
        }
    }

    let winner = loser.map(|c| c.opponent());
    Ok(ParsedCsa {
        winner,
        names,
        moves,
    })
}

fn ensure_initialized(color: Option<Color>) -> Result<Color> {
    color.ok_or_else(|| CsaParseError("Starting player is not defined".to_string()))
}

fn parse_move_str(move_str: &str) -> Result<Move> {
    let color = Color::from_str(&move_str[..1])?;
    let from = parse_square(&move_str[1..3])?;
    let to = parse_square(&move_str[3..5])?
        .ok_or_else(|| CsaParseError(format!("Invalid destination: {}", move_str)))?;
    let piece = move_str[5..7].parse::<Piece>()?;
    Ok(Move {
        color,
        from,
        to,
        piece,
    })
}

fn parse_square(square_str: &str) -> Result<Option<Square>> {
    if square_str == "00" {
        Ok(None)
    } else {
        let sq = Square::from_str(square_str)?;
        Ok(Some(sq))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::debug::dump_board;
    use crate::Board;

    #[test]
    fn test_parse_csa() {
        let csa_str = r"V2
        N+SUGAI
        N-INABA
        P1-KY-KE-GI-KI-OU-KI-GI-KE-KY
        P2 * -HI *  *  *  *  * -KA * 
        P3-FU-FU-FU-FU-FU-FU-FU-FU-FU
        P4 *  *  *  *  *  *  *  *  * 
        P5 *  *  *  *  *  *  *  *  * 
        P6 *  *  *  *  *  *  *  *  * 
        P7+FU+FU+FU+FU+FU+FU+FU+FU+FU
        P8 * +KA *  *  *  *  * +HI * 
        P9+KY+KE+GI+KI+OU+KI+GI+KE+KY
        +
        +2726FU
        -3334FU
        +7776FU
        -2288KA
        +7988GI
        -2133KE
        +4958KI
        -4142KI
        +2625FU
        -3345KE
        +8877GI
        -9394FU
        +4746FU
        -4537KE
        +2937KE
        -8384FU
        +2524FU
        -2324FU
        +2824HI
        -4233KI
        +2421RY
        -0022KA
        +3745KE
        -5142OU
        +0025KE
        -3332KI
        +0033FU
        -3223KI
        +3332TO
        -3132GI
        +2161RY
        -8272HI
        +0051KA
        -4231OU
        +5133UM
        -3241GI
        +0021KI
        -3121OU
        +6141RY
        -0031KI
        +3343UM
        -2112OU
        +0021GI
        -3121KI
        +4321UM
        %TORYO
        ";
        let result = parse_csa_string(csa_str).unwrap();
        let mut board = Board::default();
        for mv in result.moves {
            board.push_move(mv).unwrap();
        }

        let dumped_board = dump_board(&board);
        assert_eq!(
            r"P1-KY-KE-GI *  * +RY * +UM-KY
P2 *  * -HI *  *  *  * -KA-OU
P3 *  * -FU-FU-FU *  * -KI-FU
P4-FU-FU *  *  *  * -FU *  * 
P5 *  *  *  *  * +KE * +KE * 
P6 *  * +FU *  * +FU *  *  * 
P7+FU+FU+GI+FU+FU *  *  * +FU
P8 *  *  *  * +KI *  *  *  * 
P9+KY+KE * +KI+OU * +GI * +KY
P+00FU00KI
P-00FU00FU00FU00GI
",
            dumped_board
        );
    }

    #[test]
    fn test_parse_square() {
        assert_eq!(
            Some(Square { file: 9, rank: 1 }),
            parse_square("91").unwrap()
        );
        assert_eq!(
            Some(Square { file: 1, rank: 1 }),
            parse_square("11").unwrap()
        );
        assert_eq!(None, parse_square("00").unwrap());
        assert!(parse_square("A1").is_err());
        assert!(parse_square("01").is_err());
        assert!(parse_square("1B").is_err());
    }
}
