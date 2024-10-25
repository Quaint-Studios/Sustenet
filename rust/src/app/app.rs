use std::env;
use std::sync::{ Arc, Mutex };

use crate::clients::Client;
use crate::master::MasterServer;
use crate::options;
use crate::transport::ThreadManager;
use crate::utils::constants;

pub struct App {
    client_list: Arc<Mutex<Vec<Client>>>,
    // cluster_server: Option<ClusterServer,
    master_server: Option<MasterServer>,
}

impl App {
    pub fn init() -> Self {
        App {
            client_list: Arc::new(Mutex::new(Vec::new())),
            master_server: None,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        let args: Vec<String> = env::args().collect();

        if args.len() > 1 {
            match args[1].as_str() {
                "help" => {
                    options::show_help();
                    return Ok(());
                }
                "version" => {
                    println!("Sustenet v{}", constants::VERSION);
                    return Ok(());
                }
                "client" | "c" => {
                    self.start_client(&args[2..]);
                    return Ok(());
                }
                "cluster" | "cs" => {
                    return Ok(());
                }
                "master" | "ms" => {
                    // TODO Use config file
                    self.master_server = Some(MasterServer::new());
                    return Ok(());
                }
                _ => {
                    println!("Add 'help' to this command to get a list of options.");
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    /// Start the client mode.
    ///
    /// Only meant for debugging.
    fn start_client(&self, args: &[String]) {
        let mut client_list = self.client_list.lock().unwrap();
        let max_clients: u32 = if args.len() > 0 { args[0].parse().unwrap_or(1) } else { 1 };

        println!(
            "{}Starting client mode...{}",
            constants::TERMINAL_ORANGE,
            constants::TERMINAL_DEFAULT
        );

        if args.len() > 0 {
            println!(
                "{}Number of clients: {}{}",
                constants::TERMINAL_BLUE,
                &max_clients,
                constants::TERMINAL_DEFAULT
            );
        } else {
            println!(
                "{}No number of clients provided. Defaulting to 1.{}",
                constants::TERMINAL_BLUE,
                constants::TERMINAL_DEFAULT
            );
        }

        let client_list = Arc::clone(&self.client_list);
        tokio::spawn(async move {
            for _ in 0..max_clients {
                let client_list2 = Arc::clone(&client_list);
                let thread_manager = ThreadManager::get_instance();
                thread_manager.execute_on_side_thread(
                    Box::new(move || {
                        let client = Client::new(None, None);
                        // println!(
                        //     "Connecting client to IP {}:{}",
                        //     client.master_connection.ip, client.master_connection.port
                        // );

                        // client.connect(Client::ConnectionType.MasterServer); // TODO

                        let mut list = client_list2.lock().unwrap();
                        list.push(client);
                    })
                );
            }
        });

        println!(
            "{}Finished connecting {} clients to the server.{}",
            constants::TERMINAL_GREEN,
            max_clients,
            constants::TERMINAL_DEFAULT
        )
    }
}
