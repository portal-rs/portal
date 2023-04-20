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
    constants::{misc::DEFAULT_RESOLV_CONFIG_PATH, udp::MIN_MESSAGE_SIZE},
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

    /// Record type, e.g. A, AAAA or TXT
    #[arg(name = "TYPE")]
    ty: Option<Type>,

    /// Network class, e.g. IN, CH or HS
    #[arg(default_value_t = Class::IN)]
    class: Class,

    /// Only use IPv4 enabled nameservers
    #[arg(short = '4', group = "addr_family")]
    use_ipv4: bool,

    /// Only use IPv6 enabled nameservers
    #[arg(short = '6', group = "addr_family")]
    use_ipv6: bool,

    /// Target DNS server IP address
    #[arg(short, long)]
    server: Option<IpAddr>,

    /// Use a different port than 53
    #[arg(short, long, default_value_t = 53)]
    port: u16,

    /// Benchmark file
    #[arg(long)]
    bench_file: Option<PathBuf>,

    /// Benchmark results output file
    #[arg(long, default_value = "./bench.json")]
    bench_output: PathBuf,

    /// The receive buffer size
    #[arg(short, long, default_value_t = MIN_MESSAGE_SIZE)]
    buffer_size: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Build the client based on the provided params
    let client = Client::builder()
        .with_buffer_size(cli.buffer_size)
        .with_ip_version((cli.use_ipv4, cli.use_ipv6))
        .build()
        .await?;

    // If the user provided a bench file, do a benchmark
    if cli.bench_file.is_some() {
        return do_bench(client, cli.bench_file.unwrap(), cli.bench_output).await;
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

    let (msg, len, dur) = client
        .query_duration((name, ty, cli.class), socket_addr)
        .await?;

    println!(
        "{msg}\n\
        ;; QUERY TIME: {} msec\n\
        ;; SERVER: {}\n\
        ;; MSG SIZE: {}, rcvd: {}",
        dur.as_millis(),
        socket_addr,
        msg.size(),
        len,
    );
    Ok(())
}

async fn do_bench(client: Client, bench_file: PathBuf, output_file: PathBuf) -> Result<()> {
    // Read the benchmark config and create Rng
    let config = BenchConfig::from_file(bench_file)?;
    let mut rng = rand::thread_rng();

    // Precalculate domain name and type for each run
    let mut types = Vec::new();
    let mut runs = Vec::new();

    let domain_count = config.data.domains.len();
    let type_count = config.data.types.len();

    for _ in 0..config.bench.runs {
        runs.push(rng.gen_range(0..domain_count));
        types.push(rng.gen_range(0..type_count));
    }

    // Create delay duration and keep track of current run
    let delay = Duration::from_millis(config.bench.delay as u64);
    let mut current_run = 1;

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
            Ok((msg, _, dur)) => results.push(BenchResult::success(
                current_run,
                name,
                ty,
                msg.answers(),
                dur,
            )),
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
