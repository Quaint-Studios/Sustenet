use sustenet::app::master;

#[tokio::main]
async fn main() {
    master::start().await;
}
