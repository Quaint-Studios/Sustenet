# Sustenet
A Rust & Zig, formerly C#, networking solution for the Godot Engine, Unreal Engine, and Unity3D. The primary focus is to enable scaling by allowing multiple servers to work together.

[![Sustenet (Rust)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-rust.yml/badge.svg)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-rust.yml) [![Sustenet (Zig)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-zig.yml/badge.svg)](https://github.com/Quaint-Studios/Sustenet/actions/workflows/sustenet-zig.yml) [![CodeFactor](https://www.codefactor.io/repository/github/quaint-studios/sustenet/badge)](https://www.codefactor.io/repository/github/quaint-studios/sustenet)

> **Sustenet powers [Reia](https://www.playreia.com) — a large-scale multiplayer game and the primary showcase for this networking solution.**

Sustenet is a networking solution for game engines designed for MMOs and large-scale multiplayer games. It is built in Rust and Zig, with a focus on modularity, security, and extensibility. The primary Game Engine supported by Sustenet is current Godot. But it's flexible and can be integrated with Unity3D, Unreal Engine, and other game engines. Support for other engines will continue to grow over time.

## Collaboration
While I am still in the process of designing the structure of this project, I will not be actively accepting any collaborative efforts via pull requests. If there's an open issue, feel free to contribute. I'll work with you through it. I am also open to being pointed in certain directions. Articles and documentation on specific issues are greatly appreciated. Even discussing it directly is welcome. If you're interested in that, feel free to join [Reia's Discord](https://discord.playreia.com). You can discuss more about it there. We're very close to getting Sustenet to a point where it can actively accept contributions daily!

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
sustenet = { version = "0.1.2", features = ["shared", "cluster", "master", "client", "full"] } # Choose your features
```

Or via git:

```toml
[dependencies]
sustenet = { git = "https://github.com/Quaint-Studios/Sustenet", version = "0.1.2", features = ["shared", "cluster", "master", "client", "full"] }
```

### Example: Writing a Cluster Plugin

Below is a minimal example of a custom plugin for a cluster server, based on [`rust_example/src/main.rs`](rust_example/src/main.rs):

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
- [`main.rs`](rust/auth/src/main.rs): Entry point for the authentication server (WIP).
- [`lib.rs`](rust/auth/src/lib.rs): Authentication logic, planned for integration with Supabase and Turso.

### client
- [`main.rs`](rust/client/src/main.rs): Entry point for the client, handles startup and shutdown.
- [`lib.rs`](rust/client/src/lib.rs): Core logic for client operation, including server discovery, connection management, and data transfer.

### cluster
- [`main.rs`](rust/cluster/src/main.rs): Entry point for the cluster server, handles startup and plugin integration.
- [`lib.rs`](rust/cluster/src/lib.rs): Core logic for cluster operation, including master connection, client handling, and plugin support.

### master
- [`main.rs`](rust/master/src/main.rs): Entry point for the master server, handles startup and main event loop.
- [`lib.rs`](rust/master/src/lib.rs): Core logic for master server operation, including cluster and client management.
- [`security.rs`](rust/master/src/security.rs): Security primitives and helpers for loading keys and generating passphrases.

### shared
- [`config.rs`](rust/shared/src/config.rs): Handles and reads the *Config.toml* file.
- [`lib.rs`](rust/shared/src/lib.rs): Contains the plugin definition and other core functions.
- [`logging.rs`](rust/shared/src/logging.rs): Logging macros and log level/type enums.
- [`macros.rs`](rust/shared/src/macros.rs): Useful macros for error handling and parsing.
- [`network.rs`](rust/shared/src/network.rs): Protocols, events, and cluster info types.
- [`packets.rs`](rust/shared/src/packets.rs): Packet enums for master and cluster communication.
- [`security.rs`](rust/shared/src/security.rs): AES encryption, key management, and base64 helpers.
- [`utils.rs`](rust/shared/src/utils.rs): Constants and utility functions.

## Real-World Usage

Sustenet is actively used in [Reia](https://www.playreia.com), a large-scale multiplayer game. For more real-world examples, see [Quaint-Studios/Reia](https://github.com/Quaint-Studios/Reia).

## License

This project is licensed under the MIT license.

# Old README

> Note: This section of the README was out of date since we were still migrating from C# to Rust/Zig. Things like solutions and project files no longer exist. Regardless, the layout should remain the same for everything. It's left here to get a good understanding of what we're trying to accomplish.

## Vision

*This is a rough vision of where this project is headed, a more detailed layout will eventually be added.*

The goal for Sustenet is to develop a connetion of servers. There are four major components in Sustenet.

- The `Master` server is where all clusters go to be registered. There should really only be one Master Server. But I can't stop you if you want to do something more with it.
- The `Cluster` would be considered your traditional server. You load Sustenet as a Cluster and it contains some configurations:
    - `Key` - The Cluster has a key in it, much like the SSH key you place on a server. Each Cluster should have a unique key. But, like I said, I can't stop you. You can reuse keys if you'd like. Just be aware that if one key is compromised, they all are. I will need some more research on how much security is required in an instance like this. Or what other approaches are an option.
    - `Master Server IP` - This is just telling the Cluster what IP to register to.
    - `[Master Server Port = 6256]` - Again, just some information to properly connect to the Master Server.
    - `[Connection Limit = 0]` - This is an optional property. Since it's set to 0, no connection limit is being enforced.
    - *more soon...*
- The `Fragment` is used to give different settings to certain areas in a Cluster. This includes the size of the Fragment in-game, the amount of players in it before the settings might change, keeping track of which players are in this Fragment, and update-rates.
- The `Client` is simply a Client. They'll connect to the Master server and have two options:
    - Login immediately, joining whatever Cluster they've been automatically allocated to, based on how much traffic the other Clusters are experiencing or based on their ping.
    - Manaully select a cluster, browsing their names and other information. If there's a connection limit, then lock access to join that server.

    That's it. After that, they'll just send and receive messages from their Cluster and the Cluster will handle swapping the player between Fragments based on their position.

Sustenet is aiming to improve this methodology over time. It's a learning experience. The structure may change over time. This will be the route taken for now though.

## Building & Testing
Here's a little context on the structure. There are two solutions. `Sustenet` and `SustenetUnity`. The former generates an executable while the latter generates a library. `SustenetUnity` also excludes all files related to the master server.

Inside of the SustenetUnity.csproj file under PostBuild is a line that says `ImplementationPath`. That is the path you want the library to be automatically exported to. It's advised to change it to something valid. Other than that, everything should work as is.

## Testing with no GUI
You can run the Sustenet.exe by itself with the parameter --master (this is the default options, so you don't actually have to provide it) and --client in two separate programs. This will show you an example connection.