# sustenet-master

`sustenet-master` is the authoritative server crate for the Sustenet networking solution. It manages client connections, authentication, and communication with clusters, acting as the central coordinator for distributed game server clusters. It's designed to accept cluster servers as well as clients and routes clients to clusters.

Sustenet is a networking solution for game engines. It's made to primarily be used for MMO or large-scale multiplayer games in Godot Engine but can also be used in Unity and Unreal Engine. Support for other engines will continue to grow over time.

## Features

- **Connection Management**: Handles client connections, disconnections, and data transfer.
- **Cluster Coordination**: Manages clusters and distributes clients efficiently.
- **Configurable**: Reads settings from a TOML configuration file.
- **Logging**: Unified logging macros for debugging and monitoring.
- **Security**: Integrates with shared security primitives for encryption and key management.
- **Extensible**: Built to work with the `sustenet-shared` crate for code reuse and consistency (not implmeneted, checking for user interest)

## Modules

- [`main.rs`](src/main.rs): Entry point for the master server, handles startup and main event loop.
- [`security.rs`](src/security.rs): Security primitives and helpers for loading keys and generating passphrasess.

## Usage

`sustenet-master` is meant to be used as standalone CLI. You can build it with `cargo build --release` and running the executable in your terminal.

## Configuration

The configuration file is *Config.toml*. Below is an example configuration:

```toml
[all]
server_name = "Master Server"
max_connections = 0
port = 6256
```

## License

This project is licensed under the MIT license.