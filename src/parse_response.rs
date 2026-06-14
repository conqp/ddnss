use std::sync::LazyLock;

use log::error;
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
    REGEX
        .captures(text.as_ref())
        .and_then(|c| c.iter().flatten().next())
        .and_then(|capture| {
            capture
                .as_str()
                .parse()
                .inspect_err(|error| error!("Failed to parse usize: {error}"))
                .ok()
        })
}
