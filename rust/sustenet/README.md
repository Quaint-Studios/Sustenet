# Sustenet

Sustenet is a networking solution for game engines. It's made to primarily be used for MMO or large-scale multiplayer games in Godot Engine but can also be used in Unity and Unreal Engine. Support for other engines will continue to grow over time.

## Usage

Add `sustenet` as a dependency in your Cargo.toml (usually crates.io):

```toml
[dependencies]
sustenet = { version = "0.1.0", features = ["shared", "cluster", "master", "client"] } # Choose your features
```

Or via git:
```toml
[dependencies]
sustenet = { git = "https://github.com/Quaint-Studios/Sustenet", version = "0.1.0", features = ["shared", "cluster", "master", "client"] } # Choose your features
```

