# sustenet-client

`sustenet-client` is the official client crate for Sustenet's networking solution. It connects to the master or cluster servers, manages server discovery, and handles communication for multiplayer games. The client is designed to work seamlessly with Sustenet's master and client servers, supporting scalable and secure networking for game engines.

Sustenet is a networking solution for game engines. It's made to primarily be used for MMO or large-scale multiplayer games in Godot Engine but can also be used in Unity and Unreal Engine. Support for other engines will continue to grow over time.

## Features

- **Server Discovery**: Connects to the master server to discover available clusters.
- **Cluster Connection**: Connects to cluster servers for gameplay sessions.
- **Data Transfer**: Handles sending and receiving data to/from servers.
- **Configurable**: Reads settings from a TOML configuration file.
- **Logging**: Unified logging macros for debugging and monitoring.
- **Security**: Integrates with shared security primitives for encryption and key management.

## Modules

- [`main.rs`](src/main.rs): Entry point for the client, handles startup and shutdown.
- [`lib.rs`](src/lib.rs): Core logic for client operation, including server discovery, connection management, and data transfer.

## Usage

> Plugin support coming soon.

Add `sustenet-client` to your project and call the main entry point:


```rust
use sustenet_client::{ cleanup, start };

#[tokio::main]
async fn main() {
    start().await;
    cleanup().await;
}
```

## License

This project is licensed under the MIT license.