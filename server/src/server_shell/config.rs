use std::net::SocketAddr;
use std::num::NonZeroU32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub runtime_config: RuntimeConfig,
    pub network_config: NetworkConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// The number of threads to use for the IO pool.
    pub tokio_threads: Option<NonZeroU32>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            tokio_threads: Some(NonZeroU32::new(4).unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The address to bind the server to.
    pub bind: SocketAddr,
    /// Message Of The Day, shown to the client when they ping the server.
    pub motd: String,
    /// The maximum number of players that can be connected to the server.
    pub max_players: NonZeroU32,
    /// If true, the server will allow only online-mode clients to connect.
    pub online_mode: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            bind: "127.0.0.1:25565".parse().unwrap(),
            motd: "Hello, world!".to_string(),
            max_players: NonZeroU32::new(4).unwrap(),
            online_mode: false,
        }
    }
}

