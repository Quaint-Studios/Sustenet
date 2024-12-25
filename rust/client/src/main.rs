use std::{ net::Ipv4Addr, str::FromStr };

use shared::packets::cluster::ToClient;
use shared::packets::master::{FromUnknown, ToUnknown};
use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
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
    success("The Client has been shut down.");
}

fn get_ip(ip: &str) -> Ipv4Addr {
    Ipv4Addr::from_str(ip).unwrap_or(Ipv4Addr::from_str(DEFAULT_IP).unwrap_or(Ipv4Addr::LOCALHOST))
}

async fn cleanup() {}

async fn start(ip: Ipv4Addr, port: u16) {
    let mut stream = TcpStream::connect(format!("{}:{}", ip, port)).await.expect(
        "Failed to connect to the Master Server."
    );

    // TODO: Mutating this may not work since it's moved.
    let connection_type = ConnectionType::MasterServer;

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);

    let handler = tokio::spawn(async move {
        let (reader, mut writer) = stream.split();

        let mut reader = BufReader::new(reader);

        loop {
            select! {
                command = reader.read_u8() => {
                    if command.is_err() {
                        break;
                    }

                    debug(format!("Client received data: {:?}", command).as_str());

                    match connection_type {
                        ConnectionType::MasterServer => match command.unwrap() {
                            x if x == ToUnknown::SendClusters as u8 => todo!(),
                            _ => (),
                        }
                        ConnectionType::ClusterServer => match command.unwrap() {
                            x if x == ToClient::SendClusters as u8 => todo!(),
                            x if x == ToClient::DisconnectCluster as u8 => todo!(),
                            x if x == ToClient::LeaveCluster as u8 => todo!(),

                            x if x == ToClient::VersionOfKey as u8 => todo!(),
                            x if x == ToClient::SendPubKey as u8 => todo!(),
                            x if x == ToClient::Authenticate as u8 => todo!(),

                            x if x == ToClient::Move as u8 => todo!(),
                            _ => (),
                        }
                        _ => (),
                    }
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

    send_data(&tx, Box::new([FromUnknown::RequestClusters as u8])).await;

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
fn debug(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Client, "{}", message);
}

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

fn error(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Client, "{}", message);
}

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Client, "{}", message);
}
// endregion
