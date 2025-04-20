# sustenet-cluster

`sustenet-cluster` is the cluster server crate for Sustenet's networking solution. It connects to the master server, manages game clients, and handles distributed game logic as part of a scalable cluster architecture. Each cluster server communicates with the master and other clusters to provide seamless multiplayer experiences.

Sustenet is a networking solution for game engines. It's made to primarily be used for MMO or large-scale multiplayer games in Godot Engine but can also be used in Unity and Unreal Engine. Support for other engines will continue to grow over time.

## Features

- **Master Connection**: Connects to the master server for authentication and coordination.
- **Client Management**: Handles client connections, disconnections, and data transfer within the cluster.
- **Cluster Coordination**: Works with other clusters for distributed load and seamless gameplay.
- **Configurable**: Reads settings from a TOML configuration file.
- **Logging**: Unified logging macros for debugging and monitoring.
- **Security**: Integrates with shared security primitives for encryption and key management.
- **Extensible**: Supports plugins for custom server logic via the `sustenet-shared` crate.

## Modules

- [`main.rs`](src/main.rs): Entry point for the cluster server, handles startup and main event loop.
- [`lib.rs`](src/lib.rs): Core logic for cluster operation, including master connection, client handling, and plugin integration.

## Usage

```rs
use sustenet::cluster::{ cleanup, error, start };
use sustenet::shared::Plugin;
use tokio::sync::mpsc::Sender;

struct Reia;
impl Plugin for Reia {
    fn receive(
        &self,
        tx: Sender<Box<[u8]>>,
        command: u8
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
        Box::pin(async move {
			// You can modify this with a `match` statement to be more
			// organized.

			// Sends data whenever a custom command comes in.

			// The code "20" has no significance. But the max possible code
			// is 255. If for some reason you needed more, you can always
			// mix two u8s together (i.e. [20, 1]).
            if let Err(e) = tx.send(Box::new([20])).await {
                error(format!("Failed to send message. {e}").as_str());
            }
        })
    }

    fn info(&self, message: &str) {
        println!("{message}");
    }
}

#[tokio::main]
async fn main() {
    start(Reia {}).await;
    cleanup().await;
}
```

## Configuration

The configuration file is *Config.toml*. Below is an example configuration:

```toml
[all]
server_name = "Default Cluster Server"

max_connections = 0
port = 0

[cluster]
key_name = "cluster_key"
master_ip = "127.0.0.1"
master_port = 0

domain_pub_key = "https://www.playreia.com/game/pubkey.pub" # Remove this if you want to use the server's bandwidth to send a key to a user directly. | This isn't implemented yet but will be in the future.

```

## License

This project is licensed under the MIT license.