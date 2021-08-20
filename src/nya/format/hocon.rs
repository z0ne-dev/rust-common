use miau::configuration::ConfigurationTree;
use miau::error::{ConfigurationError, ErrorCode};
use miau::format::Format;

pub struct Hocon {}

impl Hocon {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for Hocon {
    fn default() -> Self {
        Self::new()
    }
}

impl Format for Hocon {
    fn transform(&self, input: Vec<u8>) -> Result<ConfigurationTree, ConfigurationError> {
        let str_input = String::from_utf8(input).map_err(|e| -> ConfigurationError {
            ErrorCode::DeserializationError(e.to_string()).into()
        })?;

        hocon::de::from_str::<ConfigurationTree>(str_input.as_str())
            .map_err(|e| ErrorCode::DeserializationError(e.to_string()).into())
    }

    fn describe(&self) -> String {
        "hocon".into()
    }
}
