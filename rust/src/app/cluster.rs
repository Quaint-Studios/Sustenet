//! The goal for the Cluster Server is the following:
//! 1. Register with the Master Server.
//! 2. Deregister when disconnecting gracefully.
//! 3. Accept clients from the Master Server along with their authentication.
//! 4. Assign clients to a Fragment Server which is just an "isolated instance".
//! 5. Send and receive data from the clients through the Fragment Server.

use crate::{ cluster_debug, cluster_info, cluster_success, cluster_warning };

use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::TcpListener,
    net::TcpStream,
    select,
    sync::broadcast,
};

/// Start the Cluster Server and handles shutdown signals.
pub async fn start() {
    let mut shutdown_rx = crate::app::shutdown_channel().unwrap();
    let mut is_running = true;

    select! {
        _ = shutdown_rx.recv() => {
            is_running = false;
            cluster_warning!("Shutting down...");
        }
        _ = run(&mut is_running) => {}
    }

    if !is_running {
        cleanup().await;
        cluster_success!("Cluster Server has been shut down.");
    }
}

/// Cleanup the Cluster Server before shutting down.
async fn cleanup() {
    // TODO: Cleanup the Cluster Server.
    cluster_info!("Cleaning up the Cluster Server...");
}

/// Entrypoint for the Cluster Server.
#[inline(always)]
async fn run(is_running: &mut bool) {
    cluster_info!("Starting the Cluster Server...");

    // TODO: Read from config.
    const IP: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    let listener = TcpListener::bind(format!("{IP}:{PORT}")).await.unwrap();
    cluster_success!("Now listening on {IP}:{PORT}. Press Ctrl+C to stop.");
    cluster_debug!("Waiting for incoming connections...");

    let (tx, _rx) = broadcast::channel(10);

    while *is_running {
        let (mut socket, addr) = listener.accept().await.unwrap();
        cluster_debug!("Accepted connection from {:?}", addr);

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
