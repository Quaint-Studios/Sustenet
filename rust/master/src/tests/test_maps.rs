#[cfg(test)]
mod tests {
    const MAX_ITERS: usize = 10_000_000;
    const MAX_THREADS: usize = 8;

    /// Tests the speed of adding, getting, updating, and removing items from a HashMap.
    /// Time taken to add 10000000 items: 7.9314173s
    /// Time taken to get 10000000 items: 4.9250668s
    /// Time taken to update 10000000 items: 5.0271649s
    /// Time taken to remove 10000000 items: 7.4092275s
    #[test]
    fn test_hashmaps() {
        use std::collections::HashMap;
        use std::time::Instant;

        let mut map: HashMap<usize, usize> = HashMap::new();

        // Test adding items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.insert(i, i);
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items: {:?}", MAX_ITERS, duration_add);

        // Test getting items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            let _ = map.get(&i);
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items: {:?}", MAX_ITERS, duration_get);

        // Test updating items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.insert(i, i + 1);
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items: {:?}", MAX_ITERS, duration_update);

        // Test removing items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.remove(&i);
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items: {:?}", MAX_ITERS, duration_remove);
    }

    /// Tests the speed of adding, getting, updating, and removing items from a DashMap.
    /// Time taken to add 10000000 items: 12.5746573s
    /// Time taken to get 10000000 items: 6.2193888s
    /// Time taken to update 10000000 items: 6.5075287s
    /// Time taken to remove 10000000 items: 8.9363106s
    #[test]
    fn test_dashmaps() {
        use dashmap::DashMap;
        use std::time::Instant;

        let map: DashMap<usize, usize> = DashMap::new();

        // Test adding items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.insert(i, i);
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items: {:?}", MAX_ITERS, duration_add);

        // Test getting items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            let _ = map.get(&i);
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items: {:?}", MAX_ITERS, duration_get);

        // Test updating items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.insert(i, i + 1);
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items: {:?}", MAX_ITERS, duration_update);

        // Test removing items
        let start = Instant::now();
        for i in 0..MAX_ITERS {
            map.remove(&i);
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items: {:?}", MAX_ITERS, duration_remove);
    }

    /// Tests the speed of adding, getting, updating, and removing items from a HashMap with threads.
    /// Time taken to add 10000000 items with threads: 12.3948596s
    /// Time taken to get 10000000 items with threads: 9.8344117s
    /// Time taken to update 10000000 items with threads: 9.98254s
    /// Time taken to remove 10000000 items with threads: 13.0771387s
    #[test]
    fn test_hashmaps_with_threads() {
        use std::collections::HashMap;
        use std::sync::{ Arc, Mutex };
        use std::thread;
        use std::time::Instant;

        let map: Arc<Mutex<HashMap<usize, usize>>> = Arc::new(Mutex::new(HashMap::new()));

        // Test adding items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.lock().unwrap().insert(j, j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items with threads: {:?}", MAX_ITERS, duration_add);

        // Test getting items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    let _ = map_clone.lock().unwrap().get(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items with threads: {:?}", MAX_ITERS, duration_get);

        // Test updating items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone
                        .lock()
                        .unwrap()
                        .insert(j, j + 1);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items with threads: {:?}", MAX_ITERS, duration_update);

        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.lock().unwrap().remove(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items with threads: {:?}", MAX_ITERS, duration_remove);
    }

    /// Tests the speed of adding, getting, updating, and removing items from a DashMap with threads.
    /// Time taken to add 10000000 items with threads: 6.2227182s
    /// Time taken to get 10000000 items with threads: 1.3018232s
    /// Time taken to update 10000000 items with threads: 1.4454566s
    /// Time taken to remove 10000000 items with threads: 2.0433425s
    #[test]
    fn test_dashmaps_with_threads() {
        use dashmap::DashMap;
        use std::sync::Arc;
        use std::thread;
        use std::time::Instant;

        let map: Arc<DashMap<usize, usize>> = Arc::new(DashMap::new());

        // Test adding items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.insert(j, j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items with threads: {:?}", MAX_ITERS, duration_add);

        // Test getting items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    let _ = map_clone.get(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items with threads: {:?}", MAX_ITERS, duration_get);

        // Test updating items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.insert(j, j + 1);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items with threads: {:?}", MAX_ITERS, duration_update);

        // Test removing items with threads
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = thread::spawn(move || {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.remove(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items with threads: {:?}", MAX_ITERS, duration_remove);
    }

    /// Tests the speed of adding, getting, updating, and removing items from a HashMap on tokio.
    /// Time taken to add 10000000 items with tokio: 8.4185739s
    /// Time taken to get 10000000 items with tokio: 5.542175s
    /// Time taken to update 10000000 items with tokio: 5.5900584s
    /// Time taken to remove 10000000 items with tokio: 7.4953945s
    #[tokio::test]
    async fn test_hashmaps_tokio() {
        use std::collections::HashMap;
        use std::sync::{ Arc, Mutex };
        use tokio::time::Instant;

        let map: Arc<Mutex<HashMap<usize, usize>>> = Arc::new(Mutex::new(HashMap::new()));

        // Test adding items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.lock().unwrap().insert(j, j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items with tokio: {:?}", MAX_ITERS, duration_add);

        // Test getting items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    let _ = map_clone.lock().unwrap().get(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items with tokio: {:?}", MAX_ITERS, duration_get);

        // Test updating items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone
                        .lock()
                        .unwrap()
                        .insert(j, j + 1);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items with tokio: {:?}", MAX_ITERS, duration_update);

        // Test removing items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.lock().unwrap().remove(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items with tokio: {:?}", MAX_ITERS, duration_remove);
    }

    /// Tests the speed of adding, getting, updating, and removing items from a DashMap with tokio.
    /// Time taken to add 10000000 items with tokio: 12.3175472s
    /// Time taken to get 10000000 items with tokio: 6.2607827s
    /// Time taken to update 10000000 items with tokio: 7.4993663s
    /// Time taken to remove 10000000 items with tokio: 9.1845476s
    #[tokio::test]
    async fn test_dashmaps_tokio() {
        use dashmap::DashMap;
        use std::sync::Arc;
        use tokio::time::Instant;

        let map: Arc<DashMap<usize, usize>> = Arc::new(DashMap::new());

        // Test adding items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.insert(j, j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_add = start.elapsed();
        println!("Time taken to add {} items with tokio: {:?}", MAX_ITERS, duration_add);

        // Test getting items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    let _ = map_clone.get(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_get = start.elapsed();
        println!("Time taken to get {} items with tokio: {:?}", MAX_ITERS, duration_get);

        // Test updating items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.insert(j, j + 1);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_update = start.elapsed();
        println!("Time taken to update {} items with tokio: {:?}", MAX_ITERS, duration_update);

        // Test removing items
        let start = Instant::now();
        let mut handles = vec![];
        for i in 0..MAX_THREADS {
            let map_clone = Arc::clone(&map);
            let handle = tokio::spawn(async move {
                for j in (i * MAX_ITERS) / MAX_THREADS..((i + 1) * MAX_ITERS) / MAX_THREADS {
                    map_clone.remove(&j);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.await.unwrap();
        }
        let duration_remove = start.elapsed();
        println!("Time taken to remove {} items with tokio: {:?}", MAX_ITERS, duration_remove);
    }
}
