pub mod enc_helpers;

mod fix_versions;
pub use fix_versions::*;

pub mod prelude {
    pub use super::{
        AsFixMessage, AsFixMessageField, FixFieldItems, FixParseError, FromFixMessage,
        FromFixMessageField, MessageDest,
    };
}

use std::collections::HashMap;
use std::io::{self, Write};
use std::str::Utf8Error;
use thiserror::Error;

pub type FixFieldItems<'a> = HashMap<u32, &'a [u8]>;

const SEP_CHAR: u8 = 0x01;

pub fn split_message_iter<'a>(data: &'a [u8]) -> impl Iterator<Item=(u32, &'a [u8])> {
    data.split(|x| *x == SEP_CHAR)
        .filter_map(|field| {
            let mut fields = field.splitn(2, |x| *x == '=' as u8);

            let field_id = fields.next()?;
            let field_id = std::str::from_utf8(field_id).ok()?;
            let field_id = field_id.parse::<u32>().ok()?;

            let field_data = fields.next()?;

            Some((field_id, field_data))
        })
}

/// Split message items
pub fn split_message_items<'a>(data: &'a [u8]) -> FixFieldItems<'a> {
    split_message_iter(data).collect()
}

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
    #[error("missing tag: {0}")]
    MissingTag(u32),

    #[error("invalid field")]
    InvalidField(&'static str),

    #[error("invalid group")]
    InvalidGroup(&'static str),

    #[error("invalid data: {0:?}")]
    InvalidData(bstr::BString),

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
            .ok_or_else(|| FixParseError::MissingTag(key_id))?;

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
