use std::sync::{Arc, Mutex};

use crate::clients::client::Client;
use crate::master::master_server::MasterServer;

#[derive(Default)]
pub struct App {
    client_list: Arc<Mutex<Vec<Client>>>,
    master_server: Option<MasterServer>,
}

impl App {
    pub fn init() -> Self {
        App {
            client_list: Arc::new(Mutex::new(Vec::new())),
            master_server: None,
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
