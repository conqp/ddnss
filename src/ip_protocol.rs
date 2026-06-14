use std::fmt::Display;

use clap::ValueEnum;
use clap::builder::PossibleValue;
use url::Url;

const IPV4_URL: &str = "https://ip4.ddnss.de/upd.php";
const IPV6_URL: &str = "https://ddnss.de/upd.php";

/// IP protocol type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IpProtocol {
    V4,
    V6,
}

impl Display for IpProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V4 => write!(f, "IPv4"),
            Self::V6 => write!(f, "IPv6"),
        }
    }
}

impl From<IpProtocol> for Url {
    fn from(endpoint: IpProtocol) -> Self {
        match endpoint {
            IpProtocol::V4 => Self::parse(IPV4_URL),
            IpProtocol::V6 => Self::parse(IPV6_URL),
        }
        .expect("URL is valid.")
    }
}

impl ValueEnum for IpProtocol {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::V4, Self::V6]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::V4 => Some(PossibleValue::new("ipv4")),
            Self::V6 => Some(PossibleValue::new("ipv6")),
        }
    }
}
