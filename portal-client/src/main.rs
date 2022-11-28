use std::env;

use portal::{
    client,
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};
use tokio;

#[tokio::main]
async fn main() {
    let client = match client::Client::new().await {
        Ok(c) => c,
        Err(err) => panic!("{}", err),
    };

    // Get args from command
    let args: Vec<String> = env::args().collect();

    // The domain name we want to query for RRs for
    let name = Name::try_from(args[1].clone()).unwrap();

    match client
        .query((name, Type::A, Class::IN), args[2].parse().unwrap())
        .await
    {
        Ok(_) => {}
        Err(err) => panic!("{}", err),
    }
}
