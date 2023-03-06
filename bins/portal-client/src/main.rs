use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use tokio;

use portal::{
    client::Client,
    constants::misc::DEFAULT_RESOLV_CONFIG_PATH,
    resolv::{ResolvConfig, ResolvOption},
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Domain name to look up records for
    name: Option<Name>,

    /// Target DNS server IP address
    #[arg(short, long)]
    server: Option<IpAddr>,

    /// Record type, e.g. A, AAAA or TXT
    #[arg(name = "TYPE")]
    ty: Option<Type>,

    /// Use a different port than 53
    #[arg(short, long, default_value_t = 53)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let target = match cli.server {
        Some(target) => target,
        None => {
            // If no target DNS server IP address is provided, fallback to
            // local resolv.conf file.
            let config = ResolvConfig::from_file(DEFAULT_RESOLV_CONFIG_PATH.into())?;

            // Set the target IP address to 127.0.0.1 in case we don't find
            // a DNS server IP address in the resolv.conf file.
            let mut ip_addr = IpAddr::V4(Ipv4Addr::LOCALHOST);

            for option in config.options() {
                match option {
                    ResolvOption::Nameserver(ip) => ip_addr = *ip,
                    _ => continue,
                }
            }

            ip_addr
        }
    };

    let (name, ty) = if cli.name.is_none() && cli.ty.is_none() {
        (Name::default(), Type::NS)
    } else if cli.ty.is_none() {
        (cli.name.unwrap(), Type::A)
    } else {
        (cli.name.unwrap(), cli.ty.unwrap())
    };

    let socket_addr = SocketAddr::new(target, cli.port);
    let client = Client::new().await?;

    let (msg, dur) = client
        .query_duration((name, ty, Class::IN), socket_addr)
        .await?;

    println!(
        "{msg}\n\
        ;; QUERY TIME: {} msec\n\
        ;; SERVER: {}\n\
        ;; MSG SIZE: {}",
        dur.as_millis(),
        socket_addr,
        msg.size(),
    );
    Ok(())
}
