use std::sync::LazyLock;

use log::{error, trace};
use regex::Regex;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
    r"(Updated \d+ hostname\.)"
        .parse()
        .expect("Regex is valid.")
});

/// Parse the HTTP response text.
pub fn parse_response<T>(text: T) -> Option<usize>
where
    T: AsRef<str>,
{
    let text = text.as_ref();
    trace!("Parsing response: {text}");

    REGEX
        .captures(text)
        .and_then(|c| c.iter().flatten().next())
        .and_then(|capture| {
            capture
                .as_str()
                .parse()
                .inspect_err(|error| error!("Failed to parse usize: {error}"))
                .ok()
        })
}
