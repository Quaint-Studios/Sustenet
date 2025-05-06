

// use mio::event::Event;
// use sustenet_shared::logging::{ LogType, Logger };
// use sustenet_shared::utils::constants::{ DEFAULT_IP, MASTER_PORT };

// use dashmap::DashMap;
// use mio::net::{ TcpListener, TcpStream };
// use mio::{ Events, Interest, Poll, Registry, Token };
// use std::collections::HashMap;
// use std::io::{ Read, Write };
// use std::net::SocketAddr;
// use std::str::from_utf8;
// use std::sync::atomic::AtomicU64;
// use std::sync::{ Arc, LazyLock, RwLock };
// use std::thread::Scope;
// use std::{ io, thread };

// pub static LOGGER: LazyLock<Logger> = LazyLock::new(|| Logger::new(LogType::Master));

// const SERVER: Token = Token(0);
// const DATA: &[u8] = b"Hello world!\n";

// pub struct ClusterInfo {
//     pub address: SocketAddr,
//     pub name: String,
// }

// pub struct MasterServer {
//     max_workers: usize,
// }

// impl MasterServer {
//     pub fn new() -> io::Result<Self> {
//         Ok(MasterServer { max_workers: 8 })
//     }

//     pub fn start(&self) -> io::Result<()> {
//         thread::scope(
//             |scope: &Scope| -> io::Result<()> {
//                 let mut poll = Arc::new(RwLock::new(Poll::new()?));
//                 let mut events = Arc::new(RwLock::new(Events::with_capacity(1024)));

//                 let addr = format!("{}:{}", DEFAULT_IP, 6258)
//                     .parse()
//                     .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
//                 let mut server = TcpListener::bind(addr)?;

//                 let next_id = AtomicU64::new(0);
//                 let connections: Arc<DashMap<Token, TcpStream>> = Arc::new(DashMap::new());
//                 let cluster_servers: Arc<DashMap<Token, ClusterInfo>> = Arc::new(DashMap::new());

//                 {
//                     let poll_guard = match poll.read() {
//                         Ok(guard) => guard,
//                         Err(e) => {
//                             return Err(
//                                 io::Error::new(io::ErrorKind::Other, "Failed to acquire poll lock")
//                             );
//                         }
//                     };
//                     poll_guard.registry().register(&mut server, SERVER, Interest::READABLE)?;
//                 }
//                 let server = Arc::new(server);

//                 // Unique token for each incoming connection.
//                 let mut unique_token = Token(SERVER.0 + 1);

//                 for _ in 0..self.max_workers {
//                     let poll = Arc::clone(&poll);
//                     let events = Arc::clone(&events);
//                     let server = Arc::clone(&server);
//                     let connections = Arc::clone(&connections);
//                     let cluster_servers = Arc::clone(&cluster_servers);

//                     scope.spawn(move || -> io::Result<()> {});
//                 }

//                 Ok(())
//             }
//         )
//     }
// }

// fn worker(poll: Arc<RwLock<Poll>>,
// events: Arc<RwLock<Events>>,
// server: Arc<TcpListener>,
// connections: Arc<DashMap<Token, TcpStream>>,
// unique_token: &mut Token
// ) -> io::Result<()> {
//     loop {
//         if let Ok(mut poll_guard) = poll.write() {
//             let mut event_guard = match events.write() {
//                 Ok(guard) => guard,
//                 Err(_e) => {
//                     return Err(
//                         io::Error::new(io::ErrorKind::Other, "Failed to acquire events lock")
//                     );
//                 }
//             };
//             if let Err(err) = poll_guard.poll(&mut *event_guard, None) {
//                 if interrupted(&err) {
//                     continue;
//                 }
//                 return Err(err);
//             }
//         }

//         for event in events.read().unwrap().iter() {
//             match event.token() {
//                 SERVER =>
//                     loop {
//                         // Received an event for the TCP server socket, which
//                         // indicates we can accept an connection.
//                         let (mut connection, address) = match server.accept() {
//                             Ok((connection, address)) => (connection, address),
//                             Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
//                                 // If we get a `WouldBlock` error we know our
//                                 // listener has no more incoming connections queued,
//                                 // so we can return to polling and wait for some
//                                 // more.
//                                 break;
//                             }
//                             Err(e) => {
//                                 // If it was any other kind of error, something went
//                                 // wrong and we terminate with an error.
//                                 return Err(e);
//                             }
//                         };

//                         println!("Accepted connection from: {}", address);

//                         let token = next(&mut unique_token);
//                         {
//                             let poll_guard = match poll.read() {
//                                 Ok(guard) => guard,
//                                 Err(_e) => {
//                                     return Err(
//                                         io::Error::new(
//                                             io::ErrorKind::Other,
//                                             "Failed to acquire poll lock"
//                                         )
//                                     );
//                                 }
//                             };
//                             poll_guard
//                                 .registry()
//                                 .register(
//                                     &mut connection,
//                                     token,
//                                     Interest::READABLE.add(Interest::WRITABLE)
//                                 )?;
//                         }

//                         connections.insert(token, connection);
//                     }
//                 token => {
//                     // Maybe received an event for a TCP connection.
//                     let done = if let Some(mut connection) = connections.get_mut(&token) {
//                         let poll_guard = match poll.read() {
//                             Ok(guard) => guard,
//                             Err(_e) => {
//                                 return Err(
//                                     io::Error::new(
//                                         io::ErrorKind::Other,
//                                         "Failed to acquire poll lock"
//                                     )
//                                 );
//                             }
//                         };
//                         handle_connection_event(poll_guard.registry(), &mut *connection, event)?
//                     } else {
//                         // Sporadic events happen, we can safely ignore them.
//                         false
//                     };
//                     if done {
//                         if let Some(mut connection) = connections.remove(&token) {
//                             let poll_guard = match poll.read() {
//                                 Ok(guard) => guard,
//                                 Err(_e) => {
//                                     return Err(
//                                         io::Error::new(
//                                             io::ErrorKind::Other,
//                                             "Failed to acquire poll lock"
//                                         )
//                                     );
//                                 }
//                             };
//                             poll_guard.registry().deregister(&mut connection.1)?;
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// /// Returns `true` if the connection is done.
// fn handle_connection_event(
//     registry: &Registry,
//     connection: &mut TcpStream,
//     event: &Event
// ) -> io::Result<bool> {
//     if event.is_writable() {
//         // We can (maybe) write to the connection.
//         match connection.write(DATA) {
//             // We want to write the entire `DATA` buffer in a single go. If we
//             // write less we'll return a short write error (same as
//             // `io::Write::write_all` does).
//             Ok(n) if n < DATA.len() => {
//                 return Err(io::ErrorKind::WriteZero.into());
//             }
//             Ok(_) => {
//                 // After we've written something we'll reregister the connection
//                 // to only respond to readable events.
//                 registry.reregister(connection, event.token(), Interest::READABLE)?;
//             }
//             // Would block "errors" are the OS's way of saying that the
//             // connection is not actually ready to perform this I/O operation.
//             Err(ref err) if would_block(err) => {}
//             // Got interrupted (how rude!), we'll try again.
//             Err(ref err) if interrupted(err) => {
//                 return handle_connection_event(registry, connection, event);
//             }
//             // Other errors we'll consider fatal.
//             Err(err) => {
//                 return Err(err);
//             }
//         }
//     }

//     if event.is_readable() {
//         let mut connection_closed = false;
//         let mut received_data = vec![0; 4096];
//         let mut bytes_read = 0;
//         // We can (maybe) read from the connection.
//         loop {
//             match connection.read(&mut received_data[bytes_read..]) {
//                 Ok(0) => {
//                     // Reading 0 bytes means the other side has closed the
//                     // connection or is done writing, then so are we.
//                     connection_closed = true;
//                     break;
//                 }
//                 Ok(n) => {
//                     bytes_read += n;
//                     if bytes_read == received_data.len() {
//                         received_data.resize(received_data.len() + 1024, 0);
//                     }
//                 }
//                 // Would block "errors" are the OS's way of saying that the
//                 // connection is not actually ready to perform this I/O operation.
//                 Err(ref err) if would_block(err) => {
//                     break;
//                 }
//                 Err(ref err) if interrupted(err) => {
//                     continue;
//                 }
//                 // Other errors we'll consider fatal.
//                 Err(err) => {
//                     return Err(err);
//                 }
//             }
//         }

//         if bytes_read != 0 {
//             let received_data = &received_data[..bytes_read];
//             if let Ok(str_buf) = from_utf8(received_data) {
//                 println!("Received data: {}", str_buf.trim_end());
//             } else {
//                 println!("Received (none UTF-8) data: {:?}", received_data);
//             }
//         }

//         if connection_closed {
//             println!("Connection closed");
//             return Ok(true);
//         }
//     }

//     Ok(false)
// }

// fn next(current: &mut Token) -> Token {
//     let next = current.0;
//     current.0 += 1;
//     Token(next)
// }

// fn would_block(err: &io::Error) -> bool {
//     err.kind() == io::ErrorKind::WouldBlock
// }

// fn interrupted(err: &io::Error) -> bool {
//     err.kind() == io::ErrorKind::Interrupted
// }
