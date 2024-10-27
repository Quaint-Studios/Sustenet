pub use self::base_client::BaseClient;
pub use self::base_server::{BaseServer, ServerType};
pub use self::thread_manager::ThreadManager;

mod base_client;
mod base_server;
mod thread_manager;

pub enum Protocols {
    TCP,
    UDP,
}
