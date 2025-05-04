#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytes::Bytes;
    use tokio::time::Instant;

    struct Connection {
        _sender: tokio::sync::mpsc::Sender<Bytes>,
    }

    trait Server {
        fn add_connection(&mut self) -> u64;
        fn remove_connection(&mut self, id: u64) -> bool;
        fn get_connection_by_id(&self, id: u64) -> Option<&Connection>;
        fn get_connections(&self) -> &HashMap<u64, Connection>;
        fn next_id(&self) -> u64;
    }

    struct MasterServer {
        connections: HashMap<u64, Connection>,
        free_list: Vec<u64>, // Tracks free IDs for reuse
        next_id: u64, // The next ID for new connections
    }

    impl MasterServer {
        fn new() -> Self {
            Self {
                connections: HashMap::new(),
                free_list: Vec::new(),
                next_id: 0,
            }
        }

    }
    impl Server for MasterServer {
        fn next_id(&self) -> u64 {
            self.next_id
        }

        fn add_connection(&mut self) -> u64 {
            let id = if let Some(reused_id) = self.free_list.pop() {
                // Reuse an ID from the free list
                reused_id
            } else {
                // Assign the next available ID
                let id = self.next_id;
                self.next_id += 1;
                id
            };

            // Create a new channel for the connection
            let (_sender, _receiver) = tokio::sync::mpsc::channel(100); // Example channel with a buffer size of 100

            // Add the connection
            self.connections.insert(id, Connection { _sender });
            id
        }

        fn remove_connection(&mut self, id: u64) -> bool {
            if self.connections.remove(&id).is_some() {
                self.free_list.push(id); // Mark the ID as reusable
                return true;
            }
            false
        }

        fn get_connection_by_id(&self, id: u64) -> Option<&Connection> {
            self.connections.get(&id)
        }

        fn get_connections(&self) -> &HashMap<u64, Connection> {
            &self.connections
        }
    }

    struct MasterServerNoFree {
        connections: HashMap<u64, Connection>,
        next_id: u64, // The next ID for new connections
    }

    impl MasterServerNoFree {
        fn new() -> Self {
            Self {
                connections: HashMap::new(),
                next_id: 0,
            }
        }
    }
    impl Server for MasterServerNoFree {
        fn next_id(&self) -> u64 {
            self.next_id
        }

        fn add_connection(&mut self) -> u64 {
            // Create a new channel for the connection
            let (_sender, _receiver) = tokio::sync::mpsc::channel(100); // Example channel with a buffer size of 100

            // Add the connection
            self.connections.insert(self.next_id, Connection { _sender });
            let id = self.next_id;
            self.next_id += 1;
            id
        }

        fn remove_connection(&mut self, id: u64) -> bool {
            if self.connections.remove(&id).is_none() {
                return false;
            }
            true
        }

        fn get_connection_by_id(&self, id: u64) -> Option<&Connection> {
            self.connections.get(&id)
        }

        fn get_connections(&self) -> &HashMap<u64, Connection> {
            &self.connections
        }
    }

    async fn add_nums(num1: usize, num2: usize) -> usize {
        num1 + num2
    }

    #[tokio::test]
    #[ignore]
    async fn tokio_spawn_speed_test() {
        const MAX_ITERS: usize = 1_000_000;

        // The goal is to compare 3 different implementations.
        // 1. No spawning.
        // 2. Spawning only the whole loop.
        // 3. Spawning each iteration.
        let start_time = Instant::now();
        let mut res = 0;
        for _ in 0..MAX_ITERS {
            // Simulate some work
            res = add_nums(res, 1).await;
        }
        let elapsed = start_time.elapsed();
        println!("Time for no spawning: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);

        let start_time = Instant::now();
        let mut res = 0;
        tokio::spawn(async move {
            for _ in 0..MAX_ITERS {
                res = add_nums(res, 1).await;
            }
        }).await.unwrap();
        let elapsed = start_time.elapsed();
        println!("Time for spawning loop: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);

        let start_time = Instant::now();
        // Spawn the each iteration
        let mut handles = Vec::new();
        let mut res = 0;
        for _ in 0..MAX_ITERS {
            let handle = tokio::spawn(async move {
                res = add_nums(res, 1).await;
            });
            handles.push(handle);
        }
        for handle in handles {
            let _ = handle.await.unwrap();
        }
        println!("{res}");
        let elapsed = start_time.elapsed();
        println!("Time for spawning each iteration: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);
    }

    #[tokio::test]
    #[ignore]
    async fn test_add_connection() {
        let mut server = MasterServer::new();

        // Add connections and verify they are added
        let id1 = server.add_connection();
        let id2 = server.add_connection();
        let id3 = server.add_connection();

        assert!(server.get_connection_by_id(id1).is_some());
        assert!(server.get_connection_by_id(id2).is_some());
        assert!(server.get_connection_by_id(id3).is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_remove_connection() {
        let mut server = MasterServer::new();

        // Add connections
        let id1 = server.add_connection();
        let id2 = server.add_connection();
        let id3 = server.add_connection();

        // Remove a connection and verify it is removed
        let removed_id = id2;
        let removed = server.remove_connection(removed_id);
        assert!(removed);
        assert!(server.get_connection_by_id(removed_id).is_none());

        // Verify that other connections are unaffected
        assert!(server.get_connection_by_id(id1).is_some());
        assert!(server.get_connection_by_id(id3).is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn test_performance_at_scale() {
        const MAX_CONNS: usize = 1_000_000;

        let mut server = MasterServer::new();

        // Performance test: adding connections
        let start_time = Instant::now();
        for _ in 0..MAX_CONNS {
            server.add_connection();
        }
        let elapsed = start_time.elapsed();
        println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);

        // Verify all connections are added
        assert_eq!(server.connections.len(), MAX_CONNS);
        assert!(elapsed.as_secs() < 5);
    }

    #[tokio::test]
    #[ignore]
    async fn test_parallel_performance_at_scale() {
        const MAX_CONNS: usize = 1_000_000;

        let server = std::sync::Arc::new(tokio::sync::Mutex::new(MasterServer::new()));

        // Performance test: adding connections in parallel
        let start_time = Instant::now();
        let mut handles = Vec::new();
        for _ in 0..MAX_CONNS {
            let server = std::sync::Arc::clone(&server);
            handles.push(
                tokio::spawn(async move {
                    let mut server = server.lock().await;
                    server.add_connection();
                })
            );
        }

        for handle in handles {
            let _ = handle.await.unwrap();
        }

        let elapsed = start_time.elapsed();
        println!("Time to add {MAX_CONNS} connections in parallel: {:?}", elapsed);

        // Verify all connections are added
        assert!(elapsed.as_secs() < 5);

        let server = server.lock().await;
        assert_eq!(server.connections.len(), MAX_CONNS);
    }

    #[tokio::test]
    #[ignore]
    async fn test_performance_get_connection_by_id() {
        const MAX_CONNS: usize = 1_000_000;

        let mut server = MasterServer::new();

        // Add multiple connections
        let start_time = Instant::now();
        for _ in 0..MAX_CONNS {
            server.add_connection();
        }
        let elapsed = start_time.elapsed();
        println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);

        assert!(elapsed.as_secs() < 5);

        // Performance test: retrieving connections by ID
        let start_time = Instant::now();
        for id in server.connections.keys().cloned() {
            let _ = server.get_connection_by_id(id);
        }
        let elapsed = start_time.elapsed();
        println!("Time to get {MAX_CONNS} connections by ID: {:?}", elapsed);

        // Assert the lookup performance
        assert!(elapsed.as_secs() < 5);
    }

    #[tokio::test]
    #[ignore]
    /// does add get remove
    async fn test_performance_full_connection_by_id_looped() {
        const MAX_CONNS: usize = 1_000_000;
        const LOOP_COUNT: usize = 5;

        let mut server = MasterServer::new();
        let mut server_no_free = MasterServerNoFree::new();

        let servers: Vec<&mut dyn Server> = vec![&mut server, &mut server_no_free];

        println!("Free list vs No Free List Performance Test\n");
        for server in servers {
            for i in 0..LOOP_COUNT {
                // Add multiple connections
                let start_time = Instant::now();
                for _ in 0..MAX_CONNS {
                    server.add_connection();
                }
                let elapsed = start_time.elapsed();
                println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);

                assert!(elapsed.as_secs() < 5);

                // Performance test: retrieving connections by ID
                let start_time = Instant::now();
                for id in server.get_connections().keys().cloned() {
                    let _ = server.get_connection_by_id(id);
                }
                let elapsed = start_time.elapsed();
                println!("Time to get {MAX_CONNS} connections by ID: {:?}", elapsed);

                // Assert the lookup performance
                assert!(elapsed.as_secs() < 5);

                // Remove all connections

                let ids: Vec<u64> = server.get_connections().keys().cloned().collect();
                for id in ids {
                    server.remove_connection(id);
                }

                println!(
                    "Completed iteration {} with capacity: {}",
                    i + 1,
                    server.get_connections().capacity()
                );
                println!("Final next_id: {}", server.next_id());
            }
            println!("\n----\n");
        }
    }

    #[tokio::test]
    #[ignore]
    async fn test_remove_all_connections() {
        const MAX_CONNS: usize = 1_000_000;

        let mut server = MasterServer::new();

        // Add multiple connections
        let start_time = Instant::now();
        for _ in 0..MAX_CONNS {
            server.add_connection();
        }
        let elapsed = start_time.elapsed();
        println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);

        // Verify all connections are added
        assert_eq!(server.connections.len(), MAX_CONNS);

        // Remove all connections
        let start_time = Instant::now();
        let ids: Vec<u64> = server.connections.keys().cloned().collect();
        for id in ids {
            server.remove_connection(id);
        }
        let elapsed = start_time.elapsed();
        println!("Time to remove {MAX_CONNS} connections: {:?}", elapsed);

        // Verify all connections are removed
        assert!(server.connections.is_empty());
        assert!(elapsed.as_secs() < 5);
    }

    #[tokio::test]
    #[ignore]
    async fn test_remove_all_connections_looped() {
        const MAX_CONNS: usize = 1_000_000;
        const LOOP_COUNT: usize = 5;

        let mut server = MasterServer::new();

        for i in 0..LOOP_COUNT {
            // Add multiple connections
            let start_time = Instant::now();
            for _ in 0..MAX_CONNS {
                server.add_connection();
            }
            let elapsed = start_time.elapsed();
            println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);
            assert!(elapsed.as_secs() < 5);

            // Verify all connections are added
            assert_eq!(server.connections.len(), MAX_CONNS);

            // Remove all connections
            let start_time = Instant::now();
            let ids: Vec<u64> = server.connections.keys().cloned().collect();
            for id in ids {
                server.remove_connection(id);
            }
            let elapsed = start_time.elapsed();
            println!("Time to remove {MAX_CONNS} connections: {:?}", elapsed);

            // Verify all connections are removed
            assert!(server.connections.is_empty());
            assert!(elapsed.as_secs() < 5);

            println!(
                "Completed iteration {} with capacity: {}",
                i + 1,
                server.connections.capacity()
            );
        }
    }

    async fn get_conn_mem_usage(server: &MasterServer) -> usize {
        std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>() +
            std::mem::size_of_val(&server.next_id)
    }

    #[tokio::test]
    #[ignore]
    async fn test_ram_usage() {
        const MAX_CONNS: usize = 1_000_000;

        let mut server = MasterServer::new();
        let memory_usage_before = std::mem::size_of_val(&server);
        println!("Estimated server memory usage before operations: {} bytes", memory_usage_before);

        // Add multiple connections
        let memory_connections_before = get_conn_mem_usage(&server).await;
        println!("Estimated memory usage before adding connections: {} bytes", memory_connections_before);
        let start_time = Instant::now();
        for _ in 0..MAX_CONNS {
            server.add_connection();
        }
        let elapsed = start_time.elapsed();
        let memory_connections_after = get_conn_mem_usage(&server).await;

        println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);
        println!("Estimated memory usage after adding connections: {} bytes", memory_connections_after);

        // Verify all connections are added
        assert_eq!(server.connections.len(), MAX_CONNS);

        // Remove all connections
        let memory_connections_before = get_conn_mem_usage(&server).await;
        println!("Estimated memory usage before adding connections: {} bytes", memory_connections_before);
        let start_time = Instant::now();
        for id in server.connections.keys().cloned().collect::<Vec<u64>>() {
            server.remove_connection(id);
        }
        let elapsed = start_time.elapsed();
        let memory_connections_after = get_conn_mem_usage(&server).await;

        println!("Time to remove {MAX_CONNS} connections: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);
        println!("Estimated memory usage after removing connections: {} bytes", memory_connections_after);

        // Verify all connections are removed
        assert!(server.connections.is_empty());

        let memory_connections_end = get_conn_mem_usage(&server).await;
        println!("Estimated memory usage after operations: {} bytes", memory_connections_end);
    }
}

/// The point of this module is to stress test the current implementation of the MasterServer.
mod integrated_tests {}
