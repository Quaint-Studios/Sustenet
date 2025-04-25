# Sustenet

> **Sustenet powers [Reia](https://www.playreia.com) — a large-scale multiplayer game and the primary showcase for this networking solution.**

Sustenet is a networking solution for game engines designed for MMOs and large-scale multiplayer games. It is built in Rust and Zig, with a focus on modularity, security, and extensibility. The primary Game Engine supported by Sustenet is current Godot. But it's flexible and can be integrated with Unity3D, Unreal Engine, and other game engines. Support for other engines will continue to grow over time.

## Features

- **Master/Cluster Architecture:** Central master server coordinates distributed cluster servers for seamless scaling.
- **Secure Communication:** AES-256-GCM encryption, key management, and base64 utilities.
- **Plugin Support:** Easily extend server logic with custom Rust plugins.
- **Unified Logging:** Consistent macros and log levels for debugging and monitoring.
- **Configurable:** TOML-based configuration for all server types.
- **Extensible Protocols:** Common event types, protocol enums, and packet definitions.

## Usage

Add `sustenet` as a dependency in your `Cargo.toml` (from crates.io):

```toml
[dependencies]
sustenet = { version = "0.1.3", features = ["shared", "cluster", "master", "client", "full"] } # Choose your features
```

Or via git:

```toml
[dependencies]
sustenet = { git = "https://github.com/Quaint-Studios/Sustenet", version = "0.1.3", features = ["shared", "cluster", "master", "client", "full"] }
```

### Example: Writing a Cluster Plugin

Below is a minimal example of a custom plugin for a cluster server, based on [`rust_example/src/main.rs`](../../rust_example/src/main.rs):

```rust
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

```

## Configuration

Each cluster server uses a `Config.toml` file for settings. Example below:

```toml
[all]
server_name = "Default Cluster Server"

max_connections = 0
port = 0

[cluster]
key_name = "cluster_key"
master_ip = "127.0.0.1"
master_port = 0

domain_pub_key = "https://site-cdn.playreia.com/game/pubkey.pub" # Remove this if you want to use the server's bandwidth to send a key to a user directly. | Currently does nothing.
```

## Modules

### auth
- [`main.rs`](../../rust/auth/src/main.rs): Entry point for the authentication server (WIP).
- [`lib.rs`](../../rust/auth/src/lib.rs): Authentication logic, planned for integration with Supabase and Turso.

### client
- [`main.rs`](../../rust/client/src/main.rs): Entry point for the client, handles startup and shutdown.
- [`lib.rs`](../../rust/client/src/lib.rs): Core logic for client operation, including server discovery, connection management, and data transfer.

### cluster
- [`main.rs`](../../rust/cluster/src/main.rs): Entry point for the cluster server, handles startup and plugin integration.
- [`lib.rs`](../../rust/cluster/src/lib.rs): Core logic for cluster operation, including master connection, client handling, and plugin support.

### master
- [`main.rs`](../../rust/master/src/main.rs): Entry point for the master server, handles startup and main event loop.
- [`lib.rs`](../../rust/master/src/lib.rs): Core logic for master server operation, including cluster and client management.
- [`security.rs`](../../rust/master/src/security.rs): Security primitives and helpers for loading keys and generating passphrases.

### shared
- [`config.rs`](../../rust/shared/src/config.rs): Handles and reads the *Config.toml* file.
- [`lib.rs`](../../rust/shared/src/lib.rs): Contains the plugin definition and other core functions.
- [`logging.rs`](../../rust/shared/src/logging.rs): Logging macros and log level/type enums.
- [`macros.rs`](../../rust/shared/src/macros.rs): Useful macros for error handling and parsing.
- [`network.rs`](../../rust/shared/src/network.rs): Protocols, events, and cluster info types.
- [`packets.rs`](../../rust/shared/src/packets.rs): Packet enums for master and cluster communication.
- [`security.rs`](../../rust/shared/src/security.rs): AES encryption, key management, and base64 helpers.
- [`utils.rs`](../../rust/shared/src/utils.rs): Constants and utility functions.

## Real-World Usage

Sustenet is actively used in [Reia](https://www.playreia.com), a large-scale multiplayer game. For more real-world examples, see [Quaint-Studios/Reia](https://github.com/Quaint-Studios/Reia).

## License

This project is licensed under the MIT license.