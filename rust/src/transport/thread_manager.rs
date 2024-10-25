use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::sync::{ Arc, Mutex };

pub struct ThreadManager {
    main_pool: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
    side_pool: Arc<Mutex<VecDeque<Box<dyn FnOnce() + Send>>>>,
    execute_main_event: Arc<Mutex<bool>>,
    execute_side_event: Arc<Mutex<bool>>,
}

lazy_static! {
    static ref INSTANCE: Arc<ThreadManager> = Arc::new(ThreadManager::new());
}

impl ThreadManager {
    fn new() -> Self {
        ThreadManager {
            main_pool: Arc::new(Mutex::new(VecDeque::new())),
            side_pool: Arc::new(Mutex::new(VecDeque::new())),
            execute_main_event: Arc::new(Mutex::new(false)),
            execute_side_event: Arc::new(Mutex::new(false)),
        }
    }

    /// Get the singleton instance.
    pub fn get_instance() -> Arc<ThreadManager> {
        INSTANCE.clone()
    }
    //#region Execution Functions
    /// Sets an event to be executed on the main thread.
    /// * `callable` - The event to be executed on the main thread.
    pub fn execute_on_main_thread(&self, action: Box<dyn FnOnce() + Send>) {
        let mut execute_main_event = self.execute_main_event.lock().unwrap();
        *execute_main_event = true;

        let mut main_pool = self.main_pool.lock().unwrap();
        main_pool.push_back(action);
    }

    /// Execute all code meant to run on the main thread. Should only be called from the main thread.
    pub fn update_main(&self) {
        let mut execute_main_event = self.execute_main_event.lock().unwrap();
        if *execute_main_event {
            let mut main_pool = self.main_pool.lock().unwrap();

            *execute_main_event = false;
            drop(execute_main_event);

            let mut main_pool_copied = VecDeque::new();
            std::mem::swap(&mut *main_pool, &mut main_pool_copied);

            drop(main_pool);

            while let Some(action) = main_pool_copied.pop_front() {
                action();
            }
        }
    }

    pub fn execute_on_side_thread(&self, action: Box<dyn FnOnce() + Send>) {
        let mut execute_side_event = self.execute_side_event.lock().unwrap();
        *execute_side_event = true;

        let mut side_pool = self.side_pool.lock().unwrap();
        side_pool.push_back(action);
    }

    pub fn update_side(&self) {
        let mut execute_side_event = self.execute_side_event.lock().unwrap();
        if *execute_side_event {
            let mut side_pool = self.side_pool.lock().unwrap();

            *execute_side_event = false;
            drop(execute_side_event);

            let mut side_pool_copied = VecDeque::new();
            std::mem::swap(&mut *side_pool, &mut side_pool_copied);

            drop(side_pool);

            while let Some(action) = side_pool_copied.pop_front() {
                action();
            }
        }
    }

    pub fn deinit(&self) {
        let mut main_pool = self.main_pool.lock().unwrap();
        main_pool.clear();

        let mut side_pool = self.side_pool.lock().unwrap();
        side_pool.clear();
    }
}
