use tracing::info;
use validator::Validate;

use crate::server::config;

pub struct CoreServer {}

impl CoreServer {
    pub fn new() -> Self {
        CoreServer {}
    }

    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Booting up the server...");

        // load the configuration
        // TODO: load the configuration from environment variables
        let config = config::Config::default();

        config.validate()?;

        Ok(())
    }
}
