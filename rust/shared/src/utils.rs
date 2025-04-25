/// Create a channel to listen for shutdown signals.
pub fn shutdown_channel() -> Result<tokio::sync::broadcast::Receiver<bool>, ctrlc::Error> {
    let (tx, rx) = tokio::sync::broadcast::channel::<bool>(1);

    // Handle shutdowns gracefully.
    ctrlc
        ::set_handler(move || {
            tx.send(true).unwrap();
        })
        .expect("Error setting Ctrl-C handler");

    Ok(rx)
}

pub mod constants {
    pub const VERSION: &str = "0.1.3";

    pub const DEBUGGING: bool = cfg!(debug_assertions);

    /// How many ticks are in a second.
    pub const TICK_RATE: i32 = 30;
    pub const MS_PER_TICK: u64 = 1000 / (TICK_RATE as u64);

    pub const DEFAULT_IP: &str = "127.0.0.1";
    pub const MASTER_PORT: u16 = 6256;
    pub const CLUSTER_PORT: u16 = 6257;

    pub const TERMINAL_BG_GRAY: &str = "\x1b[47m";
    pub const TERMINAL_DEFAULT: &str = "\x1b[39m";
    pub const TERMINAL_BLACK: &str = "\x1b[30m";
    pub const TERMINAL_WHITE: &str = "\x1b[97m";
    pub const TERMINAL_RED: &str = "\x1b[91m";
    pub const TERMINAL_GREEN: &str = "\x1b[92m";
    pub const TERMINAL_BLUE: &str = "\x1b[94m";
    pub const TERMINAL_ORANGE: &str = "\x1b[93m";
}
