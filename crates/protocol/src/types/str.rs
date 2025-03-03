// The original file is owned by the valence project, which can be found at:
// https://valence.rs/rustdoc/src/valence_protocol/bounded.rs.html#20
// The original file is licensed under the MIT License, which can be found at:
// https://github.com/valence-rs/valence/blob/main/LICENSE.txt

use std::borrow::Borrow;
use std::fmt::Display;

use derive_more::{AsRef, Deref, DerefMut, From};

/// A newtype wrapper for `T` which modifies the [`Encode`](crate::Encode) and
/// [`Decode`](crate::Decode) impls to be bounded by some upper limit `MAX`.
/// Implementations are expected to error eagerly if the limit is exceeded.
///
/// What exactly `MAX` represents depends on the type `T`. Here are some
/// instances:
/// - **arrays/slices**: The maximum number of elements.
/// - **strings**: The maximum number of utf16 code units.
/// - **[`RawBytes`]**: The maximum number of bytes.
///
/// [`RawBytes`]: crate::RawBytes
#[derive(
    Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deref, DerefMut, AsRef, From,
)]
pub struct Bounded<T, const MAX: usize>(pub T);

impl<T, const MAX: usize> Bounded<T, MAX> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Bounded<U, MAX> {
        Bounded(f(self.0))
    }

    pub fn map_into<U: From<T>>(self) -> Bounded<U, MAX> {
        Bounded(self.0.into())
    }
}

impl<T, const MAX: usize> Borrow<T> for Bounded<T, MAX> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T: Display, const MAX: usize> Display for Bounded<T, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
