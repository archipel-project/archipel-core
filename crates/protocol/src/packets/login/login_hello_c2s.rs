use uuid::Uuid;

use crate::{types::str::Bounded, Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 0)]
pub struct LoginHelloC2s<'a> {
    pub username: Bounded<&'a str, 16>,
    pub profile_id: Uuid,
}
