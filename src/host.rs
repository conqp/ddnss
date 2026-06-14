use serde::Deserialize;
use url::Url;

use crate::update::update;

/// Information about a host.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize)]
pub struct Host {
    name: String,
    key: String,
}

impl Host {
    /// Update this host.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if the HTTP request fails.
    pub async fn update(&self, mut url: Url) -> reqwest::Result<Option<usize>> {
        url.set_query(Some(&format!("hostname={}&key={}", self.name, self.key)));
        update(url).await
    }
}
