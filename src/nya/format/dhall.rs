use miau::configuration::ConfigurationTree;
use miau::error::{ConfigurationError, ErrorCode};
use miau::format::Format;

pub struct Dhall {}

impl Dhall {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Dhall {
    fn default() -> Self {
        Self::new()
    }
}

impl Format for Dhall {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        serde_dhall::from_str(str_input.as_str()).parse::<ConfigurationTree>()
                                                 .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "dhall".into()
    }
}