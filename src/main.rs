use std::process;

mod cmd;
mod config;
mod constants;
mod server;

fn main() {
    match cmd::execute() {
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
        Ok(_) => {}
    };
}
