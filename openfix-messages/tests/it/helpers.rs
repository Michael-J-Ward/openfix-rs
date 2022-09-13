

use std::collections::HashMap;
use std::iter::FromIterator;

use openfix_messages::dec_helpers::split_message_items;

macro_rules! split_non_static {
    ($x:expr) => {{
        let result: HashMap<u32, Vec<u8>> = HashMap::from_iter(
            split_message_items($x)
                .iter()
                .map(|(k, v)| (*k, v.to_vec())),
        );

        result
    }};
}

#[test]
fn test_split_message_items_std_payload() {
    assert_eq!(
        split_non_static!(b""),
        HashMap::from_iter([])
    );
    assert_eq!(
        split_non_static!(b"\x01"),
        HashMap::from_iter([])
    );
    assert_eq!(
        split_non_static!(b"\x01\x01\x01\x01"),
        HashMap::from_iter([])
    );
    assert_eq!(
        split_non_static!(b"5=foo"),
        HashMap::from_iter([(5, b"foo".to_vec())])
    );
    assert_eq!(
        split_non_static!(b"5=foo\x012631=bar"),
        HashMap::from_iter([
            (5, b"foo".to_vec()),
            (2631, b"bar".to_vec())
        ])
    );
    assert_eq!(
        split_non_static!(b"\x01\x01\x015=foo\x012631=bar\x01\x01\x01"),
        HashMap::from_iter([
            (5, b"foo".to_vec()),
            (2631, b"bar".to_vec())
        ])
    );
}

#[test]
fn test_split_message_items_weird_payload() {
    assert_eq!(
        split_non_static!(b"5="),
        HashMap::from_iter([(5, b"".to_vec())])
    );
    assert_eq!(
        split_non_static!(b"foo=bar"),
        HashMap::from_iter([])
    );
    assert_eq!(
        split_non_static!(b"foobar"),
        HashMap::from_iter([])
    );
}