use cli::Arguments;
use sniffer::scan_ports_multi_threads;
use colored::Colorize;

mod cli;
mod sniffer;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Arguments::new()?;
    let open_ports =
        scan_ports_multi_threads(args.ip, &args.ports, args.timeout, args.num_threads).await;

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
