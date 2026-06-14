use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Result};
use std::time::Duration;

use serde::Deserialize;

use crate::ip_protocol::IpProtocol;
use crate::update::Update;

/// Settings of a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
pub struct Settings {
    key: String,
    hosts: BTreeMap<IpProtocol, Vec<String>>,
    #[serde(default)]
    timeout_secs: Option<u64>,
}

impl Settings {
    const FILE_NAME: &str = "/etc/ddnss.json";

    /// Load the config file contents.
    pub fn load() -> Result<Self> {
        let mut text = String::new();
        OpenOptions::new()
            .read(true)
            .open(Self::FILE_NAME)?
            .read_to_string(&mut text)?;
        serde_json::from_str(&text).map_err(Error::other)
    }

    /// Iterate over all updates.
    pub fn updates(self) -> impl Iterator<Item = Update> {
        self.hosts.into_iter().map(move |(protocol, hosts)| {
            Update::new(
                protocol,
                self.key.clone(),
                self.timeout_secs.map(Duration::from_secs),
                hosts,
            )
        })
    }
}
