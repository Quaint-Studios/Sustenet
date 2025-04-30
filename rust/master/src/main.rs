use sustenet_master::MasterClient;
use sustenet_shared::utils;

#[tokio::main]
async fn main() {
    // Create a shutdown channel
    let mut shutdown_rx = match utils::shutdown_channel() {
        Ok(rx) => rx,
        Err(e) => {
            eprintln!("Error creating shutdown channel: {e}");
            return;
        }
    };

    let mut master = match sustenet_master::MasterServer::new_from_config().await {
        Ok(master) => master,
        Err(e) => {
            eprintln!("Failed to create master server: {e}");
            return;
        }
    };

    // Wait for the shutdown signal or start the server
    tokio::select! {
        _ = shutdown_rx.recv() => {
            println!("Shutting down...");
        }
        _ = master.start() => {
            println!("Master server started.");
        }
    }
}
