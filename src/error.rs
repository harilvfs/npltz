use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NpltzError {
    #[error("{0}")]
    InvalidDate(String),
    #[error("Config: {0}")]
    Config(String),
    #[error("{0}")]
    InvalidTheme(String),
    #[error(transparent)]
    Io(#[from] io::Error),
}

pub type Result<T> = std::result::Result<T, NpltzError>;
