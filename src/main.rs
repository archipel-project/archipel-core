use std::process;

use tracing::error;

mod bootstrap;
mod server;

fn main() {
    bootstrap::init().expect("Failed to initialize the logger");

    let mut server = server::CoreServer::new();

    match server.start() {
        Ok(_) => {}
        Err(e) => {
            error!("An error occured: {:?}", e);
            process::exit(1)
        }
    }
}
