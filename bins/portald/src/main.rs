use std::process;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start and run the DNS server
    Run(run::Arguments),
}

fn main() {
    if let Err(err) = execute() {
        println!("{}", err);
        process::exit(1)
    }
}

pub fn execute() -> Result<()> {
    let cli = Cli::try_parse()?;

    match cli.commands {
        Commands::Run(args) => run::execute(args)?,
    }

    Ok(())
}
