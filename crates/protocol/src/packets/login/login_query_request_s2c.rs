use std::borrow::Cow;

use ident::Ident;

use crate::{
    packets::types::VarInt,
    types::{impls::raw::RawBytes, str::Bounded},
    Decode, Encode, Packet, PacketState,
};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 4)]
pub struct LoginQueryRequestS2c<'a> {
    pub message_id: VarInt,
    pub channel: Ident<Cow<'a, str>>,
    pub data: Bounded<RawBytes<'a>, 1048576>,
}
