use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Parsing Error: {0}")]
    ParseError(String),
    #[error("Serialization Error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("CSV Error: {0}")]
    CsvError(#[from] csv::Error),
}
