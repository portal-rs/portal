use std::path::PathBuf;

use clap::Args;

use crate::config;

#[derive(Args)]
pub struct Arguments {
    /// Path to the TOML config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config_path: Option<PathBuf>,
}

pub fn execute(args: Arguments) -> Result<(), clap::Error> {
    let cfg = match args.config_path {
        Some(path) => match config::read(path) {
            Ok(cfg) => cfg,
            Err(err) => return Err(err),
        },
        None => config::default(),
    };
}
