use sustenet_cluster::LOGGER;

#[tokio::main]
async fn main() {
    // Create a shutdown channel
    let mut shutdown_rx = match utils::shutdown_channel() {
        Ok(rx) => rx,
        Err(e) => {
            LOGGER.error(&format!("Error creating shutdown channel: {e}"));
            return;
        }
    };

    // Wait for the shutdown signal or start the server
    lselect! {
        _ = shutdown_rx.recv() => {
            LOGGER.warning("Shutting down...");
            break;
        }
    }
    
    LOGGER.success("The Cluster Server has been shut down.");
}

// use sustenet_shared as shared;

// use tokio::{ select, sync::mpsc::Sender };

// use shared::utils;
// use sustenet_cluster::{ cleanup, start_with_config, LOGGER };

// struct DefaultPlugin {
//     sender: std::sync::OnceLock<Sender<Box<[u8]>>>,
// }
// impl shared::ServerPlugin for DefaultPlugin {
//     fn set_sender(&self, tx: Sender<Box<[u8]>>) {
//         // Set the sender
//         if self.sender.set(tx).is_err() {
//             LOGGER.error("Failed to set sender");
//         }
//     }

//     fn receive<'plug>(
//         &self,
//         _tx: Sender<Box<[u8]>>,
//         command: u8,
//         _reader: &'plug mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>
//     ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
//         Box::pin(async move {
//             match command {
//                 0 => println!("Command 0 received"),
//                 1 => println!("Command 1 received"),
//                 _ => println!("Unknown command received"),
//             }
//         })
//     }

//     fn info(&self, _: &str) {}
// }

// #[tokio::main]
// async fn main() {
//     let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

//     select! {
//         _ = shutdown_rx.recv() => {
//             LOGGER.warning("Shutting down...");
//         }
//         _ = start_with_config(DefaultPlugin { sender: std::sync::OnceLock::new() }) => {}
//     }

//     cleanup().await;
//     LOGGER.success("The Cluster Server has been shut down.");
// }
