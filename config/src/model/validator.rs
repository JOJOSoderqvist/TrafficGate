use crate::errors::ValidationError;
use crate::manager::manager::ValidateConfig;
use crate::model::config::TrafficGateConfig;
use crate::model::raw::RawConfig;

pub struct ConfigValidator;

impl ValidateConfig for ConfigValidator {
    fn validate_config(raw_cfg: RawConfig) -> Result<TrafficGateConfig, ValidationError> {
        todo!()
    }
}