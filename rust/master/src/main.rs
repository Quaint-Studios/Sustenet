use sustenet_master::MasterClient;
use sustenet_shared::utils;

#[tokio::main]
async fn main() {
    // Create a shutdown channel
    let mut shutdown_rx = match utils::shutdown_channel() {
        Ok(rx) => rx,
        Err(e) => {
            println!("Error creating shutdown channel: {e}");
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

// use sustenet_shared as shared;

// use sustenet_master::{ LOGGER, start_with_config };

// use tokio::select;

// use shared::utils;

// pub mod security;

// #[tokio::main]
// async fn main() {
//     let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

//     select! {
//         _ = shutdown_rx.recv() => {
//             LOGGER.warning("Shutting down...");
//         }
//         _ = start_with_config() => {}
//     }

//     LOGGER.success("The Master Server has been shut down.");
// }
