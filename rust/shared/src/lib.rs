//! This crate contains the shared code for the master server, cluster server, and client.

use std::{ future::Future, pin::Pin };

use tokio::{ io::BufReader, net::tcp::ReadHalf, sync::mpsc::Sender };

pub mod config;
pub mod logging;
pub mod network;
pub mod packets;
pub mod utils;

pub mod security;

pub mod macros;

pub type SenderBox = Sender<Box<[u8]>>;
pub type PluginPin<'plug> = Pin<Box<dyn Future<Output = ()> + Send + 'plug>>;

pub trait ServerPlugin: Send + Sync {
    fn set_sender(&self, tx: SenderBox);

    fn receive<'plug>(
        &self,
        tx: SenderBox,
        command: u8,
        reader: &'plug mut BufReader<ReadHalf<'_>>
    ) -> PluginPin<'plug>;

    /// Only used when debugging is enabled.
    fn info(&self, message: &str);
}

pub trait ClientPlugin: Send + Sync {
    fn set_sender(&self, tx: SenderBox);

    fn receive_master<'plug>(
        &self,
        tx: SenderBox,
        command: u8,
        reader: &'plug mut BufReader<ReadHalf<'_>>
    ) -> PluginPin<'plug>;

    fn receive_cluster<'plug>(
        &self,
        tx: SenderBox,
        command: u8,
        reader: &'plug mut BufReader<ReadHalf<'_>>
    ) -> PluginPin<'plug>;

    /// Only used when debugging is enabled.
    fn info(&self, message: &str);
}

// TODO: Use a Result instead.
// #[derive(Debug)]
// pub enum PluginError {
//     Other {
//         msg: String,
//     },
// }
