use crate::errors::{ConfigManagerError, ParseError, StorageError, ValidationError};
use crate::model::config::TrafficGateConfig;
use crate::model::raw::RawConfig;
use arc_swap::ArcSwap;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub(crate) trait ConfigRepository: Send + Sync {
    async fn read_cfg(&self) -> Result<String, StorageError>;
    async fn write_cfg(&self, raw_cfg: &str) -> Result<(), StorageError>;
}

pub(crate) trait ValidateConfig: Send + Sync {
    fn parse_raw(&self, unparsed_cfg: &str) -> Result<RawConfig, ParseError>;
    fn validate_config(&self, raw_cfg: RawConfig) -> Result<TrafficGateConfig, ValidationError>;
}

pub(crate) struct ConfigManager<S, V>
where
    S: ConfigRepository,
    V: ValidateConfig,
{
    storage: S,
    validator: V,
    current_cfg: ArcSwap<TrafficGateConfig>,
    tx: tokio::sync::watch::Sender<Arc<TrafficGateConfig>>,
    update_lock: Mutex<()>,
}

impl<S, V> ConfigManager<S, V>
where
    S: ConfigRepository,
    V: ValidateConfig,
{
    pub async fn new(storage: S, validator: V) -> Result<Self, ConfigManagerError> {
        let initial_cfg = storage.read_cfg().await?;
        let raw_cfg = validator.parse_raw(initial_cfg.as_str())?;
        let clean_cfg = Arc::new(validator.validate_config(raw_cfg)?);
        let tx = tokio::sync::watch::Sender::new(clean_cfg.clone());
        let current_cfg = ArcSwap::new(clean_cfg.clone());

        Ok(Self {
            storage,
            validator,
            current_cfg,
            tx,
            update_lock: Mutex::default(),
        })
    }

    pub fn get_cfg(&self) -> Arc<TrafficGateConfig> {
        self.current_cfg.load().clone()
    }

    pub fn subscribe(&self) -> tokio::sync::watch::Receiver<Arc<TrafficGateConfig>> {
        self.tx.subscribe()
    }

    pub async fn update_cfg(&self, req_cfg: &str) -> Result<(), ConfigManagerError> {
        let _guard = self.update_lock.lock();
        let raw_cfg = self.validator.parse_raw(req_cfg)?;
        let clean_cfg = Arc::new(self.validator.validate_config(raw_cfg)?);
        self.storage.write_cfg(req_cfg).await?;
        self.current_cfg.store(clean_cfg.clone());
        self.tx.send_replace(clean_cfg);
        Ok(())
    }
}
