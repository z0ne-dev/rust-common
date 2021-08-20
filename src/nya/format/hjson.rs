use miau::configuration::ConfigurationTree;
use miau::error::{ConfigurationError, ErrorCode};
use miau::format::Format;

pub struct Hjson {}

impl Hjson {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Hjson {
    fn default() -> Self {
        Self::new()
    }
}

impl Format for Hjson {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        deser_hjson::from_str::<ConfigurationTree>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "hjson".into()
    }
}