use crate::{ transport::{BaseServer, ServerType}, utils::constants };

pub struct MasterServer {
    pub base: BaseServer,
}

#[derive(Debug)]
pub enum MasterServerError {
    // TODO Implement
}

// TODO Implement
impl MasterServer {
    pub async fn  new(max_connections: Option<u32>, port: Option<u16>) -> Result<Self, (MasterServerError, String)> {
        Ok(MasterServer {
            base: BaseServer::new(ServerType::MasterServer, max_connections, port).await.unwrap(),
        })
    }

    pub async fn start(&mut self) {
        self.base.start().await;
    }
}
