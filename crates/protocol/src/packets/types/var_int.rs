use byteorder::ReadBytesExt;
use derive_more::{Deref, DerefMut, From, Into};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    usize,
};

use crate::__private::{Decode, Encode};

use super::var_numbers::{VarDecodeError, CONTINUE_BIT, SEGMENT_MASK};

/// An `i32` encoded with variable length.
#[derive(
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Debug,
    Deref,
    DerefMut,
    From,
    Into,
    Serialize,
    Deserialize,
)]
#[serde(transparent)]
#[repr(transparent)]
pub struct VarInt(pub i32);

impl VarInt {
    /// The maximum number of bytes a VarInt could occupy when read from and
    /// written to the Minecraft protocol.
    pub const MAX_SIZE: usize = 5;

    /// Returns the exact number of bytes this varint will write when
    /// [`Encode::encode`] is called, assuming no error occurs.
    pub const fn written_size(self) -> usize {
        match self.0 {
            0 => 1,
            n => (31 - n.leading_zeros() as usize) / 7 + 1,
        }
    }

    pub fn decode_partial(mut r: impl Read) -> Result<i32, VarDecodeError> {
        let mut val = 0;

        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8().map_err(|_| VarDecodeError::Incomplete)?;
            val |= ((byte & SEGMENT_MASK) as i32) << (i * 7);
            if byte & CONTINUE_BIT == 0 {
                return Ok(val);
            }
        }

        Err(VarDecodeError::TooLarge)
    }
}

impl Encode for VarInt {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        let x = self.0 as u64;
        let stage1 = (x & 0x000000000000007f)
            | ((x & 0x0000000000003f80) << 1)
            | ((x & 0x00000000001fc000) << 2)
            | ((x & 0x000000000fe00000) << 3)
            | ((x & 0x00000000f0000000) << 4);

        let leading = stage1.leading_zeros();

        let unused_bytes = (leading - 1) >> 3;
        let bytes_needed = 8 - unused_bytes;

        // set all but the last MSBs
        let msbs = 0x8080808080808080;
        let msbmask = 0xffffffffffffffff >> (((8 - bytes_needed + 1) << 3) - 1);

        let merged = stage1 | (msbs & msbmask);
        let bytes = merged.to_le_bytes();

        w.write_all(unsafe { bytes.get_unchecked(..bytes_needed as usize) })?;

        Ok(())
    }
}

impl Decode<'_> for VarInt {
    fn decode(r: &mut &'_ [u8]) -> anyhow::Result<Self> {
        let mut val = 0;
        for i in 0..Self::MAX_SIZE {
            let byte = r.read_u8()?;
            val |= ((byte & SEGMENT_MASK) as i32) << (i * 7);
            if byte & CONTINUE_BIT == 0 {
                return Ok(Self(val));
            }
        }

        Err(VarDecodeError::TooLarge.into())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn varint_to_i32() {
        let v = VarInt::decode(&mut [0xff, 0xff, 0xff, 0xff, 0x0f].as_slice()).unwrap();
        assert_eq!(v.0, -1);
    }

    #[test]
    #[should_panic(expected = "Var is too large")]
    fn invalid_varint() {
        // 6 bytes
        VarInt::decode(&mut [0xff, 0xff, 0xff, 0xff, 0xff, 0xff].as_slice()).unwrap();
    }

    use rand::{thread_rng, Rng};

    #[test]
    fn varint_written_size() {
        let mut rng = thread_rng();
        let mut buf = vec![];

        for n in (0..100_000)
            .map(|_| rng.gen())
            .chain([0, i32::MIN, i32::MAX])
            .map(VarInt)
        {
            buf.clear();
            n.encode(&mut buf).unwrap();
            assert_eq!(buf.len(), n.written_size());
        }
    }

    #[test]
    fn varint_round_trip() {
        let mut rng = thread_rng();
        let mut buf = vec![];

        for n in (0..1_000_000)
            .map(|_| rng.gen())
            .chain([0, i32::MIN, i32::MAX])
        {
            VarInt(n).encode(&mut buf).unwrap();

            let mut slice = buf.as_slice();
            assert!(slice.len() <= VarInt::MAX_SIZE);

            assert_eq!(n, VarInt::decode(&mut slice).unwrap().0);

            assert!(slice.is_empty());
            buf.clear();
        }
    }
}
