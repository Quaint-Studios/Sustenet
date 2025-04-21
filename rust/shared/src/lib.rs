use tokio::sync::mpsc::Sender;

pub mod config;
pub mod logging;
pub mod network;
pub mod packets;
pub mod utils;

pub mod security;

pub mod macros;

pub trait Plugin: Send + Sync {
    fn set_sender(&self, tx: Sender<Box<[u8]>>);

    fn receive<'plug>(
        &self,
        tx: Sender<Box<[u8]>>,
        command: u8,
        reader: &'plug mut tokio::io::BufReader<tokio::net::tcp::ReadHalf<'_>>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'plug>>;

    fn info(&self, message: &str);
}

// TODO: Use a Result instead.
// #[derive(Debug)]
// pub enum PluginError {
//     Other {
//         msg: String,
//     },
// }
