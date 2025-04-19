use sustenet_client::{ cleanup, start, success, warning, CONNECTION };
use shared::{ lselect, utils };

#[tokio::main]
pub async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    lselect! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
            break;
        }
        _ = start() => {
            if CONNECTION.read().await.is_none() {
                warning("Closing client...");
                break;
            }
        }
    }

    cleanup().await;
    success("The Client has been shut down.");
}
