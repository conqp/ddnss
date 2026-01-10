use std::collections::BTreeMap;

use config::{Config, ConfigError, File};

const FILE_NAME: &str = "ddnss.conf";

/// Load the config file contents.
pub fn load() -> Result<BTreeMap<String, String>, ConfigError> {
    Config::builder()
        .add_source(File::with_name(FILE_NAME))
        .build()?
        .try_deserialize::<BTreeMap<String, String>>()
}
