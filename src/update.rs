use log::error;
use regex::Regex;
use reqwest::get;
use url::Url;

const REGEX: &str = r"(Updated \d+ hostname\.)";

/// Update the given URL.
///
/// # Errors
///
/// Returns a [`reqwest::Error`] if the HTTP request fails.
pub async fn update(url: Url) -> reqwest::Result<Option<usize>> {
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
