use kerb::decode_cp1252;

#[test]
fn decode_cp1252_ascii() {
    assert_eq!(decode_cp1252(b"hello"), "hello");
}

#[test]
fn decode_cp1252_accented() {
    // é in cp1252 is byte 0xE9
    let bytes = &[0x41u8, 0x6C, 0x65, 0x78, 0x20, 0xE9]; // "Alex é"
    let result = decode_cp1252(bytes);
    assert!(result.contains('é'));
}
