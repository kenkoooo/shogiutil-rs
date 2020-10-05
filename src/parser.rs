use crate::error::ShogiUtilError::CsaParseError;
use crate::piece::Piece;
use crate::{Color, Move, Result, ShogiUtilError, Square};
use std::fs::read_to_string;
use std::path::Path;
use std::str::FromStr;

const COLOR_SYMBOLS: [&str; 2] = ["+", "-"];

pub fn parse_csa_file<P: AsRef<Path>>(csa_filepath: P) -> Result<()> {
    let content = read_to_string(csa_filepath)?;
    parse_csa_string(&content)?;
    Ok(())
}

pub fn parse_csa_string(csa_str: &str) -> Result<()> {
    let mut names = [None, None];
    let mut position_lines = vec![];
    let mut current_turn = None;
    for (line_number, line) in csa_str.split("\n").enumerate() {
        if line.is_empty() {
            //ignore
        } else {
            match &line[..1] {
                "'" | "V" | "$" | "T" => {
                    //ignore
                }
                "N" => match &line[1..2] {
                    "+" => names[0] = Some(&line[2..]),
                    "-" => names[1] = Some(&line[2..]),
                    _ => {
                        return Err(ShogiUtilError::CsaParseError(format!(
                            "Unknown symbol {}: {}",
                            line_number + 1,
                            line
                        )));
                    }
                },
                "P" => position_lines.push(line),
                "+" | "-" => {
                    if line.len() == 1 {
                        current_turn = Some(line);
                    } else {
                        let mv = parse_move_str(line)?;
                    }
                }
                "%" => {
                    // end of game
                    match line {
                        "%TORYO" | "%TIME_UP" | "%ILLEGAL_MOVE" => {}
                        "%+ILLEGAL_ACTION" => {}
                        "%-ILLEGAL_ACTION" => {}
                        _ => {
                            return Err(ShogiUtilError::CsaParseError(format!(
                                "{} is not supported.",
                                line
                            )));
                        }
                    }
                }
                "/" => {
                    return Err(ShogiUtilError::CsaParseError(
                        "Separator is not supported.".to_string(),
                    ));
                }
                _ => {
                    return Err(ShogiUtilError::CsaParseError(format!(
                        "Invalid line {}: {}",
                        line_number + 1,
                        line
                    )));
                }
            }
        }
    }
    Ok(())
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
    #[test]
    fn test_parse_file() {
        parse_csa_file("../super-duper-dragon/floodgate-kifu/wdoor2016/2016/wdoor+floodgate-600-10F+MAJO_6700HQ_160917+SILENT_HIRAOKA_20160930_2700K+20161004190005.csa").unwrap();
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
