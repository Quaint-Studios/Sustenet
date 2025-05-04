#[cfg(all(test, not(feature = "ignored_tests")))]
mod tests {
    use std::time::{ Duration, Instant };

    use tokio::{ io::{ AsyncReadExt, AsyncWriteExt }, net::TcpStream };

    mod normal {
        use tokio::io::{ AsyncReadExt, AsyncWriteExt };
        use tokio::net::TcpListener;

        #[tokio::main]
        async fn main() {
            // Bind the TCP listener to a specific address and port
            let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

            println!("Server running on 127.0.0.1:8080");

            loop {
                // Accept an incoming connection
                let (mut socket, addr) = listener
                    .accept().await
                    .expect("Failed to accept connection");
                // println!("New connection from {}", addr);

                // Spawn a new task to handle the client connection
                tokio::spawn(async move {
                    let mut buffer = [0; 1024];

                    loop {
                        // Read data from the client
                        match socket.read(&mut buffer).await {
                            Ok(0) => {
                                // Connection was closed
                                println!("Connection closed by {}", addr);
                                return;
                            }
                            Ok(n) => {
                                // Echo the data back to the client
                                if let Err(e) = socket.write_all(&buffer[..n]).await {
                                    eprintln!("Failed to write to socket; err = {:?}", e);
                                    return;
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to read from socket; err = {:?}", e);
                                return;
                            }
                        }
                    }
                });
            }
        }
    }

    mod tick {
        use std::collections::HashMap;
        use std::sync::Arc;
        use tokio::io::{ AsyncReadExt, AsyncWriteExt };
        use tokio::net::TcpListener;
        use tokio::sync::{ Mutex, broadcast };
        use tokio::time::{ self, Duration };

        pub async fn main() {
            // Bind the TCP listener to a specific address and port
            let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind");

            println!("Server running on 127.0.0.1:8080");

            // Create a broadcast channel for ticks
            let (tick_tx, _) = broadcast::channel::<()>(100);

            // Shared map of connections
            let connections: Arc<Mutex<HashMap<u64, tokio::net::TcpStream>>> = Arc::new(
                Mutex::new(HashMap::new())
            );

            // Clone the connections map for the accept loop
            let connections_clone = Arc::clone(&connections);

            // Spawn a task to accept connections
            tokio::spawn(async move {
                let mut next_id: u64 = 0;

                loop {
                    // Accept an incoming connection
                    match listener.accept().await {
                        Ok((socket, addr)) => {
                            // println!("New connection from {}", addr);

                            // Store the connection in the HashMap
                            {
                                let mut connections = connections_clone.lock().await;
                                connections.insert(next_id, socket);
                            }
                            next_id += 1;
                        }
                        Err(e) => {
                            // eprintln!("Failed to accept connection: {:?}", e);
                        }
                    }
                }
            });

            // Clone the connections map for the tick-based system
            let connections_clone = Arc::clone(&connections);

            // Spawn the tick-based system
            tokio::spawn(async move {
                let mut interval = time::interval(Duration::from_millis(100)); // 10 ticks per second
                loop {
                    interval.tick().await;

                    // Send a tick event to all listeners
                    if tick_tx.send(()).is_err() {
                        // println!("No active listeners. Continuing...");
                    }

                    // Read from all connections
                    let mut to_remove = Vec::new();
                    {
                        let mut connections = connections_clone.lock().await;
                        for (id, stream) in connections.iter_mut() {
                            let mut buffer = [0; 1024];
                            match stream.read(&mut buffer).await {
                                Ok(0) => {
                                    // Connection was closed
                                    // println!("Connection {} closed", id);
                                    to_remove.push(*id);
                                }
                                Ok(n) => {
                                    // Echo the data back to the client
                                    if let Err(e) = stream.write_all(&buffer[..n]).await {
                                        eprintln!(
                                            "Failed to write to connection {}; err = {:?}",
                                            id,
                                            e
                                        );
                                        to_remove.push(*id);
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Failed to read from connection {}; err = {:?}",
                                        id,
                                        e
                                    );
                                    to_remove.push(*id);
                                }
                            }
                        }

                        // Remove closed connections
                        for id in to_remove {
                            connections.remove(&id);
                        }
                    }
                }
            });

            // Prevent the main task from exiting
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }

    #[tokio::test]
    async fn test_stress_server() {
        tokio::spawn(tick::main());

        const CLIENT_COUNT: usize = 10000; // Number of simulated clients
        const TEST_MAX_DURATION: Duration = Duration::from_secs(10);

        let start_time = Instant::now();
        for i in 0..CLIENT_COUNT {
            let mut stream = TcpStream::connect("127.0.0.1:8080").await.expect("Failed to connect");
            // let message = format!("Client {}: Hello, server!", i);
            // stream.write_all(message.as_bytes()).await.expect("Failed to send data");
            // let mut buffer = [0; 1024];
            // let _ = stream.read(&mut buffer).await; // Read response (echo)
        }
        let elapsed_time = start_time.elapsed();
        println!("Stress test completed in {:.2?} seconds", elapsed_time);

        assert!(elapsed_time <= TEST_MAX_DURATION, "Test ran longer than expected!");
    }
}
