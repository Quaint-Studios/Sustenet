//! The job of the Client is the following:
//! 1. Connect to the Master Server.
//! 2. Authenticate or sign up.
//! 3. Receive a list of clusters.
//! 4. Connect to a cluster randomly or by choice.
//! 5. Send and receive data from the cluster.
//! 6. Gracefully go back to the Master Server on disconnect from the cluster.
//! 7. Reconnect to Master Server in increasing intervals on disconnect or failed connection.

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}