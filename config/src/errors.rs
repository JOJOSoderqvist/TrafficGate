use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum ConfigError {
    #[error("configuration error happened")]
    ConfigError,
}

#[derive(Error, Debug)]
pub(crate) enum FileError {
    #[error("failed to open file {0}")]
    FailedToOpenFile(#[from] io::Error),
    #[error("failed to acquire mutex: {0}")]
    FailedToAcquireMutex(String),
    #[error("Failed to set seek {0}")]
    FailedToSetSeek(#[source] io::Error),
    #[error("Failed to read file content {0}")]
    FailedToReadFileContent(#[source] io::Error),
}
