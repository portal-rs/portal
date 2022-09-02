use std::path::PathBuf;

use clap::Args;

use crate::{cmd::error::CmdError, config::Config, server};

#[derive(Args)]
pub struct Arguments {
    /// Path to the TOML config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config_path: Option<PathBuf>,
}

pub fn execute(args: Arguments) -> Result<(), CmdError> {
    let cfg = match args.config_path {
        Some(path) => match Config::from_file(path, None) {
            Err(err) => return Err(CmdError::new("run", err.to_string())),
            Ok(cfg) => cfg,
        },
        None => Config::default(),
    };

    let mut srv = match server::Server::new(cfg) {
        Ok(srv) => srv,
        Err(err) => return Err(CmdError::new("run", err.to_string())),
    };

    match srv.run() {
        Ok(_) => return Ok(()),
        Err(err) => return Err(CmdError::new("run", err.to_string())),
    };
}
