use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use portal::{config::Config, server::Server};

#[derive(Args)]
pub struct Arguments {
    /// Path to the TOML config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config_path: Option<PathBuf>,
}

pub fn execute(args: Arguments) -> Result<()> {
    // Load config located at provided path or use default config
    let cfg = match args.config_path {
        Some(path) => Config::from_file(path, None)?,
        None => Config::default(),
    };

    // Create and run DNS server
    let mut srv = Server::new(cfg)?;
    srv.run()?;

    Ok(())
}
