use std::borrow::Cow;

use uuid::Uuid;

use crate::{
    packets::types::profile::Property, types::str::Bounded, Decode, Encode, Packet, PacketState,
};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 2)]
pub struct LoginSuccessS2c<'a> {
    pub uuid: Uuid,
    pub username: Bounded<&'a str, 16>,
    pub properties: Cow<'a, [Property<&'a str>]>,
}
