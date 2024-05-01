use anyhow::ensure;
use bytes::BytesMut;

use crate::__private::{Decode, Packet};

#[derive(Clone, Debug)]
pub struct PacketFrame {
    /// The ID of the decoded packet.
    pub id: i32,
    /// The contents of the packet after the leading VarInt ID.
    pub body: BytesMut,
}

impl PacketFrame {
    /// Attempts to decode this packet as type `P`. An error is returned if the
    /// packet ID does not match, the body of the packet failed to decode, or
    /// some input was missed.
    pub fn decode<'a, P>(&'a self) -> anyhow::Result<P>
    where
        P: Packet + Decode<'a>,
    {
        ensure!(
            P::ID == self.id,
            "packet ID mismatch while decoding '{}': expected {}, got {}",
            P::NAME,
            P::ID,
            self.id
        );

        let mut r = &self.body[..];

        let pkt = P::decode(&mut r)?;

        ensure!(
            r.is_empty(),
            "missed {} bytes while decoding '{}'",
            r.len(),
            P::NAME
        );

        Ok(pkt)
    }
}
