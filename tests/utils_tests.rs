use kerb::decode_cp1252;

#[test]
fn decode_cp1252_ascii() {
    assert_eq!(decode_cp1252(b"hello"), "hello");
}

// A slice that mixes ASCII with one high byte must bypass decode_cp1252's
// all-ASCII fast path and decode as a whole.
#[test]
fn decode_cp1252_mixed_ascii_and_high_byte() {
    // é in cp1252 is byte 0xE9
    let bytes = &[0x41u8, 0x6C, 0x65, 0x78, 0x20, 0xE9]; // "Alex é"

    assert_eq!(decode_cp1252(bytes), "Alex é");
}

// The sim writes a fixed single-byte encoding, so decoding must not depend on
// the reader's Windows locale. This is the whole point of using cp1252 rather
// than GetACP(): the same bytes have to produce the same text on a Russian,
// Japanese or Western install, otherwise two machines watching one session
// disagree about a driver's name.
#[test]
fn decode_cp1252_is_locale_independent() {
    // Bytes that every single-byte ANSI code page maps differently: 0xE9 is
    // 'é' in cp1252 but 'щ' in cp1251, 0xFC is 'ü' but 'ю'.
    let bytes = &[
        0x4Au8, 0x6F, 0x73, 0xE9, 0x20, 0x4D, 0xFC, 0x6C, 0x6C, 0x65, 0x72,
    ];

    assert_eq!(decode_cp1252(bytes), "José Müller");
}

// cp1252 and iso-8859-1 differ only in 0x80-0x9F, where iso-8859-1 has C1
// control codes and cp1252 has printable punctuation. The sim writes the
// printable ones, so cp1252 is the correct reading of that range.
#[test]
fn decode_cp1252_reads_the_c1_range_as_printable_punctuation() {
    assert_eq!(decode_cp1252(&[0x93, 0x94]), "\u{201C}\u{201D}"); // curly double quotes
    assert_eq!(decode_cp1252(&[0x92]), "\u{2019}"); // curly apostrophe
    assert_eq!(decode_cp1252(&[0x96]), "\u{2013}"); // en dash
    assert_eq!(decode_cp1252(&[0x85]), "\u{2026}"); // ellipsis
}

// A single-byte code page can only represent one script, so iRacing substitutes
// characters outside it *before* writing to shared memory — the bytes that
// reach us are already plain ASCII '?'. No decoding can recover the original,
// which is why non-Latin names need irsdkUTF8SessionStr=1 (a separate,
// UTF-8 code path). This test documents that the substitutes pass through
// unchanged rather than being mangled further.
#[test]
fn decode_cp1252_unrepresentable_chars_stay_as_question_marks() {
    let bytes = b"?? ?";

    assert_eq!(decode_cp1252(bytes), "?? ?");
}

// Undefined cp1252 positions must not panic or truncate the string; encoding_rs
// maps them to the replacement char.
#[test]
fn decode_cp1252_undefined_positions_become_replacement_chars() {
    let decoded = decode_cp1252(&[0x41, 0x81, 0x42]);

    assert_eq!(decoded.chars().count(), 3);
    assert!(decoded.starts_with('A') && decoded.ends_with('B'));
}
