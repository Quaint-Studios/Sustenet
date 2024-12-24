use tokio::select;

use shared::config::master::{ read, Settings };
use shared::log_message;
use shared::utils::{ self, constants };

#[tokio::main]
async fn main() {
    let mut shutdown_rx = utils::shutdown_channel().expect("Error creating shutdown channel.");

    select! {
        _ = shutdown_rx.recv() => {
            warning("Shutting down...");
        }
        _ = start() => {}
    }

    success("The Master Server has been shut down.");
}

async fn start() {
    let is_running = true;

    let Settings { server_name, max_connections, port } = read();
    println!("Server name: {}", server_name);

    {
        let max_connections_str = match max_connections {
            0 => "unlimited max connections".to_string(),
            1 => "1 max connection".to_string(),
            _ => format!("{} max connections", max_connections),
        };

        debug(
            format!("Starting the Master Server on port {} with {max_connections_str}...", port).as_str()
        );
    }

    while is_running {
        println!("Master is running...");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!("Master is shutting down...");
}

// region: Logging
fn debug(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Debug, LogType::Master, "{}", message);
}

// fn info(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Info, LogType::Master, "{}", message);
//}

fn warning(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Warning, LogType::Master, "{}", message);
}

// fn error(message: &str) {
//     if !constants::DEBUGGING {
//         return;
//     }
//     log_message!(LogLevel::Error, LogType::Master, "{}", message);
// }

fn success(message: &str) {
    if !constants::DEBUGGING {
        return;
    }
    log_message!(LogLevel::Success, LogType::Master, "{}", message);
}
// endregion
