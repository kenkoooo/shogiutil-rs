use thiserror::Error;

pub type Result<T> = std::result::Result<T, ShogiUtilError>;

#[derive(Error, Debug)]
pub enum ShogiUtilError {
    #[error("io error")]
    IOError(#[from] std::io::Error),

    #[error("CSA parse error: {0}")]
    CsaParseError(String),

    #[error("USI parse error: {0}")]
    UsiParseError(String),

    #[error("Invalid move: {0}")]
    InvalidMove(String),
}
