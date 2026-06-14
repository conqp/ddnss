use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Result};

use crate::host::{Host, Settings};

const FILE_NAME: &str = "/etc/ddnss.toml";

/// Load the config file contents.
pub fn load() -> Result<Vec<Host>> {
    let mut text = String::new();
    OpenOptions::new()
        .read(true)
        .open(FILE_NAME)?
        .read_to_string(&mut text)?;
    toml::from_str::<BTreeMap<String, Settings>>(&text)
        .map(|map| map.into_iter().map(Into::into).collect())
        .map_err(Error::other)
}
