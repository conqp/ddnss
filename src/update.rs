use log::error;
use regex::Regex;
use reqwest::get;
use url::Url;

const REGEX: &str = r"(Updated \d+ hostname\.)";

/// Extension trait to update URLs.
pub trait Update {
    /// Update the host with the given base URL.
    ///
    /// # Returns
    ///
    /// * `Some(usize)` with the amount of updated hosts.
    /// * `None` if the amount of updated hosts is unknown.
    ///
    /// # Errors
    ///
    /// Returns an [`reqwest::Error`] if the update call failed.
    fn update(&self, url: Url) -> impl Future<Output = reqwest::Result<Option<usize>>>;
}

impl Update for (String, String) {
    async fn update(&self, mut url: Url) -> reqwest::Result<Option<usize>> {
        let (hostname, key) = self;
        url.set_query(Some(&format!("hostname={hostname}&key={key}")));
        update(url).await
    }
}

/// Update the given URL.
async fn update(url: Url) -> reqwest::Result<Option<usize>> {
    let regex = Regex::new(REGEX).expect("Regex is valid.");

    get(url)
        .await?
        .error_for_status()?
        .text()
        .await
        .map(|text| {
            regex
                .captures(&text)
                .and_then(|c| c.iter().flatten().next())
                .and_then(|capture| {
                    capture
                        .as_str()
                        .parse()
                        .inspect_err(|error| error!("Failed to parse usize: {error}"))
                        .ok()
                })
        })
}
