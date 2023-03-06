use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};

use anyhow::Result;
use clap::Parser;
use rand::Rng;
use spinoff::{spinners, Color, Spinner};
use tokio::{self, time::sleep};

use portal::{
    client::Client,
    constants::misc::DEFAULT_RESOLV_CONFIG_PATH,
    resolv::{ResolvConfig, ResolvOption},
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

use crate::bench::{BenchConfig, BenchResult, BenchSummary};

mod bench;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Domain name to look up records for
    name: Option<Name>,

    /// Target DNS server IP address
    #[arg(short, long)]
    server: Option<IpAddr>,

    /// Record type, e.g. A, AAAA or TXT
    #[arg(name = "TYPE")]
    ty: Option<Type>,

    /// Use a different port than 53
    #[arg(short, long, default_value_t = 53)]
    port: u16,

    /// Benchmark file
    #[arg(long)]
    bench_file: Option<PathBuf>,

    /// Benchmark results output file
    #[arg(long, default_value = "./bench.json")]
    bench_output: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // If the user provided a bench file, do a benchmark
    if cli.bench_file.is_some() {
        return do_bench(cli.bench_file.unwrap(), cli.bench_output).await;
    }

    let target = match cli.server {
        Some(target) => target,
        None => {
            // If no target DNS server IP address is provided, fallback to
            // local resolv.conf file.
            let config = ResolvConfig::from_file(DEFAULT_RESOLV_CONFIG_PATH.into())?;

            // Set the target IP address to 127.0.0.1 in case we don't find
            // a DNS server IP address in the resolv.conf file.
            let mut ip_addr = IpAddr::V4(Ipv4Addr::LOCALHOST);

            for option in config.options() {
                match option {
                    ResolvOption::Nameserver(ip) => ip_addr = *ip,
                    _ => continue,
                }
            }

            ip_addr
        }
    };

    let (name, ty) = if cli.name.is_none() && cli.ty.is_none() {
        (Name::default(), Type::NS)
    } else if cli.ty.is_none() {
        (cli.name.unwrap(), Type::A)
    } else {
        (cli.name.unwrap(), cli.ty.unwrap())
    };

    let socket_addr = SocketAddr::new(target, cli.port);
    let client = Client::new().await?;

    let (msg, dur) = client
        .query_duration((name, ty, Class::IN), socket_addr)
        .await?;

    println!(
        "{msg}\n\
        ;; QUERY TIME: {} msec\n\
        ;; SERVER: {}\n\
        ;; MSG SIZE: {}",
        dur.as_millis(),
        socket_addr,
        msg.size(),
    );
    Ok(())
}

async fn do_bench(bench_file: PathBuf, output_file: PathBuf) -> Result<()> {
    // Read the benchmark config and create Rng
    let config = BenchConfig::from_file(bench_file)?;
    let mut rng = rand::thread_rng();

    // Precalculate domain name and type for each run
    let mut types = Vec::new();
    let mut runs = Vec::new();

    for _ in 0..config.bench.runs {
        runs.push(rng.gen_range(0..config.data.domains.len()));
        types.push(rng.gen_range(0..config.data.types.len()));
    }

    // Create delay duration and keep track of current run
    let delay = Duration::from_millis(config.bench.delay as u64);
    let mut current_run = 1;

    // Create DNS client
    let client = Client::new().await?;

    // Create loading spinner
    println!(
        "Running {} tests with a delay of {} ms",
        config.bench.runs, config.bench.delay
    );
    let mut spinner = Spinner::new(spinners::Dots, "Running benchmark...", Color::White);

    // Prepare result vector
    let mut results = Vec::new();

    // TODO (Techassi): Make this whole benchmark run in a tokio task
    // Iterate over all the runs
    for (name_index, type_index) in runs.iter().zip(types) {
        spinner.update_text(format!("Run {}/{}", current_run, config.bench.runs));

        let name = &config.data.domains[*name_index];
        let ty = &config.data.types[type_index];

        match client
            .query_duration((name, ty, &Class::IN), config.server)
            .await
        {
            Ok((_msg, dur)) => results.push(BenchResult::success(current_run, name, ty, dur)),
            Err(err) => results.push(BenchResult::error(current_run, name, ty, err.to_string())),
        };

        current_run += 1;
        sleep(delay).await;
    }

    spinner.stop_and_persist(">", "Done!");

    // Save the result
    let json = serde_json::to_string(&BenchSummary {
        server: config.server.to_string(),
        delay: config.bench.delay,
        runs: config.bench.runs,
        results,
    })?;

    fs::write(output_file.clone(), json)?;

    println!(
        "Output saved as JSON in '{}'",
        output_file.to_str().unwrap()
    );
    Ok(())
}
