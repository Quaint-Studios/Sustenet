use sustenet_cluster::ClusterServer;
use sustenet_shared::{ PluginPin, SenderBytes, ServerPlugin };
use tokio::{ io::BufReader, net::tcp::ReadHalf };

#[tokio::main]
async fn main() {
    let plugin = DefaultPlugin {
        sender: std::sync::OnceLock::new(),
    };
    let mut cluster = ClusterServer::new_from_config(plugin).await.unwrap();

    // Wait for the shutdown signal or start the server
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("Shutting down...");
        },
        _ = cluster.start() => {
            println!("Cluster server started.");
        }
    }
}

struct DefaultPlugin {
    sender: std::sync::OnceLock<SenderBytes>,
}
impl ServerPlugin for DefaultPlugin {
    fn set_sender(&self, tx: SenderBytes) {
        // Set the sender
        if self.sender.set(tx).is_err() {
            println!("Failed to set sender");
        }
    }

    fn receive<'plug>(
        &self,
        _tx: SenderBytes,
        command: u8,
        _reader: &'plug mut BufReader<ReadHalf<'_>>
    ) -> PluginPin<'plug> {
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
