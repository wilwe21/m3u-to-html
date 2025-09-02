use std::io::{self, Read, Write};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] io::Error),
    #[error("Unexpected curly brace found.")]
    UnexpectedCurlyBrace,
    #[error("Unknown variable: {0}")]
    UnknownVariable(String),
    #[error("Unknown RGB Value: {0}")]
    UnknownRgb(String),
}
