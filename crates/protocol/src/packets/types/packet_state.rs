/// The statein  which a packet is used.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PacketState {
    Handshaking,
    Status,
    Login,
    Play,
}
