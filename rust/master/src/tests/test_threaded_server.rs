struct TestThreadedServer {}

#[cfg(test)]
mod tests {
    use std::{ net::SocketAddr, str::FromStr };

    use mio::net::TcpStream;

    use crate::tests::test_threaded_server::run_server;

    const MAX_CONNS: u32 = 1_000_000;

    #[test]
    // Checks the speed it takes to just iterate.
    fn test_raw() {
        let start = std::time::Instant::now();
        for _ in 0..MAX_CONNS {
        }
        let duration = start.elapsed();
        println!("Time taken for {MAX_CONNS} iterations: {duration:?}");
    }

    #[test]
    // Checks the speed it takes to just iterate with threads.
    fn test_threaded() {
        let start = std::time::Instant::now();
        let max_conns = MAX_CONNS;
        let num_threads = 8;
        let chunk_size = max_conns / num_threads;

        let mut handles = vec![];

        for i in 0..num_threads {
            let start_index = i * chunk_size;
            let end_index = if i == num_threads - 1 { max_conns } else { start_index + chunk_size };

            let handle = std::thread::spawn(move || {
                for _ in start_index..end_index {
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let duration = start.elapsed();
        println!("Time taken for {MAX_CONNS} iterations with threads: {duration:?}");
    }

    #[tokio::test]
    // Checks the speed it takes to just iterate with async tasks.
    async fn test_async() {
        let start = std::time::Instant::now();
        for _ in 0..MAX_CONNS {
        }
        let duration = start.elapsed();
        println!("Time taken for {MAX_CONNS} iterations with async tasks: {duration:?}");
    }

    #[tokio::test]
    // Checks the speed it takes to just iterate with async spawns.
    async fn test_async_spawn() {
        let start = std::time::Instant::now();
        let max_conns = MAX_CONNS;
        let num_tasks = 8;
        let chunk_size = max_conns / num_tasks;

        let mut handles = vec![];

        for i in 0..num_tasks {
            let start_index = i * chunk_size;
            let end_index = if i == num_tasks - 1 { max_conns } else { start_index + chunk_size };

            let handle = tokio::spawn(async move {
                for _ in start_index..end_index {
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let duration = start.elapsed();
        println!("Time taken for {MAX_CONNS} iterations with async tasks: {duration:?}");
    }

    #[test]
    fn test_mio_million_connections() {
        // This test will run a simple Mio server that accepts connections and
        // immediately drops them. It is intended to test the performance of
        // handling a large number of connections.

        const MAX_CONNS: usize = 1_000_000;

        let server_thread = std::thread::spawn(|| {
            run_server().unwrap();
        });

        let start = std::time::Instant::now();

        let addr = SocketAddr::from_str("127.0.0.1:6258").unwrap();
        let num_threads = 8;
        let chunk_size = MAX_CONNS / num_threads;

        let mut handles = vec![];

        for i in 0..num_threads {
            let start_index = i * chunk_size;
            let end_index = if i == num_threads - 1 { MAX_CONNS } else { start_index + chunk_size };

            let addr = addr.clone();
            let handle = std::thread::spawn(move || {
                for _ in start_index..end_index {
                    let _ = TcpStream::connect(addr);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        let duration = start.elapsed();
        println!("Time taken for {MAX_CONNS} connections: {duration:?}");
    }
}

use std::error::Error;

use mio::net::{ TcpListener, TcpStream };
use mio::{ Events, Interest, Poll, Token };

// Some tokens to allow us to identify which event is for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn run_server() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:6258".parse()?;
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry().register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    let mut client = TcpStream::connect(addr)?;
    // Register the socket.
    poll.registry().register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    // Start an event loop.
    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;

        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    let connection = server.accept();
                    drop(connection);
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }
}
