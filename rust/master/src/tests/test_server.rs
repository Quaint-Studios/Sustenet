use dashmap::DashMap;
use std::sync::Arc;
use sustenet_shared::{ lselect, utils::constants::{ DEFAULT_IP, MASTER_PORT } };
use tokio::net::{ TcpListener, TcpStream };

pub struct TestServer {
    connections: Arc<DashMap<u64, TcpStream>>,
}
impl TestServer {
    pub fn new() -> Self {
        TestServer {
            connections: Arc::new(DashMap::new()),
        }
    }

    pub async fn start(&self, max_conns: u32) -> std::io::Result<tokio::time::Duration> {
        let addr = format!("{}:{}", DEFAULT_IP, MASTER_PORT + 2);
        let listener = TcpListener::bind(&addr).await?;

        println!("TestServer listening on {addr}");
        let duration = match tokio::spawn(Self::handler(max_conns, listener, self.connections.clone())).await {
			Ok(result) => match result {
				Ok(duration) => duration,
				Err(e) => {
					println!("Handler task failed: {e}");
					return Err(std::io::Error::new(std::io::ErrorKind::Other, "Handler task failed"));
				}
			},
			Err(e) => {
				println!("Failed to spawn handler task: {e}");
				return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to spawn handler task"));
			}
		};
		println!("TestServer closed.");

        Ok(duration)
    }

    async fn handler(max_conns: u32, listener: TcpListener, connections: Arc<DashMap<u64, TcpStream>>) -> tokio::io::Result<tokio::time::Duration> {
		let mut start_time: tokio::time::Instant = tokio::time::Instant::now();
		let mut next_id: u64 = 0;

        lselect!(
            pair = listener.accept() => {
                match pair {
                    Ok((stream, _peer)) => {
						if next_id == 0 {
							println!("First connection accepted.");
							start_time = tokio::time::Instant::now();
						}

                        connections.insert(next_id, stream);
                        next_id += 1;

						if next_id % 10_000 == 0 {
							println!("Accepted {} connections", next_id);
						}
						
						// Test: When next_id exceeds max_conns, we can stop accepting new connections
						if next_id >= max_conns as u64 {
							println!("Max connections reached, stopping acceptance of new connections.");
							return Ok(start_time.elapsed());
						}
                    }
                    Err(_e) => {}
                }
            }
			_ = tokio::time::sleep(tokio::time::Duration::from_millis(20)) => {
				// Loop all connections and do nothing.
				for entry in connections.iter() {
					let (_id, stream) = entry.pair();
					// Read from stream without blocking
					let mut buffer = [0; 1024];
					if let Ok(bytes_read) = stream.try_read(&mut buffer) {
						if bytes_read == 0 {
							// connections.remove(&_id);
						} else {

						}
					}
				}
			}
        )


    }
}

#[cfg(test)]
mod tests {
	#[tokio::test]
	async fn test_server_accepts_connections() {
		const MAX_CONNS: u32 = 40_000;

		let server = super::TestServer::new();
		let duration = server.start(MAX_CONNS).await.unwrap();
		println!("TestServer accepted {} connections in {:.2?}", MAX_CONNS, duration);
	}
}

