use std::net::{IpAddr, SocketAddr};

use anyhow::Result;
use clap::Parser;
use portal::{
    client,
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};
use tokio;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Domain name to look up records for
    name: Name,

    /// Target DNS server IP address
    target: IpAddr,

    /// Record type, e.g. A, AAAA or TXT
    #[arg(name = "TYPE", default_value_t = Type::A)]
    ty: Type,

    /// Use a different port than 53
    #[arg(short, long, default_value_t = 53)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let socket_addr = SocketAddr::new(cli.target, cli.port);

    let client = match client::Client::new().await {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };

    let (msg, dur) = client
        .query_duration((cli.name, cli.ty, Class::IN), socket_addr)
        .await?;

    println!(
        "{msg}\n\
        ;; QUERY TIME: {} msec\n\
        ;; SERVER: {}\n\
        ;; MSG SIZE: {}",
        dur.as_millis(),
        socket_addr,
        msg.len(),
    );
    Ok(())
}
