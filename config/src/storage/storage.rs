use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::sync::Mutex;
use crate::errors::FileError;
use crate::errors::FileError::{FailedToOpenFile, FailedToReadFileContent, FailedToSetSeek};

pub(crate) struct ConfigStorage {
    storage: Mutex<File>,
}

impl ConfigStorage {
    pub(crate) fn new(filename: &str) -> Result<Self, FileError> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(filename)
            .map_err(FailedToOpenFile)?;

        Ok(ConfigStorage{
            storage: Mutex::new(file)
        })
    }
}
