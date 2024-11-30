use tokio::{
    io::{ AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader },
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::events::{ClientPackets, Event};

pub struct ServerClient {
    pub id: u32,
    pub event_sender: Sender<Event>,
}

impl ServerClient {
    pub fn new(id: u32, event_sender: Sender<Event>) -> Self {
        ServerClient { id, event_sender }
    }

    /// Handle the data from the client.
    pub async fn handle_data(&self, mut stream: TcpStream) {
        let id = self.id;
        let event_sender = self.event_sender.clone();
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let addr = stream.peer_addr().unwrap();

        tokio::spawn(async move {
            let (reader, mut writer) = stream.split();

            let mut reader = BufReader::new(reader);

            loop {
                tokio::select! {
                    // Incoming data from the client.
                    command = reader.read_u8() => {
                        println!("SC Received: {:?}", command);

                        if command.is_err() {
                            break;
                        }

                        match ClientPackets::from_u8(command.unwrap()) {
                            ClientPackets::RequestClusterServers => todo!(),
                            ClientPackets::Message => {
                                let len = reader.read_u8().await.unwrap();
                                let mut msg = vec![0; len as usize];
                                reader.read_exact(&mut msg).await.unwrap();
                                let msg_str = String::from_utf8(msg).unwrap();
                                println!("SC Message Received: {:?}", msg_str);
                            },
                            ClientPackets::Login => todo!(),
                            ClientPackets::StartUDP => todo!(),
                            ClientPackets::JoinCluster => todo!(),
                            ClientPackets::LeaveCluster => todo!(),
                            ClientPackets::MoveTo => todo!(),
                            ClientPackets::Error => {
                                println!("SC Error");
                            },
                        }

                        if 0 == 12 { // Serves no purpose. Just temporary.
                            tx.send([0]).await.unwrap();
                        }
                    }
                    // Outgoing data to the client.
                    result = rx.recv() => {
                        println!("SC Sending: {:?}", result);
                        // Write the message to the writer.
                        let msg = result.unwrap();

                        writer.write_all(&msg).await.unwrap();
                        writer.flush().await.unwrap();
                    }
                }
            }
        });
    }
}
