pub const MAX_CONNS: usize = 10_000;
pub const MAX_THREADS: usize = 8;
pub const ADDR: &str = "0.0.0.0:6258";

/// Creates connections to a TCP listener based on the number of connections specified in the CLI.
pub fn test_create_connections() {
    if std::env::args().any(|arg| (arg == "--help" || arg == "-h")) {
        println!("Usage: test_create_connections [options]");
        println!("-d | --dest <IP:port> to specify the destination address (default: {})", ADDR);
        println!("-c | --conns <number> to specify the number of connections (default: {})", MAX_CONNS);
        println!("-t | --threads <number> to specify the number of threads (default: {})", MAX_THREADS);
        return;
    }
    println!("Add --help or -h for usage information.");
    let mut addr = ADDR.to_string();
    let mut max_conns = MAX_CONNS;
    let mut max_threads = MAX_THREADS;
    let mut args = std::env::args().peekable();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-d" | "--dest" => {
                if let Some(val) = args.next() {
                    addr = val;
                }
            }
            "-c" | "--conns" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        if num > 0 {
                            max_conns = num;
                        } else {
                            eprintln!("Number of connections must be greater than 0.");
                        }
                    }
                }
            }
            "-t" | "--threads" => {
                if let Some(val) = args.next() {
                    if let Ok(num) = val.parse::<usize>() {
                        if num > 0 {
                            max_threads = num;
                        } else {
                            eprintln!("Number of threads must be greater than 0.");
                        }
                    }
                }
            }
            _ => {}
        }
    }
    let mut handles = vec![];
    println!("Starting to create {} connections to {}", max_conns, addr);
    println!("Press Enter to begin...");
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let start = std::time::Instant::now();

    let connections_per_thread = max_conns / max_threads;
    let remainder = max_conns % max_threads;
    // Break it up over the threads
    for i in 0..max_threads {
        let addr_clone = addr.clone();
        let num_conns = if i < remainder {
            connections_per_thread + 1
        } else {
            connections_per_thread
        };
        let handle = std::thread::spawn(move || {
            for _ in 0..num_conns {
                match std::net::TcpStream::connect(&addr_clone) {
                    Ok(mut _stream) => {
                        // Optionally, send a message to the server
                        // let _ = stream.write_all(b"Hello from client").unwrap();
                    }
                    Err(e) => {
                        eprintln!("Failed to connect: {}", e);
                    }
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
    let duration = start.elapsed();
    println!("Created {} connections in {:?}", max_conns, duration);
}

#[cfg(test)]
pub mod tests {
    // use std::io::Write;

    use std::sync::atomic::AtomicUsize;

    use super::{ ADDR, MAX_CONNS, MAX_THREADS };

    /// Tests the time it takes to accept a fixed number of TCP connections.
    #[test]
    pub fn test_tcplistener_default() {
        let server = std::net::TcpListener::bind(ADDR).unwrap();
        println!("TCP listener bound to {}", ADDR);
        // Accept the first connection, then start timing
        let _ = server.accept().unwrap();
        println!("First connection accepted, starting timer...");
        let start = std::time::Instant::now();
        for _ in 1..MAX_CONNS {
            let _ = server.accept().unwrap();
        }
        let duration = start.elapsed();
        println!("Time taken to accept {} connections: {:?}", MAX_CONNS, duration);
    }

    /// Tests the time it takes to accept a fixed number of TCP connections with tokio.
    #[tokio::test]
    #[ignore]
    async fn test_tcplistener_tokio() {
        let server = tokio::net::TcpListener::bind(ADDR).await.unwrap();
        println!("Tokio TCP listener bound to {}", ADDR);
        // Accept the first connection, then start timing
        let _ = server.accept().await.unwrap();
        println!("First connection accepted, starting timer...");
        let start = tokio::time::Instant::now();
        for _ in 1..MAX_CONNS {
            let _ = server.accept().await.unwrap();
        }
        let duration = start.elapsed();
        println!("Time taken to accept {} connections with Tokio: {:?}", MAX_CONNS, duration);
    }

    /// Tests the time it takes to accept a fixed number of TCP connections with threads.
    #[test]
    fn test_tcplistener_threads() {
        let server = std::sync::Arc::new(std::net::TcpListener::bind(ADDR).unwrap());
        println!("Tokio TCP listener bound to {}", ADDR);

        // Accept the first connection, then start timing
        let _ = server.accept().unwrap();
        println!("First connection accepted, starting timer...");

        let start = std::time::Instant::now();
        let mut handles = vec![];

        let connections_per_thread = MAX_CONNS / MAX_THREADS;
        let remainder = MAX_CONNS % MAX_THREADS;
        let curr: std::sync::Arc<AtomicUsize> = std::sync::Arc::new(AtomicUsize::new(0));
        // Break it up over the threads
        for i in 0..MAX_THREADS {
            let curr_clone = curr.clone();
            let server_clone = server.clone();
            let num_conns = if i < remainder {
                connections_per_thread + 1
            } else {
                connections_per_thread
            };
            let handle = std::thread::spawn(move || {
                println!("Thread {} accepting {} connections...", i, num_conns);

                for _ in 0..num_conns {
                    let _ = server_clone.accept();

                    curr_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                    // If Curr is a multiple of 100, print progress
                    if curr_clone.load(std::sync::atomic::Ordering::SeqCst) % 100 == 0 {
                        println!(
                            "Accepted {} connections so far...",
                            curr_clone.load(std::sync::atomic::Ordering::SeqCst)
                        );
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration = start.elapsed();
        println!("Time taken to accept {} connections with threads: {:?}", MAX_CONNS, duration);
    }

    /// Tests the time it takes to accept a fixed number of TCP connections with threads and tokio.
    #[tokio::test]
    #[ignore]
    async fn test_tcplistener_threads_tokio() {
        let server = std::sync::Arc::new(tokio::net::TcpListener::bind(ADDR).await.unwrap());
        println!("Tokio TCP listener bound to {}", ADDR);

        // Accept the first connection, then start timing
        let _ = server.accept().await.unwrap();
        println!("First connection accepted, starting timer...");

        let start = tokio::time::Instant::now();
        let mut handles = vec![];

        let connections_per_thread = MAX_CONNS / MAX_THREADS;
        let remainder = MAX_CONNS % MAX_THREADS;
        let curr: std::sync::Arc<AtomicUsize> = std::sync::Arc::new(AtomicUsize::new(0));
        // Break it up over the threads
        for i in 0..MAX_THREADS {
            let curr_clone = curr.clone();
            let server_clone = server.clone();
            let num_conns = if i < remainder {
                connections_per_thread + 1usize
            } else {
                connections_per_thread
            };
            let handle = tokio::spawn(async move {
                for _ in 0..num_conns {
                    let _ = server_clone.accept().await;
                    curr_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                    // If Curr is a multiple of 100, print progress
                    if curr_clone.load(std::sync::atomic::Ordering::SeqCst) % 100 == 0 {
                        println!(
                            "Accepted {} connections so far...",
                            curr_clone.load(std::sync::atomic::Ordering::SeqCst)
                        );
                    }
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration = start.elapsed();
        println!(
            "Time taken to accept {} connections with threads and Tokio: {:?}",
            MAX_CONNS,
            duration
        );
    }
}
