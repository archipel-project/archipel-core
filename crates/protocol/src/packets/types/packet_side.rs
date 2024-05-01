/// The side a packet is intended for.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum PacketSide {
    /// Server -> Client
    Clientbound,
    /// Client -> Server
    Serverbound,
}
