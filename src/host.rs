use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::ip_protocol::IpProtocol;
use crate::parse_response::parse_response;

/// Information about a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Host {
    name: String,
    key: String,
    #[serde(default)]
    protocol: IpProtocol,
    #[serde(default)]
    timeout_secs: Option<u64>,
}

impl Host {
    /// Update this host.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if the HTTP request fails.
    pub async fn update(&self) -> reqwest::Result<Option<usize>> {
        let mut client = Client::new().get(self.url());

        if let Some(timeout_secs) = self.timeout_secs {
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
        let mut url: Url = self.protocol.into();
        url.set_query(Some(&format!("hostname={}&key={}", self.name, self.key)));
        url
    }
}
