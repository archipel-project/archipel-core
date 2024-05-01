use super::pa::VarInt;
use bytes::BytesMut;

pub struct PacketContainer {
    length: VarInt,
    id: VarInt,
    data: BytesMut,
}
