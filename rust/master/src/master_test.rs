#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bytes::Bytes;
    use tokio::time::Instant;

    struct Connection {
		sender: tokio::sync::mpsc::Sender<Bytes>,
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
			let (sender, _receiver) = tokio::sync::mpsc::channel(100); // Example channel with a buffer size of 100

            // Add the connection
            self.connections.insert(id, Connection {
				sender,
			});
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

        // Verify the removed ID is added to the free list
        assert!(server.free_list.contains(&removed_id));
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

	#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
	#[ignore]
	/// NOTE: This is bad.
	async fn test_parallel_performance_at_scale() {
		const MAX_CONNS: usize = 1_000_000;

		let mut server = std::sync::Arc::new(tokio::sync::Mutex::new(MasterServer::new()));

		// Performance test: adding connections in parallel
		let start_time = Instant::now();
		let mut handles = Vec::new();
		for _ in 0..MAX_CONNS {
			let server = std::sync::Arc::clone(&server);
			handles.push(tokio::spawn(async move {
				let mut server = server.lock().await;
				server.add_connection();
			}));
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
        assert_eq!(server.free_list.len(), MAX_CONNS);
        assert!(elapsed.as_secs() < 5);
    }

    #[tokio::test]
	#[ignore]
    async fn test_ram_usage() {
        const MAX_CONNS: usize = 1_000_000;

        let mut server = MasterServer::new();
        let memory_usage_before = std::mem::size_of_val(&server);
        println!("Estimated memory usage before operations: {} bytes", memory_usage_before);

        // Add multiple connections
        let memory_connections_before =
            std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>();
        let memory_free_list_before =
            std::mem::size_of_val(&server.free_list) +
            server.free_list.capacity() * std::mem::size_of::<u64>();
        println!(
            "Estimated memory usage before adding connections: ({} conn bytes) ({} free_list bytes) {} total bytes",
            memory_connections_before,
            memory_free_list_before,
            memory_connections_before + memory_free_list_before
        );
        let start_time = Instant::now();
        for _ in 0..MAX_CONNS {
            server.add_connection();
        }
        let elapsed = start_time.elapsed();
        let memory_connections_after =
            std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>();
        let memory_free_list_after =
            std::mem::size_of_val(&server.free_list) +
            server.free_list.capacity() * std::mem::size_of::<u64>();

        println!("Time to add {MAX_CONNS} connections: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);
        println!(
            "Estimated memory usage after adding connections: ({} conn bytes) ({} free_list bytes) {} total bytes",
            memory_connections_after,
            memory_free_list_after,
            memory_connections_after + memory_free_list_after
        );

        // Verify all connections are added
        assert_eq!(server.connections.len(), MAX_CONNS);

        // Remove all connections
        let memory_connections_before =
            std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>();
        let memory_free_list_before =
            std::mem::size_of_val(&server.free_list) +
            server.free_list.capacity() * std::mem::size_of::<u64>();
        println!(
            "Estimated memory usage before adding connections: ({} conn bytes) ({} free_list bytes) {} total bytes",
            memory_connections_before,
            memory_free_list_before,
            memory_connections_before + memory_free_list_before
        );
        let start_time = Instant::now();
        let ids: Vec<u64> = server.connections.keys().cloned().collect();
        for id in ids {
            server.remove_connection(id);
        }
        let elapsed = start_time.elapsed();
        let memory_connections_after =
            std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>();
        let memory_free_list_after =
            std::mem::size_of_val(&server.free_list) +
            server.free_list.capacity() * std::mem::size_of::<u64>();

        println!("Time to remove {MAX_CONNS} connections: {:?}", elapsed);
        assert!(elapsed.as_secs() < 5);
        println!(
            "Estimated memory usage after removing connections: ({} conn bytes) ({} free_list bytes) {} total bytes",
            memory_connections_after,
            memory_free_list_after,
            memory_connections_after + memory_free_list_after
        );

        // Verify all connections are removed
        assert!(server.connections.is_empty());
        assert_eq!(server.free_list.len(), MAX_CONNS);

        // Check memory usage (this is a rough estimate)
        let memory_connections_end =
            std::mem::size_of_val(&server.connections) +
            server.connections.capacity() * std::mem::size_of::<(u64, Connection)>();
        let memory_free_list_end =
            std::mem::size_of_val(&server.free_list) +
            server.free_list.capacity() * std::mem::size_of::<u64>();
        let memory_usage = memory_connections_end + memory_free_list_end;
        println!("Estimated memory usage after operations: {} bytes", memory_usage);
    }
}

/// The point of this module is to stress test the current implementation of the MasterServer.
mod integrated_tests {
	
}