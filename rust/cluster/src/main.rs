use sustenet_shared as shared;

use sustenet_cluster::{cleanup, start, success, warning};
use shared::utils;
use tokio::{select, sync::mpsc::Sender};

struct DefaultPlugin;
impl shared::Plugin for DefaultPlugin {
    fn receive(
        &self,
        _tx: Sender<Box<[u8]>>,
        command: u8,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async move {
            match command {
                0 => println!("Command 0 received"),
                1 => println!("Command 1 received"),
                _ => println!("Unknown command received"),
            }
        })
    }

    fn info(&self, _: &str) {}
}

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start(DefaultPlugin {}) => {}
    }

    cleanup().await;
    success("The Cluster Server has been shut down.");
}
