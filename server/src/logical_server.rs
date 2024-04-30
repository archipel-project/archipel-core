use flume::Receiver;
use tracing::info;

use networking::client::PrimitiveClientComponents;

/**
 * This module contains the logical server implementation.
 * The logical server is responsible for managing the game state and
 * processing the game logic.
 */


pub(crate) struct GameServer {
    new_connections: Receiver<PrimitiveClientComponents>,

}

impl GameServer {
    pub fn new(new_connections: Receiver<PrimitiveClientComponents>) -> Self {
        Self {
            new_connections
        }
    }

    pub fn tick(&mut self, _tick_date: u64) -> anyhow::Result<()> {
        for client in self.new_connections.try_iter() {


            info!("New connection: {:?}, {:?}, {:?}", client.ip, client.username, client.uuid);
        }
        // todo: implement game logic
        Ok(())
    }
}