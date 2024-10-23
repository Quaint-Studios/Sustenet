pub use self::base_client::BaseClient;
pub use self::thread_manager::ThreadManager;

mod base_client;
mod thread_manager;

pub enum Protocols {
    TCP,
    UDP,
}
