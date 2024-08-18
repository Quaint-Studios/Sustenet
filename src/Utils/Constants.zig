pub const VERSION = "0.1.0";

pub const DEBUGGING = false;

/// How many ticks are in a second.
pub const TICK_RATE: i32 = 30;
pub const MS_PER_TICK = 1000 / TICK_RATE;

pub const DEFAULT_IP = "127.0.0.1";
pub const MASTER_PORT: i16 = 6256;
pub const CLUSTER_PORT: i16 = 6257;

pub const TERMINAL_BG_GRAY = "\x1b[47m";
pub const TERMINAL_DEFAULT = "\x1b[39m";
pub const TERMINAL_BLACK = "\x1b[30m";
pub const TERMINAL_WHITE = "\x1b[97m";
pub const TERMINAL_RED = "\x1b[91m";
pub const TERMINAL_GREEN = "\x1b[92m";
pub const TERMINAL_BLUE = "\x1b[94m";
pub const TERMINAL_ORANGE = "\x1b[93m";
