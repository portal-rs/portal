use std::path::PathBuf;

use clap::Args;

use crate::{cmd::error::CmdError, config};

#[derive(Args)]
pub struct Arguments {
    /// Path to the TOML config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config_path: Option<PathBuf>,
}

pub fn execute(args: Arguments) -> Result<(), CmdError> {
    let _cfg = match args.config_path {
        Some(path) => match config::read(path, None) {
            Err(err) => return Err(CmdError::new("run", err.to_string())),
            Ok(cfg) => cfg,
        },
        None => config::default(),
    };

    Ok(())
}
