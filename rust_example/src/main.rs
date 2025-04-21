use sustenet::cluster::{ LOGGER, cleanup, start };
use sustenet::shared::Plugin;
use tokio::io::AsyncReadExt;
use tokio::sync::mpsc::Sender;

struct Reia {
    sender: std::sync::OnceLock<Sender<Box<[u8]>>>,
}
impl Reia {
    fn new() -> Self {
        Reia {
            sender: std::sync::OnceLock::new(),
        }
    }

    // Actual implementation of the receive function
    async fn handle_data(
        tx: Sender<Box<[u8]>>,
        command: u8,
        reader: &mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>
    ) {
        LOGGER.info(&format!("Received new command: {}", command));

        // Send a test message back to the sender
        if let Err(e) = tx.send(Box::new([20])).await {
            LOGGER.error(&format!("Failed to send message. {e}"));
        }

        // Read the message from the reader
        let len = reader.read_u8().await.unwrap() as usize;
        let mut passphrase = vec![0u8; len];
        match reader.read_exact(&mut passphrase).await {
            Ok(_) => {}
            Err(e) => {
                LOGGER.error(&format!("Failed to read passphrase to String: {:?}", e));
                return;
            }
        };
    }
}

// Plugin initialization
impl Plugin for Reia {
    fn set_sender(&self, tx: Sender<Box<[u8]>>) {
        // Set the sender
        if self.sender.set(tx).is_err() {
            LOGGER.error("Failed to set sender.");
        }
    }

    fn receive<'plug>(
        &self,
        tx: Sender<Box<[u8]>>,
        command: u8,
        reader: &'plug mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'plug>> {
        Box::pin(Self::handle_data(tx, command, reader))
    }

    fn info(&self, message: &str) {
        println!("{message}");
    }
}

#[tokio::main]
async fn main() {
    start(Reia::new()).await;
    cleanup().await;
}
