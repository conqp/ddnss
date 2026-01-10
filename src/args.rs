use clap::Parser;

use crate::ip_protocol::IpProtocol;

#[derive(Clone, Debug, Parser)]
pub struct Args {
    #[clap(long, short, default_value_t = IpProtocol::V6, help = "IP protocol type")]
    pub(crate) ip_protocol: IpProtocol,
}
