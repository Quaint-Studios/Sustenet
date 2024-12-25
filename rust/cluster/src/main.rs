use std::{ net::Ipv4Addr, str::FromStr };

use shared::config::cluster::{ read, Settings };
use shared::packets::master::FromUnknown;
use tokio::io::{ AsyncBufReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;

use shared::log_message;
use shared::utils::constants::DEFAULT_IP;
use shared::utils::{ self, constants };

pub struct Connection {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start() => {}
    }

    cleanup().await;
    success("The Cluster Server has been shut down.");
}

fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

async fn cleanup() {}

async fn start() {
    let Settings {
        server_name,
        max_connections,
        port,
        key_name,
        master_ip,
        master_port,
        domain_pub_key,
    } = read();
    info(&server_name);

    let mut stream = TcpStream::connect(
        format!("{}:{}", get_ip(&master_ip), master_port)
    ).await.expect("Failed to connect to the Master Server.");

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);

    let handler = tokio::spawn(async move {
        let (reader, mut writer) = stream.split();

        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            select! {
                _ = reader.read_line(&mut line) => {
                    if line.is_empty() {
                        break;
                    }

                    info(&line);

                    line.clear();
                }
                result = rx.recv() => {
                    if let Some(data) = result {
                        writer.write_all(&data).await.expect("Failed to write to the Master Server.");
                        writer.flush().await.expect("Failed to flush the writer.");
                    } else {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Shutting down.");
                    }
                }
            }
        }
    });

    // Should send the server name with the passphrase as 2 separate strings but 1 packet.
    println!("Need to send {} and {} to Master Server.", server_name, key_name);
    send_data(&tx, Box::new([FromUnknown::BecomeCluster as u8])).await;

    match handler.await {
        Ok(_) => {}
        Err(e) => {
            error(format!("Error: {:?}", e).as_str());
        }
    }
}

async fn send_data(tx: &mpsc::Sender<Box<[u8]>>, data: Box<[u8]>) {
    tx.send(data).await.expect("Failed to send data to the Server.");
}

// region: Logging
// fn debug(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Debug, LogType::Cluster, "{}", message);
// }

fn info(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Cluster, "{}", message);
}

fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Cluster, "{}", message);
}

fn error(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Cluster, "{}", message);
}

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Cluster, "{}", message);
}
// endregion
