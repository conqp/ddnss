use config::{Config, ConfigError, File};

use crate::host::Host;

const FILE_NAME: &str = "/etc/ddnss";

/// Load the config file contents.
pub fn load() -> Result<Vec<Host>, ConfigError> {
    Config::builder()
        .add_source(File::with_name(FILE_NAME))
        .build()?
        .try_deserialize::<Vec<Host>>()
}
