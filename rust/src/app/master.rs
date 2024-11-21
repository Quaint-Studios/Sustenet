//! The job of the Master Server is the following:
//! 1. Register / deregister and manage all the clusters.
//! 2. Accept clients and authenticate or sign them up.
//! 3. Send clients to either a low population cluster or a specific one if provided.

use crate::{
    master::MasterServer,
    master_debug,
    master_info,
    master_success,
    master_warning,
    transport::base_server::ServerCore,
};

use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::TcpListener,
    select,
    sync::broadcast,
};

/// Start the Master Server and handles shutdown signals.
pub async fn start() {
    let mut shutdown_rx = crate::app::shutdown_channel().unwrap();
    let mut master_server = MasterServer::new(None, None).await.unwrap();

    select! {
        _ = shutdown_rx.recv() => {
            master_server.is_running = false;
            master_warning!("Shutting down...");
        }
        _ = master_server.start() => {}
    }

    if !master_server.is_running {
        cleanup().await;
        master_success!("Master Server has been shut down.");
    }
}

/// Cleanup the Master Server before shutting down.
async fn cleanup() {
    // TODO: Cleanup the Master Server.
    master_info!("Cleaning up the Master Server...");
}

/// Entrypoint for the Master Server.
#[inline(always)]
async fn run(is_running: &bool) {
    master_info!("Starting the Master Server...");

    // TODO: Read from config.
    const IP: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    let listener = TcpListener::bind(format!("{IP}:{PORT}")).await.unwrap();
    master_success!("Now listening on {IP}:{PORT}. Press Ctrl+C to stop.");
    master_debug!("Waiting for incoming connections...");

    let (tx, _rx) = broadcast::channel(10);

    while *is_running {
        let (mut socket, addr) = listener.accept().await.unwrap();
        master_debug!("Accepted connection from {:?}", addr);

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        // Break if the line is empty.
                        if result.unwrap() == 0 {
                            break;
                        }

                        // Send the line to the channel.
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        // Write the message to the writer.
                        let (msg, msg_addr) = result.unwrap();

                        if addr != msg_addr {
                            writer.write_all(&msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}

mod tests {}
