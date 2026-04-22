use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatlineError {
    #[error("Input path does not exist: {0}")]
    InputNotFound(PathBuf),

    #[error("Input path is not a file: {0}")]
    InputNotAFile(PathBuf),

    #[error("Unsupported input format for file: {0}")]
    UnsupportedFormat(PathBuf),

    #[error("Dataset is empty after loading")]
    EmptyDataset,

    #[error("{0}")]
    Message(String),

    #[error(transparent)]
    Polars(#[from] polars::error::PolarsError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, StatlineError>;
