use cluster::{ cleanup, error, start };
use shared::Plugin;
use tokio::sync::mpsc::Sender;

struct Reia;
impl Plugin for Reia {
    fn receive(
        &self,
        tx: Sender<Box<[u8]>>,
        command: u8
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async move {
            println!("Reia plugin handling command: {}", command);
            if let Err(e) = tx.send(Box::new([20])).await {
                error(format!("Failed to send message. {e}").as_str());
            }
        })
    }
}

#[tokio::main]
async fn main() {
    start(Reia {}).await;
    cleanup().await;
}
