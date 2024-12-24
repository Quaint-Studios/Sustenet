pub mod master {
    use config::{ Config, File, FileFormat::Toml };

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
            port: settings.get::<u16>("all.port").unwrap_or(0),
        }
    }
}

pub mod cluster {
    use config::{ Config, File, FileFormat::Toml };

    pub struct Settings {
        pub server_name: String,

        pub max_connections: u32,
        pub port: u16,

        pub key_name: String,
        pub master_ip: String,
        pub master_port: u16,
    }

    pub fn read() -> Settings {
        let settings = Config::builder()
            .add_source(File::new("/Config.toml", Toml))
            .build()
            .expect("Failed to read the configuration file.");

        Settings {
            server_name: settings
                .get::<String>("all.server_name")
                .unwrap_or("Cluster Server".to_string()),

            max_connections: settings.get::<u32>("max_connections").unwrap_or(0),
            port: settings.get::<u16>("port").unwrap_or(0),

            key_name: settings.get::<String>("key_name").unwrap_or("cluster_key".to_string()),
            master_ip: settings.get::<String>("master_ip").unwrap_or("127.0.0.1".to_string()),
            master_port: settings.get::<u16>("master_port").unwrap_or(0),
        }
    }
}
