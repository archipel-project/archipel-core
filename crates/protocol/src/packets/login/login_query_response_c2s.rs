use crate::{
    packets::types::VarInt,
    types::{impls::raw::RawBytes, str::Bounded},
    Decode, Encode, Packet, PacketState,
};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 4)]
pub struct LoginQueryResponseC2s<'a> {
    pub message_id: VarInt,
    pub data: Option<Bounded<RawBytes<'a>, 1048576>>,
}
