use chrono::ParseError;
use serde::{Serialize, Deserialize};
use std::{fmt, num::ParseIntError};

# [derive(Debug, Serialize,Deserialize, Clone)]
pub enum DecoderError {
    DeserializeError(String),
    EmptyEntryError(String),
    TimeParseError(String),
    GeoParseError(String),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DecoderError::DeserializeError(msg) => write!(f, "Deserialize Error: {}", msg),
            DecoderError::EmptyEntryError(msg) => write!(f, "Empty Entry Error: {}", msg),
            DecoderError::TimeParseError(msg) => write!(f, "UTC Parsing Error: {}", msg),
            DecoderError::GeoParseError(msg) => write!(f, "Geo Parse Error: {}", msg),
        }
    }
}

impl From<serde_json::Error> for DecoderError {
    fn from(err: serde_json::Error) -> Self {
        DecoderError::DeserializeError(format!("Failed to deserialize json {:#?} at line, col {},{}", err.classify(), err.line(),err.column()))
    }
}

impl From<ParseError> for DecoderError {
    fn from(err: ParseError) -> Self {
        DecoderError::TimeParseError(err.to_string())
    }
}

impl From<ParseIntError> for DecoderError {
    fn from(err: ParseIntError) -> Self {
        DecoderError::TimeParseError(err.to_string())
    }
}