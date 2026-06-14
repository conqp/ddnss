use std::collections::BTreeMap;

use config::{Config, ConfigError, File};

use crate::host::{Host, Settings};

const FILE_NAME: &str = "/etc/ddnss";

/// Load the config file contents.
pub fn load() -> Result<Vec<Host>, ConfigError> {
    Config::builder()
        .add_source(File::with_name(FILE_NAME))
        .build()?
        .try_deserialize::<BTreeMap<String, Settings>>()
        .map(|map| map.into_iter().map(Into::into).collect())
}
