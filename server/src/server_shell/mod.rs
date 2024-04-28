mod config;

use std::net::SocketAddr;
use tokio::signal;
use networking::ConnectionMode;
use crate::logical_server::GameServer;

/**
    * ServerShell is the main struct for the server.
    * It is responsible for starting the server and running the server.
    * This struct handle global services like tokio runtime, shutdown signal, HostFxr localisation, etc.
 */
pub struct ServerShell {
    config: config::Config,
    tokio: tokio::runtime::Runtime,
}

impl ServerShell {
    pub fn init() -> anyhow::Result<Self> {

        // load config in first, it could be used by other services...
        let config = Self::load_config()?;

        // tokio runtime
        let mut tokio_builder = tokio::runtime::Builder::new_multi_thread();
        tokio_builder .enable_all();

        if let Some(threads) = config.runtime_config.tokio_threads {
            tokio_builder.worker_threads(threads.get());
        }

        let tokio = tokio_builder.build()?;

        // tracing
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::TRACE)
            .finish();

        tracing::subscriber::set_global_default(subscriber)?;

        // todo: HostFxr localisation

        Ok(Self {
            config,
            tokio,
        })
    }

    fn load_config() -> anyhow::Result<config::Config> {
        let config_file = std::fs::read_to_string("config.toml");
        return match config_file {
            Ok(file) => {
                let config: config::Config = toml::from_str(&file)?;
                Ok(config)
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    tracing::warn!("Config file not found, writing default config");
                    let config = config::Config::default();
                    let toml = toml::to_string(&config)?;
                    std::fs::write("config.toml",String::from( "# go see: [insert doc link]\n\n") + &toml)?;
                    return Ok(config);
                }
                Err(anyhow::anyhow!("Error reading config file: {:?}", e))
            }
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {

        self.starting()?;

        let new_connections = networking::build_plugin(self.config.network_config.clone(), &self.tokio)?;
        let logical_server = GameServer::new(new_connections);

        let task = async {

            tokio::select! {
                result = Self::game_loop(logical_server) => result,
                result = signal::ctrl_c() => {
                    tracing::info!("shutting down server");
                    result.map_err(|e| anyhow::anyhow!("error: {:?}", e))
                },
            }
        };

        self.tokio.block_on(task)?;
        Ok(())
    }

    fn starting(&mut self) -> anyhow::Result<()> {
        tracing::info!("Starting {} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        Ok(())
    }

    async fn game_loop(mut game_server: GameServer) -> anyhow::Result<()> {
        let duration = std::time::Duration::from_millis(50); // 20 Hz
        let mut interval = tokio::time::interval(duration);

        let mut tick_date = 0u64;
        loop {
            interval.tick().await;
            game_server.tick(tick_date)?;
            tick_date += 1;
        }
    }
}
