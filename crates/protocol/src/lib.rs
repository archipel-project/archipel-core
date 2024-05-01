use derive_more::{From, Into};

pub mod decode;
pub mod encode;
mod packet_frame;
pub mod packets;
pub mod types;

pub use packet_frame::PacketFrame;

#[doc(hidden)]
pub mod __private {
    pub use crate::packets::types::VarInt;
    pub use crate::types::{
        codec::{Decode, Encode},
        packet::Packet,
    };
    pub use anyhow::{anyhow, bail, ensure, Context, Result};
}

// This allow us to use our own proc macros internally.
extern crate self as protocol_lib;

/// The maximum number of bytes in a single Minecraft packet.
pub const MAX_PACKET_SIZE: i32 = 2097152;

/// The Minecraft protocol version this library currently targets.
pub const PROTOCOL_VERSION: i32 = 765;

/// The stringified name of the Minecraft version this library currently
/// targets.
pub const MINECRAFT_VERSION: &str = "1.20.4";

pub use decode::PacketDecoder;
pub use encode::{PacketEncoder, WritePacket};
pub use packets::types::{PacketSide, PacketState};
pub use protocol_derive::{Decode, Encode, Packet};
use serde::{Deserialize, Serialize};
pub use types::{
    codec::{Decode, Encode},
    packet::Packet,
};

/// How large a packet should be before it is compressed by the packet encoder.
///
/// If the inner value is >= 0, then packets with encoded lengths >= to this
/// value will be compressed. If the value is negative, then compression is
/// disabled and no packets are compressed.
#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, From, Into, Serialize, Deserialize,
)]
pub struct CompressionThreshold(pub i32);

impl CompressionThreshold {
    /// No compression.
    pub const DEFAULT: Self = Self(-1);
}

/// No compression.
impl Default for CompressionThreshold {
    fn default() -> Self {
        Self::DEFAULT
    }
}
