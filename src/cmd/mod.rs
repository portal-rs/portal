use clap::{Parser, Subcommand};

use self::error::CmdError;

mod error;
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

pub fn execute() -> Result<(), CmdError> {
    let cli = match Cli::try_parse() {
        Err(err) => return Err(CmdError::from(err)),
        Ok(c) => c,
    };

    match cli.commands {
        Commands::Run(args) => match run::execute(args) {
            Err(err) => return Err(err),
            Ok(_) => {}
        },
    }

    Ok(())
}
