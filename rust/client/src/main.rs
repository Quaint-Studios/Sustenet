use sustenet_shared as shared;

use tokio::sync::mpsc::Sender;

use shared::{ lselect, utils };
use sustenet_client::{ CONNECTION, LOGGER, cleanup, start};

struct DefaultPlugin {
    sender: std::sync::OnceLock<Sender<Box<[u8]>>>,
}
impl shared::ClientPlugin for DefaultPlugin {
    fn set_sender(&self, tx: Sender<Box<[u8]>>) {
        // Set the sender
        if self.sender.set(tx).is_err() {
            LOGGER.error("Failed to set sender");
        }
    }

    fn receive_master<'plug>(
        &self,
        _tx: Sender<Box<[u8]>>,
        command: u8,
        _reader: &'plug mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async move {
            match command {
                0 => println!("Command 0 received"),
                1 => println!("Command 1 received"),
                _ => println!("Unknown command received"),
            }
        })
    }

    fn receive_cluster<'plug>(
        &self,
        _tx: Sender<Box<[u8]>>,
        command: u8,
        _reader: &'plug mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>
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
pub async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    lselect! {
        _ = shutdown_rx.recv() => {
            LOGGER.warning("Shutting down...");
            break;
        }
        _ = start(DefaultPlugin { sender: std::sync::OnceLock::new() }) => {
            if CONNECTION.read().await.is_none() {
                LOGGER.warning("Closing client...");
                break;
            }
        }
    }

    cleanup().await;
    LOGGER.success("The Client has been shut down.");
}
