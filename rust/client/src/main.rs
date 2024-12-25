use std::{ net::Ipv4Addr, str::FromStr };

use tokio::io::{ AsyncBufReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;

use shared::log_message;
use shared::utils::constants::{ DEFAULT_IP, MASTER_PORT };
use shared::utils::{ self, constants };

pub enum ConnectionType {
    MasterServer,
    ClusterServer,
    None,
}

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
        _ = start(get_ip(DEFAULT_IP), MASTER_PORT) => {}
    }

    cleanup().await;
    success("Client has been shut down.");
}

fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

async fn cleanup() {}

async fn start(ip: Ipv4Addr, port: u16) {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await.expect(
        "Failed to connect to the Master Server."
    );

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);

    let tcp_handler = tokio::spawn(async move {
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
                        writer.write_all(&data).await.expect("Failed to write to the Server.");
                        writer.flush().await.expect("Failed to flush the writer.");
                    } else {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Shutting down.");
                    }
                }
            }
        }
    });

    loop {
        send_data(tx.clone(), Box::new([0])).await;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}

async fn send_data(tx: mpsc::Sender<Box<[u8]>>, data: Box<[u8]>) {
    tx.send(data).await.expect("Failed to send data to the Server.");
}

// region: Logging
// fn debug(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Debug, LogType::Client, "{}", message);
// }

fn info(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Client, "{}", message);
}

fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Client, "{}", message);
}

// fn error(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Error, LogType::Client, "{}", message);
// }

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Client, "{}", message);
}
// endregion
