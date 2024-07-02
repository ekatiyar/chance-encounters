use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum FileProcessingError {
    InvalidPathError(String),
    MissingFileError,
    FileReaderError(String),
    InProcessError,
}

impl FileProcessingError {
    pub fn is_processing(&self) -> bool {
        match self {
            FileProcessingError::InProcessError => true,
            _ => false,
        }
    }
}

impl fmt::Display for FileProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileProcessingError::InvalidPathError(path) => write!(f, "Invalid path: {}", path),
            FileProcessingError::MissingFileError => write!(f, "Please provide both files"),
            FileProcessingError::FileReaderError(msg) => write!(f, "{}", msg),
            FileProcessingError::InProcessError => write!(f, "File is still being processed"),
        }
    }
}