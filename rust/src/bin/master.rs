//! The job of the Master Server is the following:
//! 1. Register / deregister and manage all the clusters.
//! 2. Accept clients and authenticate or sign them up.
//! 3. Send clients to either a low population cluster or a specific one if provided.

use sustenet::{master_debug, master_error, master_info, master_success};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    select,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let mut shutdown_rx = shutdown_channel().unwrap();
    let mut is_running = true;

    select! {
        _ = shutdown_rx.recv() => {
            is_running = false;
            master_info!("Shutting down...");
        }
        _ = run(&mut is_running) => {}
    }
}

/// Create a channel to listen for shutdown signals.
fn shutdown_channel() -> Result<broadcast::Receiver<bool>, ctrlc::Error> {
    let (tx, rx) = broadcast::channel::<bool>(1);

    // Handle shutdowns gracefully.
    ctrlc::set_handler(move || {
        tx.send(true).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    Ok(rx)
}

#[inline(always)]
async fn run(is_running: &mut bool) {
    master_info!("Starting the Master Server...");

    // TODO: Read from config.
    const IP: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    let listener = TcpListener::bind(format!("{IP}:{PORT}")).await.unwrap();
    master_success!("Now listening on {IP}:{PORT}. Press Ctrl+C to stop.");
    master_debug!("Waiting for incoming connections...");
    master_error!("This is an error message.");

    let (tx, _rx) = broadcast::channel::<String>(10);

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
                let bytes_read = reader.read_line(&mut line).await.unwrap();
                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();
                let msg = rx.recv().await.unwrap();

                writer.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
