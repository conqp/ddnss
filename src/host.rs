use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::ip_protocol::IpProtocol;
use crate::parse_response::parse_response;

/// Information about a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Host {
    name: String,
    settings: Settings,
}

impl Host {
    /// Update this host.
    ///
    /// # Error
    ///
    /// Returns a [`reqwest::Error`] if the HTTP request fails.
    pub async fn update(&self) -> reqwest::Result<Option<usize>> {
        let mut client = Client::new().get(self.url());

        if let Some(timeout_secs) = self.settings.timeout_secs {
            client = client.timeout(Duration::from_secs(timeout_secs));
        }

        client
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(parse_response)
    }

    /// Return the update URL.
    fn url(&self) -> Url {
        let mut url: Url = self.settings.protocol.into();
        url.set_query(Some(&format!(
            "hostname={}&key={}",
            self.name, self.settings.key
        )));
        url
    }
}

impl From<(String, Settings)> for Host {
    fn from((name, settings): (String, Settings)) -> Self {
        Self { name, settings }
    }
}

/// Settings of a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Settings {
    key: String,
    #[serde(default)]
    protocol: IpProtocol,
    #[serde(default)]
    timeout_secs: Option<u64>,
}
