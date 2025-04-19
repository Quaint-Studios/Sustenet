use tokio::sync::mpsc::Sender;

pub mod config;
pub mod logging;
pub mod network;
pub mod packets;
pub mod utils;

pub mod security;

pub mod macros;

pub trait Plugin: Send + Sync {
    fn receive(
        &self,
        tx: Sender<Box<[u8]>>,
        command: u8
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>>;
}

// TODO: Use a Result instead.
// #[derive(Debug)]
// pub enum PluginError {
//     Other {
//         msg: String,
//     },
// }
