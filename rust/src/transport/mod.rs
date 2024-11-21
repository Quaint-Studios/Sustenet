pub use self::base_client::BaseClient;
pub use self::thread_manager::ThreadManager;

mod base_client;
pub mod base_server;
mod thread_manager;

#[derive(Debug)]
pub enum Protocols {
    TCP,
    UDP,
}
impl From<Protocols> for String {
    fn from(protocol: Protocols) -> String {
        match protocol {
            Protocols::TCP => "TCP".to_string(),
            Protocols::UDP => "UDP".to_string(),
        }
    }
}
