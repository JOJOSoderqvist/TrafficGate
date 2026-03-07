use std::sync::Arc;
use arc_swap::ArcSwap;
use crate::errors::{StorageError, ValidationError};
use async_trait::async_trait;
use crate::model::config::TrafficGateConfig;
use crate::model::raw::RawConfig;

#[async_trait]
pub(crate) trait ConfigRepository: Send + Sync {
    async fn read_cfg(&self) -> Result<String, StorageError>;
    async fn write_cfg(&self, raw_cfg: &str) -> Result<(), StorageError>;
}

pub(crate) trait ValidateConfig: Send + Sync {
    fn validate_config(raw_cfg: RawConfig) -> Result<TrafficGateConfig, ValidationError>;
}

struct ConfigManager<S, V>
where
    S: ConfigRepository,
    V: ValidateConfig
{
    storage: S,
    validator: V,
    current_cfg: ArcSwap<TrafficGateConfig>,
    tx: tokio::sync::watch::Sender<Arc<TrafficGateConfig>>
}
