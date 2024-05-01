use std::borrow::Cow;

use crate::{Decode, Encode, Packet, PacketState};
use valence_text::Text;

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Login, id = 0)]
pub struct LoginDisconnectS2c<'a> {
    pub reason: Cow<'a, Text>,
}
