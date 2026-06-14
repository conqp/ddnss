use std::sync::LazyLock;

use log::error;
use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    r"Updated (?<n>\d+) hostname\."
        .parse()
        .expect("Regex is valid.")
});

/// Parse the HTTP response text.
pub fn parse_response<T>(text: T) -> Option<usize>
where
    T: AsRef<str>,
{
    let Some(capture) = REGEX.captures(text.as_ref()) else {
        error!("Did not capture via regex. Has the API changed?");
        error!("Original text on next line:\n{}", text.as_ref());
        return None;
    };

    capture["n"]
        .parse()
        .inspect_err(|error| error!("Failed to parse usize: {error}"))
        .ok()
}
