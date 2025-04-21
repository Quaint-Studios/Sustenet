# sustenet-shared

`sustenet-shared` is a Rust crate providing common utilities, configuration, networking types, security primitives, and macros for Sustenet's networking solution. It is designed to be used by the master, cluster, client, and other Sustenet components to ensure consistency and code reuse.

Sustenet is a networking solution for game engines. It's made to primarily be used for MMO or large-scale multiplayer games in Godot Engine but can also be used in Unity and Unreal Engine. Support for other engines will continue to grow over time.

## Features

- **Configuration**: Read and manage settings for master and cluster servers.
- **Networking**: Common event types, protocol enums, and cluster structures.
- **Security**: AES-256-GCM encryption/decryption, key management, and base64 utilities.
- **Logging**: Unified logging macros and log level/type definitions.
- **Macros**: Utility macros for error handling and data parsing.
- **Plugin**: Define plugins for extensible server logic.

## Modules

- [`config`](src/config.rs): Handles and reads the *Config.toml* file for master and cluster servers.
- [`logging`](src/logging.rs): Logging macros and log level/type enums.
- [`network`](src/network.rs): Protocols, events, and cluster info types.
- [`packets`](src/packets.rs): Packet enums for master and cluster communication.
- [`security`](src/security.rs): AES encryption, key management, and base64 helpers.
- [`utils`](src/utils.rs): Constants and utility functions.
- [`macros`](src/macros.rs): Useful macros for error handling and parsing.

## Usage

Add `sustenet-shared` as a dependency in your Cargo.toml (usually crates.io):

```toml
[dependencies]
sustenet = { version = "0.1.2", features = ["shared"] }
```

Or via git:
```toml
[dependencies]
sustenet = { git = "https://github.com/Quaint-Studios/Sustenet", version = "0.1.2", features = ["shared"] }
```

