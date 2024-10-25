//! The goal for the Cluster Server is the following:
//! 1. Register with the Master Server.
//! 2. Deregister when disconnecting gracefully.
//! 3. Accept clients from the Master Server along with their authentication.
//! 4. Assign clients to a Fragment Server which is just an "isolated instance".
//! 5. Send and receive data from the clients through the Fragment Server.

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    // Entrypoint design:
    // Immediate receival of the shutdown signal.
    // Immediate receival of packets.
    // Default select with tickrate for processing.
}