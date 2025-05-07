#[cfg(test)]
mod tests {
    use sustenet_shared::utils::constants;
    use tokio::io::AsyncWriteExt;

    use crate::MasterServer;

    const MAX_CONNS: usize = 10_000;

    // Connected 10000 clients in 7.72s (Power Save | No Turbo | 1165G7)
    #[tokio::test]
	#[ignore]
    async fn test_without_threads() {
        let mut server = MasterServer::new().await.unwrap();
        tokio::spawn(async move {
            server.start().await.unwrap();
        });

        // Simulate clients connecting
        let start = tokio::time::Instant::now();
        let addr = format!("127.0.0.1:{}", constants::MASTER_PORT);
        for i in 0..MAX_CONNS {
            match tokio::net::TcpStream::connect(&addr).await {
                Ok(mut stream) => {
                    // Simulate sending a message
                    let _ = stream.write_all(format!("Hello from client {i}").as_bytes()).await;
                }
                Err(e) => {
                    eprintln!("Failed to connect client {i}: {e}");
                }
            }
        }
        let duration = start.elapsed();
        println!("Connected {MAX_CONNS} clients in {:.2?}", duration);
    }

    // Connected 10000 clients in 3.26s (Power Save | No Turbo | 1165G7)
    #[tokio::test]
	#[ignore]
    async fn test_with_threads() {
        let mut server = MasterServer::new().await.unwrap();
        tokio::spawn(async move {
            server.start().await.unwrap();
        });

        // Simulate clients connecting
        let start = tokio::time::Instant::now();
        let mut handles = vec![];
        
        let addr: &str = "127.0.0.1:6256";
        for i in 0..MAX_CONNS {
            let handle = tokio::spawn(async move {
                match tokio::net::TcpStream::connect(addr).await {
                    Ok(mut stream) => {
                        // Simulate sending a message
                        let _ = stream.write_all(format!("Hello from client {i}").as_bytes()).await;
                    }
                    Err(e) => {
                        eprintln!("Failed to connect client {i}: {e}");
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            if let Err(e) = handle.await {
                eprintln!("Failed to join thread: {e}");
            }
        }
        let duration = start.elapsed();
        println!("Connected {MAX_CONNS} clients in {:.2?}", duration);
    }
}