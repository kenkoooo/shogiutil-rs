use thiserror::Error;

pub type Result<T> = std::result::Result<T, ShogiUtilError>;

#[derive(Error, Debug)]
pub enum ShogiUtilError {
    #[error("io error")]
    IOError(#[from] std::io::Error),

    #[error("CSA parse error: {0}")]
    CsaParseError(String),
}
