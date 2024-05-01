pub mod other;
pub mod pointer;
pub mod primitives;
pub mod raw;
pub mod sequence;
pub mod strings;

use std::mem;

/// Prevents preallocating too much memory in case we get a malicious or invalid
/// sequence length.
fn cautious_capacity<Element>(size_hint: usize) -> usize {
    const MAX_PREALLOC_BYTES: usize = 1024 * 1024;

    if mem::size_of::<Element>() == 0 {
        0
    } else {
        size_hint.min(MAX_PREALLOC_BYTES / mem::size_of::<Element>())
    }
}
