use clap::Parser;
use cli::{Arguments, Cli};
use colored::Colorize;
use sniffer::scan_ports_multi_threads;

mod cli;
mod sniffer;

#[tokio::main]
async fn main() -> Result<(), String> {
    let app = Cli::parse();
    let args = Arguments::new(&app)?;
    println!(
        "{}",
        format!("Scanning port(s) on {} ({})", app.destination, args.ip).yellow()
    );

    let open_ports =
        scan_ports_multi_threads(args.ip, &args.ports, args.timeout, args.num_threads).await;

    println!(
        "\n{}",
        format!(
            "{} port(s) scanned, {} open port(s)\n",
            args.ports.len(),
            open_ports.len()
        )
        .yellow()
    );

    for port in &open_ports {
        println!("{}", format!("{}\tOPEN", port).green());
    }

    for port in &args.ports {
        if !open_ports.contains(port) {
            println!("{}", format!("{}\tCLOSED", port).red());
        }
    }

    Ok(())
}
