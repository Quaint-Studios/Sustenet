pub mod master {
    use config::{ Config, File, FileFormat::Toml };

    use crate::utils::constants::MASTER_PORT;

    pub struct Settings {
        pub server_name: String,

        pub max_connections: u32,
        pub port: u16,
    }

    pub fn read() -> Settings {
        let settings = Config::builder()
            .add_source(File::new("Config.toml", Toml))
            .build()
            .expect("Failed to read the configuration file.");

        Settings {
            server_name: settings
                .get::<String>("all.server_name")
                .unwrap_or("Master Server".to_string()),

            max_connections: settings.get::<u32>("all.max_connections").unwrap_or(0),
            port: match settings.get::<u16>("all.port") {
                Ok(port) =>
                    match port {
                        0 => MASTER_PORT,
                        _ => port,
                    }
                Err(_) => MASTER_PORT,
            },
        }
    }
}

pub mod cluster {
    use config::{ Config, File, FileFormat::Toml };

    use crate::utils::constants::{ CLUSTER_PORT, DEFAULT_IP, MASTER_PORT };

    pub struct Settings {
        pub server_name: String,

        pub max_connections: u32,
        pub port: u16,

        pub key_name: String,
        pub master_ip: String,
        pub master_port: u16,

        pub domain_pub_key: Option<String>,
    }

    pub fn read() -> Settings {
        let settings = Config::builder()
            .add_source(File::new("Config.toml", Toml))
            .build()
            .expect("Failed to read the configuration file.");

        Settings {
            server_name: settings
                .get::<String>("all.server_name")
                .unwrap_or("Cluster Server".to_string()),

            max_connections: settings.get::<u32>("max_connections").unwrap_or(0),
            port: match settings.get::<u16>("all.port") {
                Ok(port) =>
                    match port {
                        0 => CLUSTER_PORT,
                        _ => port,
                    }
                Err(_) => CLUSTER_PORT,
            },

            key_name: settings
                .get::<String>("cluster.key_name")
                .unwrap_or("cluster_key".to_string()),
            master_ip: settings
                .get::<String>("cluster.master_ip")
                .unwrap_or(DEFAULT_IP.to_string()),
            master_port: match settings.get::<u16>("cluster.master_port") {
                Ok(port) =>
                    match port {
                        0 => MASTER_PORT,
                        _ => port,
                    }
                Err(_) => MASTER_PORT,
            },

            domain_pub_key: match settings.get::<String>("cluster.domain_pub_key") {
                Ok(domain) => Some(domain),
                Err(_) => None,
            },
        }
    }
}
