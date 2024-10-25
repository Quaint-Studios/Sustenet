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
            use $crate::utils::macros::LogLevel;
            use $crate::utils::macros::LogType;
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
                Master => "[Master]",
                Cluster => "[Cluster]",
                Client => "[Client]",
                System => "[System]",
            };

            println!("{}{} {}{}", level_str, type_str, format!($($arg)*), TERMINAL_DEFAULT);
        }
    };
}

//#region Master macros
#[macro_export]
macro_rules! master_debug {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Debug, LogType::Master, $($arg)*);
    };
}

#[macro_export]
macro_rules! master_info {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Info, LogType::Master, $($arg)*);
    };
}
#[macro_export]
macro_rules! master_warning {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Warning, LogType::Master, $($arg)*);
    };
}
#[macro_export]
macro_rules! master_error {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Error, LogType::Master, $($arg)*);
    };
}
#[macro_export]
macro_rules! master_success {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Success, LogType::Master, $($arg)*);
    };
}
//#endregion

//#region Cluster macros
#[macro_export]
macro_rules! cluster_debug {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Debug, LogType::Cluster, $($arg)*);
    };
}
#[macro_export]
macro_rules! cluster_info {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Info, LogType::Cluster, $($arg)*);
    };
}
#[macro_export]
macro_rules! cluster_warning {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Warning, LogType::Cluster, $($arg)*);
    };
}
#[macro_export]
macro_rules! cluster_error {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Error, LogType::Cluster, $($arg)*);
    };
}
#[macro_export]
macro_rules! cluster_success {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Success, LogType::Cluster, $($arg)*);
    };
}
//#endregion

//#region Client macros
#[macro_export]
macro_rules! client_debug {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Debug, LogType::Client, $($arg)*);
    };
}
#[macro_export]
macro_rules! client_info {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Info, LogType::Client, $($arg)*);
    };
}
#[macro_export]
macro_rules! client_warning {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Warning, LogType::Client, $($arg)*);
    };
}
#[macro_export]
macro_rules! client_error {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Error, LogType::Client, $($arg)*);
    };
}
#[macro_export]
macro_rules! client_success {
    ($($arg:tt)*) => {
        $crate::log_message!(LogLevel::Success, LogType::Client, $($arg)*);
    };
}
//#endregion
