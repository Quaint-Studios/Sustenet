use std::io::{self, Read};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::app::App;
use crate::transport::thread_manager::*;
use crate::utils::constants;

mod app;
mod clients;
mod master;
mod transport;
mod utils;

lazy_static::lazy_static! {
    static ref IS_RUNNING: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

fn main() {
    let thread_manager = ThreadManager::get_instance();

    {
        let mut is_running = IS_RUNNING.lock().unwrap();
        *is_running = true;
    }
    thread_manager.execute_on_main_thread(Box::new(|| {
        println!("Hello, world!");
    }));

    let is_running_clone = Arc::clone(&IS_RUNNING);
    let thread_manager_clone = Arc::clone(&thread_manager);

    thread_manager_clone.execute_on_main_thread(Box::new(|| {
        println!("Hello, world!");
    }));

    let logic_thread = thread::Builder::new()
        .name("Logic".to_string())
        .spawn(move || update_main(is_running_clone, thread_manager_clone));

    // TODO

    let app = App::init();
    let _ = app.start();

    println!("Press Enter to close Sustenet...");
    let mut buffer = [0; 1];
    let _ = io::stdin().read_exact(&mut buffer);

    println!("Closing Sustenet...");

    //#region Cleanup
    {
        let mut is_running = IS_RUNNING.lock().unwrap();
        *is_running = false;
    }
    thread_manager.deinit();
    //#endregion
}

fn update_main(is_running: Arc<Mutex<bool>>, thread_manager: Arc<ThreadManager>) {
    let mut next = Instant::now();
    while *is_running.lock().unwrap() {
        let now = Instant::now();
        while next < now {
            thread_manager.update_main();
            next += Duration::from_millis(constants::MS_PER_TICK);
            if next > now {
                thread::sleep(next - now);
            }
        }
    }
}
