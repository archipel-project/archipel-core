use crate::{Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Status, id = 0)]
pub struct QueryRequestC2s;
