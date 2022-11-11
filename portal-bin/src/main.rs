use std::process;

use clap::{Parser, Subcommand};
use portal::errors::{AppError, AppErrorVariant};

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

pub fn execute() -> Result<(), AppError> {
    let cli = match Cli::try_parse() {
        Err(err) => {
            return Err(AppError::new(
                err.to_string(),
                AppErrorVariant::Generic(String::from("Failed to parse CLI")),
            ))
        }
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
