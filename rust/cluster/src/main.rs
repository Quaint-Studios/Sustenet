use std::{ net::Ipv4Addr, str::FromStr };

use tokio::io::{ AsyncReadExt, AsyncWriteExt, BufReader };
use tokio::net::TcpStream;
use tokio::select;
use tokio::sync::mpsc;

use shared::config::cluster::{ read, Settings };
use shared::packets::master::{ FromUnknown, ToUnknown };
use shared::security::aes::{decrypt, load_key};
use shared::utils;
use shared::utils::constants::DEFAULT_IP;

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
        max_connections: _,
        port: _,
        key_name,
        master_ip,
        master_port,
        domain_pub_key: _,
    } = read();
    info(&server_name);
    let key = load_key(key_name.as_str()).expect("Failed to load the key.");

    let (tx, mut rx) = mpsc::channel::<Box<[u8]>>(10);
    let tx_clone = tx.clone();

    let handler = tokio::spawn(async move {
        let mut stream = TcpStream::connect(
            format!("{}:{}", get_ip(&master_ip), master_port)
        ).await.expect("Failed to connect to the Master Server.");

        let (reader, mut writer) = stream.split();
        let mut reader = BufReader::new(reader);

        loop {
            select! {
                command = reader.read_u8() => {
                    if command.is_err() {
                        continue;
                    }

                    debug(format!("Cluster Server received data: {:?}", command).as_str());

                    match command.unwrap() {
                        x if x == ToUnknown::VerifyCluster as u8 => {
                            let len = reader.read_u8().await.unwrap() as usize;
                            let mut passphrase = vec![0u8; len];
                            match reader.read_exact(&mut passphrase).await {
                                Ok(_) => {},
                                Err(e) => {
                                    error(format!("Failed to read passphrase to String: {:?}", e).as_str());
                                    continue;
                                }
                            }

                            let mut data = vec![FromUnknown::AnswerCluster as u8];

                            let decrypted_passphrase = decrypt(passphrase.as_slice(), &key);

                            data.push(decrypted_passphrase.len() as u8);
                            data.extend_from_slice(&decrypted_passphrase);
                            send_data(&tx_clone, data.into_boxed_slice()).await;
                        }
                        x if x == ToUnknown::CreateCluster as u8 => {
                            success("We did it! We verified the cluster!");
                        }
                        _ => (),
                    }
                }
                result = rx.recv() => {
                    if let Some(data) = result {
                        writer.write_all(&data).await.expect("Failed to write to the Master Server.");
                        writer.flush().await.expect("Failed to flush the writer.");
                    } else {
                        writer.shutdown().await.expect("Failed to shutdown the writer.");
                        info("Cluster Server is shutting down its client writer.");
                        break;
                    }
                }
            }
        }
    });

    // Should send the server name with the passphrase as 2 separate strings but 1 packet.
    println!("Need to send {} and {} to Master Server.", server_name, key_name);
    let command = FromUnknown::BecomeCluster as u8;
    let key_name = b"cluster_key";

    let mut data = [command].to_vec();
    data.push(key_name.len() as u8);
    data.extend_from_slice(key_name);

    let data = data.into_boxed_slice();
    send_data(&tx, data).await;

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
use shared::{ log_message, utils::constants::DEBUGGING };

fn debug(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Cluster, "{}", message);
}

fn info(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Info, LogType::Cluster, "{}", message);
}

fn warning(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Cluster, "{}", message);
}

fn error(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Error, LogType::Cluster, "{}", message);
}

fn success(message: &str) {
    if !DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Cluster, "{}", message);
}
// endregion
