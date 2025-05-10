use config::{ Config, File, FileFormat::Toml };

use crate::utils::constants::{ DEFAULT_IP, DOMAIN_PUB_KEY };

/// Creates a new Config.toml file with default values if it doesn't exist.
pub fn init() -> std::io::Result<()> {
    create_data_dir()?;
    let config_path = "data/Config.toml";
    if !std::path::Path::new(config_path).exists() {
        let default_config = format!(
            r#"[master]
max_connections = 0
port = 0
bind = "{DEFAULT_IP}" # This is the IP address to bind to.

[cluster]
name = "Cluster Server"

max_connections = 0
port = 0
bind = "{DEFAULT_IP}" # This is the IP address to bind to.

key_name = "cluster_key" # This is the name of the key that must exist BOTH on the cluster and the master.
master_ip = "127.0.0.1" # This is the IP address of the master server.
master_port = 0 # This is the port of the master server.

domain_pub_key = "{DOMAIN_PUB_KEY}" # Remove this if you want to use the server's bandwidth to send a key to a user directly.
"#
        );
        std::fs::write(config_path, default_config)?;
    }

    Ok(())
}

pub fn create_data_dir() -> std::io::Result<()> {
    if std::fs::DirBuilder::new().recursive(true).create("data").is_err() {
        return Err(
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to create the 'data' directory.")
        );
    }

    Ok(())
}

pub mod master {
    use config::{ Config, File, FileFormat::Toml };

    use crate::utils::constants::{ DEFAULT_IP, MASTER_PORT };

    pub struct Settings {
        pub max_connections: u32,
        pub port: u16,
        pub bind: String,
    }

    pub fn read() -> Settings {
        super::init().expect("Failed to initialize the configuration file.");
        let settings = Config::builder()
            .add_source(File::new("data/Config.toml", Toml))
            .build()
            .expect("Failed to read the configuration file.");

        Settings {
            max_connections: settings.get::<u32>("all.max_connections").unwrap_or(0),
            port: match settings.get::<u16>("all.port") {
                Ok(port) =>
                    match port {
                        0 => MASTER_PORT,
                        _ => port,
                    }
                Err(_) => MASTER_PORT,
            },
            bind: settings.get::<String>("all.bind").unwrap_or(DEFAULT_IP.to_string()),
        }
    }
}

pub mod cluster {
    use config::{ Config, File, FileFormat::Toml };

    use crate::utils::constants::{ CLUSTER_PORT, DEFAULT_IP, MASTER_PORT };

    pub struct Settings {
        pub name: String,

        pub max_connections: u32,
        pub port: u16,
        pub bind: String,

        pub key_name: String,
        pub master_ip: String,
        pub master_port: u16,

        pub domain_pub_key: Option<String>,
    }

    pub fn read() -> Settings {
        super::init().expect("Failed to initialize the configuration file.");
        let settings = Config::builder()
            .add_source(File::new("data/Config.toml", Toml))
            .build()
            .expect("Failed to read the configuration file.");

        Settings {
            name: settings.get::<String>("cluster.name").unwrap_or("Cluster Server".to_string()),

            max_connections: settings.get::<u32>("cluster.max_connections").unwrap_or(0),
            port: match settings.get::<u16>("cluster.port") {
                Ok(port) =>
                    match port {
                        0 => CLUSTER_PORT,
                        _ => port,
                    }
                Err(_) => CLUSTER_PORT,
            },
            bind: settings.get::<String>("cluster.bind").unwrap_or(DEFAULT_IP.to_string()),

            key_name: settings
                .get::<String>("cluster.key_name")
                .unwrap_or("cluster_key".to_string()),
            master_ip: settings
                .get::<String>("cluster.master_ip")
                .unwrap_or(DEFAULT_IP.to_string()),
            master_port: match settings.get::<u16>("master.port") {
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
