use crate::errors::{ParseError, ValidationError};
use crate::manager::manager::ValidateConfig;
use crate::model::config::TrafficGateConfig;
use crate::model::raw::RawConfig;

pub struct ConfigValidator;

impl ValidateConfig for ConfigValidator {
    fn parse_raw(&self, unparsed_cfg: &str) -> Result<RawConfig, ParseError> {
        todo!()
    }

    fn validate_config(&self, raw_cfg: RawConfig) -> Result<TrafficGateConfig, ValidationError> {
        todo!()
    }
}
