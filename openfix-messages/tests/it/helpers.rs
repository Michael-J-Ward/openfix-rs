

use std::collections::HashMap;
use std::iter::FromIterator;

use openfix_messages::dec_helpers::split_message_items;
use openfix_messages::enc_helpers::FixEnvelopeBuilder;

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

macro_rules! build_message {
    ($builder:expr, $content:expr) => {{
        let mut data = Vec::new();
        $builder.build_message(&mut data, $content).unwrap();
        data
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

#[test]
fn test_add_envelope_defaults() {
    let builder = FixEnvelopeBuilder::new();

    assert_eq!(
        build_message!(builder, b""),
        b"8=FIX.4.4\x019=0\x0110=200\x01".to_vec()
    );
    assert_eq!(
        build_message!(builder, b"5=foo\x019=bar\x01"),
        b"8=FIX.4.4\x019=12\x015=foo\x019=bar\x0110=094\x01".to_vec()
    );
    assert_eq!(
        build_message!(builder,
            b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
        ),
        b"8=FIX.4.4\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=064\x01".to_vec()
    );
}

#[test]
fn test_add_envelope_with_params() {
    let builder = FixEnvelopeBuilder::new().begin_string("FIX.4.2");

    assert_eq!(
        build_message!(builder, b""),
        b"8=FIX.4.2\x019=0\x0110=198\x01".to_vec()
    );
    assert_eq!(
        build_message!(builder, b"5=foo\x019=bar\x01"),
        b"8=FIX.4.2\x019=12\x015=foo\x019=bar\x0110=092\x01".to_vec()
    );
    assert_eq!(
        build_message!(builder,
            b"35=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x01"
        ),
        b"8=FIX.4.2\x019=65\x0135=A\x0149=SERVER\x0156=CLIENT\x0134=177\x0152=20090107-18:15:16\x0198=0\x01108=30\x0110=062\x01".to_vec()
    );
}