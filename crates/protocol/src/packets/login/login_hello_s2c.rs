use crate::{types::str::Bounded, Decode, Encode, Packet, PacketState};

#[derive(Copy, Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 1)]
pub struct LoginHelloS2c<'a> {
    pub server_id: Bounded<&'a str, 20>,
    pub public_key: &'a [u8],
    pub verify_token: &'a [u8],
}
