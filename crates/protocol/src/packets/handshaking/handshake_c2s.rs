use protocol_derive::{Decode, Encode, Packet};

use crate::{
    packets::types::{PacketState, VarInt},
    types::str::Bounded,
};

#[derive(Debug, Encode, Decode)]
pub enum HandshakeNextState {
    #[packet(value = 1)]
    Status,
    #[packet(value = 2)]
    Login,
}

#[derive(Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Handshaking, id = 0)]
pub struct HandshakeC2S<'a> {
    pub protocol_version: VarInt,
    pub server_address: Bounded<&'a str, 255>,
    pub server_port: u16,
    pub next_state: HandshakeNextState,
}
