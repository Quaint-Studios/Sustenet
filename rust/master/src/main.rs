// use sustenet_master::MasterServer;

fn main() {
    sustenet_master::tests::test_tcplistener::test_create_connections();
}

// #[tokio::main]
// async fn main() {
//     let mut master = MasterServer::new_from_config().await.unwrap();

//     // Wait for the shutdown signal or start the server
//     tokio::select! {
//         _ = tokio::signal::ctrl_c() => {
//             println!("Shutting down...");
//         },
//         _ = master.start() => println!("Master server started.")
//     }
// }
