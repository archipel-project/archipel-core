use crate::{Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Status, id = 1)]
pub struct QueryPongS2c {
    pub payload: u64,
}
