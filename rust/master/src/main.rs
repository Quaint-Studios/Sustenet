use sustenet_shared as shared;

use sustenet_master::{start, success, warning};

use tokio::select;

use shared::utils;

pub mod security;

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start() => {}
    }

    success("The Master Server has been shut down.");
}
