use std::io::Write;

use crate::{Decode, Encode};
use ident::{Ident, IdentError};
use uuid::Uuid;

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, mut w: impl Write) -> anyhow::Result<()> {
        match self {
            Some(t) => {
                true.encode(&mut w)?;
                t.encode(w)
            }
            None => false.encode(w),
        }
    }
}

impl<'a, T: Decode<'a>> Decode<'a> for Option<T> {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(match bool::decode(r)? {
            true => Some(T::decode(r)?),
            false => None,
        })
    }
}

impl Encode for Uuid {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        self.as_u128().encode(w)
    }
}

impl<'a> Decode<'a> for Uuid {
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        u128::decode(r).map(Uuid::from_u128)
    }
}

impl<S: Encode> Encode for Ident<S> {
    fn encode(&self, w: impl Write) -> anyhow::Result<()> {
        self.as_ref().encode(w)
    }
}

impl<'a, S> Decode<'a> for Ident<S>
where
    S: Decode<'a>,
    Ident<S>: TryFrom<S, Error = IdentError>,
{
    fn decode(r: &mut &'a [u8]) -> anyhow::Result<Self> {
        Ok(Ident::try_from(S::decode(r)?)?)
    }
}
