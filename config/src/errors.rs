use crate::model::config::TrafficGateConfig;
use atomicwrites::Error;
use std::io;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::watch::error::SendError;
use tokio::task::JoinError;

#[derive(Error, Debug)]
pub(crate) enum ConfigManagerError {
    #[error("failed to validate cfg: {0}")]
    ValidationError(#[from] ValidationError),
    #[error("failed to parse cfg: {0}")]
    ParseError(#[from] ParseError),
    #[error("storage error happened: {0}")]
    StorageError(#[from] StorageError),
    #[error("Failed to send cfg through chan: {0}")]
    SendError(#[source] SendError<Arc<TrafficGateConfig>>),
}

#[derive(Error, Debug)]
pub(crate) enum ValidationError {
    #[error("validation error happened {0}")]
    FailedToValidateConfig(String),
}

#[derive(Error, Debug)]
pub(crate) enum ParseError {
    #[error("validation error happened {0}")]
    FailedToParseConfig(String),
}
#[derive(Error, Debug)]
pub(crate) enum StorageError {
    #[error("failed to open file {0}")]
    FailedToOpenFile(#[from] io::Error),
    #[error("failed to acquire mutex: {0}")]
    FailedToAcquireMutex(String),
    #[error("Failed to create file {0}")]
    FailedToCreateFile(#[source] io::Error),
    #[error("Failed to check file for existence {0}")]
    FailedToCheckFileForExistence(#[source] io::Error),
    #[error("Failed to read file content {0}")]
    FailedToReadFileContent(#[source] io::Error),
    #[error("Failed to atomically replace files {0}")]
    FailedToReplaceAtomic(#[source] io::Error),
    #[error("Failed to atomically replace files {0}")]
    FailedToWriteToAtomic(#[source] Error<io::Error>),
    #[error("Failed to join task {0}")]
    FailedToJoinTask(#[source] JoinError),
}
