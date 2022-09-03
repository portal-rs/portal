use std::process;

mod binary;
mod cmd;
mod config;
mod constants;
mod pack;
mod server;
mod types;

fn main() {
    match cmd::execute() {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(_) => {}
    };
}
