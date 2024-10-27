use crate::{ transport::{BaseServer, ServerType}, utils::constants };

pub struct MasterServer {
    base: BaseServer,
}

pub enum MasterServerError {
    // TODO Implement
}

// TODO Implement
impl MasterServer {
    pub async fn new() -> Result<Self, (MasterServerError, String)> {
        Ok(MasterServer {
            base: BaseServer::new(ServerType::MasterServer, 0, Some(constants::MASTER_PORT)).await.unwrap(),
        })
    }

    pub fn run(&self) {
        println!("Master server started");
    }
}
