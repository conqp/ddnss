use std::time::Duration;

use reqwest::Client;
use url::Url;

use crate::ip_protocol::IpProtocol;
use crate::parse_response::parse_response;

/// Settings for an update query.
#[derive(Clone, Debug)]
pub struct Update {
    protocol: IpProtocol,
    key: String,
    timeout: Option<Duration>,
    hosts: Vec<String>,
}

impl Update {
    pub const fn new(
        protocol: IpProtocol,
        key: String,
        timeout: Option<Duration>,
        hosts: Vec<String>,
    ) -> Self {
        Self {
            protocol,
            key,
            timeout,
            hosts,
        }
    }

    /// Update this host.
    ///
    /// # Error
    ///
    /// Returns a [`reqwest::Error`] if the HTTP request fails.
    pub async fn run(&self) -> reqwest::Result<Option<usize>> {
        let mut client = Client::new().get(self.url());

        if let Some(timeout) = self.timeout {
            client = client.timeout(timeout);
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
        url.set_query(Some(&format!("key={}&host={}", self.key, self.hosts())));
        url
    }

    /// Return the list of hosts as a comma separated string.
    fn hosts(&self) -> String {
        self.hosts.join(",")
    }
}
