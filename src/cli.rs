use std::{net::IpAddr, str::FromStr, time::Duration};

use clap::Parser;

#[derive(Parser)]
#[command(about = "A simple port scanner", version = "0.1.0")]
pub struct Cli {
    #[arg(value_name = "IPAddress", help = "IP address to scan")]
    ip: String,

    #[arg(value_name = "Ports", help = "Ports to scan (e.g. 80,443,8080)")]
    ports: String,

    #[arg(
        short = 'n',
        long,
        value_name = "Threads",
        help = "Number of threads to use (default: 5)"
    )]
    threads: Option<u16>,

    #[arg(
        short,
        long,
        value_name = "Timeout",
        help = "Timeout for each port scan (default: 500ms)"
    )]
    timeout: Option<u64>,
}

pub struct Arguments {
    pub ip: IpAddr,
    pub ports: Vec<u16>,
    pub num_threads: u16,
    pub timeout: std::time::Duration,
}

impl Arguments {
    pub fn new() -> Result<Self, String> {
        let app = Cli::parse();
        let ip = IpAddr::from_str(&app.ip).map_err(|_| "Invalid IP address")?;
        // TODO: support port ranges, e.g. 80-90
        let ports = app
            .ports
            .split(',')
            .map(|port| {
                port.parse::<u16>()
                    .map_err(|_| format!("Invalid port: {}", port))
            })
            .collect::<Result<Vec<u16>, String>>()?;
        let num_threads = app.threads.unwrap_or(5);
        let timeout = Duration::from_millis(app.timeout.unwrap_or(500));
        Ok(Arguments {
            ip,
            ports,
            num_threads,
            timeout,
        })
    }
}
