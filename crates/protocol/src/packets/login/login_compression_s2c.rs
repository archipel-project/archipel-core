use crate::{packets::types::VarInt, Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 3)]
pub struct LoginCompressionS2c {
    pub threshold: VarInt,
}
