use std::{net::IpAddr, str::FromStr, time::Duration};

use clap::Parser;

#[derive(Parser)]
#[command(about = "A simple port scanner", version = "0.1.0")]
pub struct Cli {
    #[arg(value_name = "IPAddress", help = "IP address to scan")]
    ip: String,

    #[arg(value_name = "Ports", help = "Ports to scan (e.g. 80,443,8080 or 8080-8090 or 80,8080-8090)")]
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
        let ports = parse_ports(&app.ports)?;
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

fn parse_ports(args: &str) -> Result<Vec<u16>, String> {
    let mut ports = vec![];
    for arg in args.split(',').filter(|arg| !arg.is_empty()) {
        if arg.contains('-') {
            let range = arg
                .split('-')
                .map(|port| port.parse::<u16>())
                .collect::<Result<Vec<u16>, _>>()
                .map_err(|_| format!("Invalid port range: {}", arg))?;
            if range.len() != 2 {
                return Err(format!("Invalid port range: {}", arg));
            }
            let start = range[0];
            let end = range[1];
            if start > end {
                return Err(format!("Invalid port range: {}", arg));
            }
            ports.extend(start..=end);
        } else {
            let port = arg
                .parse::<u16>()
                .map_err(|_| format!("Invalid port: {}", arg));
            ports.push(port?);
        }
    }
    // Remove duplicates
    ports.sort_unstable();
    ports.dedup();
    Ok(ports)
}
