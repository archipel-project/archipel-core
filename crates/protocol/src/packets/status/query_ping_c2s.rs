use crate::{Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Status, id = 1)]
pub struct QueryPingC2s {
    pub payload: u64,
}
