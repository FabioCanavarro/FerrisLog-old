use std::fmt;
use std::error::Error;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub enum KvError {
    WriteError,
    ReadError,
    OpenError,
    ParseError,
}

impl fmt::Display for KvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KvError::WriteError => writeln!(f, "Writing has failed!"),
            KvError::ReadError => Ok(()),
            KvError::OpenError => writeln!(f, "Opening has failed!"),
            KvError::ParseError => writeln!(f, "Parsing has failed!"),
        }
    }
}

impl Error for KvError {}


pub type KvResult<T> = Result<T, KvError>;
