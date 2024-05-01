use std::error::Error;

use derive_more::Display;

#[derive(Display, Debug, Copy, Clone)]
pub struct EncodeError;

impl Error for EncodeError {}

#[derive(Display, Debug, Copy, Clone)]
pub enum DecodeError {
    #[display(fmt = "malformed packet")]
    Incomplete,
    #[display(fmt = "item too long")]
    TooLong,
    #[display(fmt = "invalid value")]
    InvalidValue,
}

impl Error for DecodeError {}
