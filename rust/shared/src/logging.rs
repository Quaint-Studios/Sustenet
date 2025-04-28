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

use crate::{ log_message, utils::constants::DEBUGGING };

/// Logger struct to handle logging messages with different log levels and types.
pub struct Logger {
    plugin_info: std::sync::OnceLock<Box<dyn Fn(&str) + Send + Sync + 'static>>,
    log_type: LogType,
}
impl Logger {
    /// Creates a new Logger instance with the specified log type.
    pub fn new(log_type: LogType) -> Self {
        Logger {
            plugin_info: std::sync::OnceLock::new(),
            log_type,
        }
    }

    /// Sets the plugin info function to be called when logging messages.
    pub fn set_plugin_info<F>(&self, plugin: F) where F: Fn(&str) + Send + Sync + 'static {
        let _ = self.plugin_info.set(Box::new(plugin));
    }

    /// Logs a debug message if debugging is enabled.
    pub async fn debug(&self, message: &str) {
        if !DEBUGGING {
            return;
        }
        if let Some(plugin_info) = self.plugin_info.get() {
            plugin_info(message);
        }
        log_message!(LogLevel::Debug, self.log_type, "{}", message);
    }

    /// Logs an info message.
    pub async fn info(&self, message: &str) {
        if let Some(plugin_info) = self.plugin_info.get() {
            plugin_info(message);
        }
        log_message!(LogLevel::Info, self.log_type, "{}", message);
    }

    /// Logs a warning message if debugging is enabled.
    pub async fn warning(&self, message: &str) {
        if !DEBUGGING {
            return;
        }
        if let Some(plugin_info) = self.plugin_info.get() {
            plugin_info(message);
        }
        log_message!(LogLevel::Warning, self.log_type, "{}", message);
    }

    /// Logs an error message.
    pub async fn error(&self, message: &str) {
        if let Some(plugin_info) = self.plugin_info.get() {
            plugin_info(message);
        }
        log_message!(LogLevel::Error, self.log_type, "{}", message);
    }

    /// Logs a success message.
    pub async fn success(&self, message: &str) {
        if let Some(plugin_info) = self.plugin_info.get() {
            plugin_info(message);
        }
        log_message!(LogLevel::Success, self.log_type, "{}", message);
    }
}