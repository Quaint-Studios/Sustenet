use tokio::{
    io::{ AsyncBufReadExt, AsyncWriteExt, BufReader },
    net::TcpStream,
    sync::mpsc::Sender,
};

use crate::events::Event;

pub struct ServerClient {
    pub id: u32,
    pub stream: TcpStream,
    pub event_sender: Sender<Event>,
}

impl ServerClient {
    pub fn new(id: u32, stream: TcpStream, event_sender: Sender<Event>) -> Self {
        ServerClient {
            id,
            stream,
            event_sender,
        }
    }

    /// Handle the data from the client.
    pub async fn handle_data(&mut self) {
        let (reader, mut _writer) = self.stream.split();

        let mut reader = BufReader::new(reader);
        let mut line = String::new();

        loop {
            tokio::select! {
                result = reader.read_line(&mut line) => {
                    // Break if the line is empty.
                    if result.unwrap() == 0 {
                        break;
                    }

                    // Read the first byte of the line to get the command.
                    let _command = line.as_bytes()[0];

                    // Handle the command. Match it with the Event.
                    match self.event_sender.send(Event::Connection(self.id)).await {
                        Ok(_) => {},
                        Err(e) => {
                            println!("Error sending connection event: {:?}", e);
                        }
                    }

                    line.clear();
                }
                // result = rx.recv() => {
                //     // Write the message to the writer.
                //     let (msg, msg_addr) = result.unwrap();

                //     if addr != msg_addr {
                //         writer.write_all(&msg.as_bytes()).await.unwrap();
                //     }
                // }
            }
        }
    }
}
