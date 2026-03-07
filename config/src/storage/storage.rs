use crate::errors::StorageError;
use crate::errors::StorageError::{
    FailedToCheckFileForExistence, FailedToCreateFile, FailedToJoinTask, FailedToReadFileContent,
    FailedToWriteToAtomic,
};
use crate::manager::manager::ConfigRepository;
use async_trait::async_trait;
use atomicwrites::{AllowOverwrite, AtomicFile};
use std::io::Write;
use std::path::PathBuf;
use tokio::fs::{File, try_exists};

pub(crate) struct ConfigStorage {
    path: PathBuf,
}

impl ConfigStorage {
    pub(crate) async fn new(filename: &str) -> Result<Self, StorageError> {
        let path = PathBuf::from(filename);

        if !try_exists(&path)
            .await
            .map_err(FailedToCheckFileForExistence)?
        {
            File::options()
                .write(true)
                .create(true)
                .truncate(false)
                .open(&path)
                .await
                .map_err(FailedToCreateFile)?;
        }

        Ok(ConfigStorage { path })
    }
}

#[async_trait]
impl ConfigRepository for ConfigStorage {
    async fn read_cfg(&self) -> Result<String, StorageError> {
        tokio::fs::read_to_string(&self.path)
            .await
            .map_err(FailedToReadFileContent)
    }
    async fn write_cfg(&self, raw_cfg: &str) -> Result<(), StorageError> {
        let path = self.path.clone();
        let content = raw_cfg.to_string();

        tokio::task::spawn_blocking(move || -> Result<(), StorageError> {
            let temp_file = AtomicFile::new(path, AllowOverwrite);
            temp_file
                .write(|f| f.write_all(content.as_bytes()))
                .map_err(FailedToWriteToAtomic)?;
            Ok(())
        })
        .await
        .map_err(FailedToJoinTask)??;

        Ok(())
    }
}
