use crate::{decoders::errors::DecoderError, utils::errors::FileProcessingError};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    DecoderError(DecoderError),
    FileProcessingError(FileProcessingError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DecoderError(err) => write!(f, "Decoder error: {}", err),
            Error::FileProcessingError(err) => write!(f, "File processing error: {}", err),
        }
    }
}

impl From<DecoderError> for Error {
    fn from(err: DecoderError) -> Error {
        Error::DecoderError(err)
    }
}

impl From<FileProcessingError> for Error {
    fn from(err: FileProcessingError) -> Error {
        Error::FileProcessingError(err)
    }
}