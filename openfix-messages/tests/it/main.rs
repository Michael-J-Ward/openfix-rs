
//! Integration tests
//! 
//! Placed here so that they are only compiled once

use std::io::{Write, self};

use openfix_messages::{FixParseError, AsFixMessageField, FromFixMessageField};

use openfix_messages::dec_helpers::split_message_items;


#[cfg(test)]
mod deserialize;

#[cfg(test)]
mod serialize_empty_trailers;

#[cfg(test)]
mod serialize_with_trailers;

#[cfg(test)]
mod helpers;


#[derive(Debug, PartialEq)]
struct TestStruct {
    value: String,
}

impl AsFixMessageField for TestStruct {
    const FIX_KEY: u32 = 42;

    fn encode_fix_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        write!(writer, "{}", self.value)
    }
}

impl FromFixMessageField for TestStruct {
    fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError> {
        Ok(Self {
            value: std::str::from_utf8(value)?.to_string(),
        })
    }
}

macro_rules! encode_field {
    ($field:expr) => {{
        let mut payload = vec![];
        $field.encode_message(&mut payload).unwrap();
        payload
    }};
}

#[test]
fn test_struct_encode() {
    let field = TestStruct {
        value: "foobar".into(),
    };
    assert_eq!(encode_field!(field), b"42=foobar\x01");
}

#[test]
fn test_struct_decode() {
    assert_eq!(
        TestStruct::decode_message(&split_message_items(b"foo")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestStruct::decode_message(&split_message_items(b"foo=bar")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestStruct::decode_message(&split_message_items(b"12=bar")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestStruct::decode_message(&split_message_items(b"42=foobar")),
        Ok(TestStruct {
            value: "foobar".into(),
        })
    );
}

#[derive(Debug, PartialEq)]
enum TestEnum {
    Opt1,
    Opt2,
}

impl AsFixMessageField for TestEnum {
    const FIX_KEY: u32 = 29;

    fn encode_fix_value<W>(&self, writer: &mut W) -> io::Result<()>
    where
        W: Write,
    {
        write!(
            writer,
            "{}",
            match *self {
                Self::Opt1 => "opt1",
                Self::Opt2 => "opt2",
            }
        )
    }
}

impl FromFixMessageField for TestEnum {
    fn from_fix_value(value: &[u8]) -> Result<Self, FixParseError> {
        match value {
            b"opt1" => Ok(Self::Opt1),
            b"opt2" => Ok(Self::Opt2),
            _ => Err(FixParseError::InvalidData),
        }
    }
}

#[test]
fn test_enum_encode() {
    let field = TestEnum::Opt1;
    assert_eq!(encode_field!(field), b"29=opt1\x01");
    let field = TestEnum::Opt2;
    assert_eq!(encode_field!(field), b"29=opt2\x01");
}

#[test]
fn test_enum_decode() {
    assert_eq!(
        TestEnum::decode_message(&split_message_items(b"foo")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestEnum::decode_message(&split_message_items(b"foo=bar")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestEnum::decode_message(&split_message_items(b"12=bar")),
        Err(FixParseError::InvalidData)
    );
    assert_eq!(
        TestEnum::decode_message(&split_message_items(b"29=opt1")),
        Ok(TestEnum::Opt1)
    );
    assert_eq!(
        TestEnum::decode_message(&split_message_items(b"29=opt2")),
        Ok(TestEnum::Opt2)
    );
}

fn main() {
    println!("integration-tests");
}