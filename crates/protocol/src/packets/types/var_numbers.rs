use thiserror::Error;

pub const SEGSZ: usize = 7;

pub const CONTINUE_BIT: u8 = 1 << SEGSZ;
pub const SEGMENT_MASK: u8 = CONTINUE_BIT - 1;

#[derive(Debug, Error)]
pub enum VarDecodeError {
    #[error("incomplete Var decode")]
    Incomplete,
    #[error("Var is too large")]
    TooLarge,
}
