use std::process;

mod binary;
mod client;
mod cmd;
mod config;
mod constants;
mod macros;
mod packing;
mod resolver;
mod server;
mod types;
mod utils;

fn main() {
    match cmd::execute() {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(_) => {}
    };
}
