pub mod dec_helpers;
pub mod enc_helpers;

mod fix_versions;
pub use fix_versions::*;

pub mod prelude {
    pub use super::{
        AsFixMessage, AsFixMessageField, FixFieldItems, FixParseError, FromFixMessage,
        FromFixMessageField, MessageDest,
    };
}

use std::io::{self, Write};
use std::str::Utf8Error;
use thiserror::Error;

pub use crate::dec_helpers::FixFieldItems;

pub trait AsFixMessageField {
    /// Fix key representation
    const FIX_KEY: u32;

    /// FIX value representation
    fn encode_fix_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;

    /// Encode field as "Key=Value"
    fn encode_message<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        write!(writer, "{}=", Self::FIX_KEY)?;
        self.encode_fix_value(writer)?;
        write!(writer, "\x01")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum FixParseError {
    #[error("invalid data")]
    InvalidData,

    #[error("invalid string")]
    InvalidString(#[from] Utf8Error),

    #[error("no data for this field ID")]
    NoData,
}

pub trait FromFixMessageField: AsFixMessageField {
    /// FIX value representation
    fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError>
    where
        Self: Sized;

    /// Decode field from map of (key ID => value) data
    fn decode_message(items: &FixFieldItems) -> Result<Self, FixParseError>
    where
        Self: Sized,
    {
        let key_id = Self::FIX_KEY;
        let data = items
            .get(&key_id)
            .ok_or_else(|| FixParseError::InvalidData)?;

        Self::from_fix_value(&data)
    }
}

pub trait AsFixMessage {
    fn encode_message<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write;
}

pub trait FromFixMessage {
    fn decode_message(items: &FixFieldItems) -> Result<Self, FixParseError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq)]
pub enum MessageDest {
    Admin,
    App,
}
