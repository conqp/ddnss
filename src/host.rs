use serde::{Deserialize, Serialize};
use url::Url;

use crate::ip_protocol::IpProtocol;
use crate::update::update;

/// Information about a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Host {
    name: String,
    key: String,
    #[serde(default)]
    protocol: IpProtocol,
}

impl Host {
    /// Update this host.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if the HTTP request fails.
    pub async fn update(&self) -> reqwest::Result<Option<usize>> {
        let mut url: Url = self.protocol.into();
        url.set_query(Some(&format!("hostname={}&key={}", self.name, self.key)));
        update(url).await
    }
}
