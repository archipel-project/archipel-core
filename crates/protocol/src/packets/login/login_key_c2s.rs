use crate::{Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 1)]
pub struct LoginKeyC2s<'a> {
    pub shared_secret: &'a [u8],
    pub verify_token: &'a [u8],
}
