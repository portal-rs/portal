use std::path::PathBuf;

use anyhow::Result;
use clap::Args;
use portal::{config::RawConfig, server::Server};

#[derive(Args)]
pub struct Arguments {
    /// Path to the TOML config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,
}

pub fn execute(args: Arguments) -> Result<()> {
    // Load config located at provided path or use default config
    let raw_config = match args.config {
        Some(path) => RawConfig::from_file(path)?,
        None => RawConfig::default(),
    };

    // Validate the raw config
    let config = raw_config.validate()?;

    // Create and run DNS server
    let mut srv = Server::new(config);
    srv.run()?;

    Ok(())
}
