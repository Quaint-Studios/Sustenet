pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Success,
}

pub enum LogType {
    Master,
    Cluster,
    Client,
    System,
}

#[macro_export]
macro_rules! log_message {
    (
        $level:expr,
        $type:expr,
        $($arg:tt)*
    ) => {
        {
            use $crate::logging::LogLevel;
            use $crate::logging::LogType;
            use $crate::utils::constants::{
                TERMINAL_BLUE,
                TERMINAL_GREEN,
                TERMINAL_ORANGE,
                TERMINAL_RED,
                TERMINAL_DEFAULT,
            };

            let level_str = match $level {
                LogLevel::Debug => format!("{}[DEBUG]", TERMINAL_BLUE),
                LogLevel::Info => format!("{}[INFO]", TERMINAL_DEFAULT),
                LogLevel::Warning => format!("{}[WARNING]", TERMINAL_ORANGE),
                LogLevel::Error => format!("{}[ERROR]", TERMINAL_RED),
                LogLevel::Success => format!("{}[SUCCESS]", TERMINAL_GREEN),
            };

            let type_str = match $type {
                LogType::Master => "[Master]",
                LogType::Cluster => "[Cluster]",
                LogType::Client => "[Client]",
                LogType::System => "[System]",
            };

            println!("{}{} {}{}", level_str, type_str, format!($($arg)*), TERMINAL_DEFAULT);
        }
    };
}