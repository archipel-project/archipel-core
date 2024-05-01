use std::num::NonZeroUsize;
use serde::{Deserialize, Serialize};
use networking::NetworkConfig;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub runtime_config: RuntimeConfig,
    pub network_config: NetworkConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// The number of threads to use for the IO pool.
    pub tokio_threads: Option<NonZeroUsize>,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            tokio_threads: None,
        }
    }
}

