use std::net::IpAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time;

pub async fn is_port_open(ip: IpAddr, port: u16, timeout: Duration) -> bool {
    let future = async { TcpStream::connect((ip, port)).await.is_ok() };
    time::timeout(timeout, future).await.unwrap_or(false)
}

pub async fn scan_ports(ip: IpAddr, ports: &[u16], timeout: Duration) -> Vec<u16> {
    let mut open_ports = Vec::new();
    for port in ports {
        if is_port_open(ip, *port, timeout).await {
            open_ports.push(*port);
        }
    }
    open_ports
}

pub async fn scan_ports_multi_threads(
    ip: IpAddr,
    ports: &[u16],
    timeout: Duration,
    num_threads: u16,
) -> Vec<u16> {
    let chunk_size: usize = (ports.len() as f32 / num_threads as f32).ceil() as usize;
    let (tx, mut rx) = mpsc::channel(num_threads as usize);
    for chunk in ports.chunks(chunk_size) {
        let tx = tx.clone();
        let ip = ip;
        let ports = chunk.to_vec();
        tokio::spawn(async move {
            let open_ports = scan_ports(ip, &ports, timeout).await;
            if tx.send(open_ports).await.is_err() {
                eprintln!("Error sending data from a thread: {}", ports[0]);
            }
        });
    }
    drop(tx);
    let mut open_ports = Vec::new();
    while let Some(mut ports) = rx.recv().await {
        open_ports.append(&mut ports);
    }
    open_ports
}
