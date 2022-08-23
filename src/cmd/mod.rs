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

pub fn execute() -> Result<(), clap::Error> {
    let cli = match Cli::try_parse() {
        Err(err) => return Err(err),
        Ok(c) => c,
    };

    match cli.commands {
        Commands::Run(args) => run::execute(args),
    }

    Ok(())
}
