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

// `decode_cp1252` decodes using the *system's* single-byte ANSI code page
// (`GetACP()`), not literal cp1252. On a Russian Windows install (ACP 1251)
// the byte 0xE9 is Cyrillic 'щ', not 'é' — so the same input decodes
// differently depending on which machine runs the test. This test verifies
// decoding is internally consistent with whatever the system ACP actually is,
// instead of assuming Western European Windows.
#[cfg(all(windows, any(feature = "iracing", feature = "ac-evo", feature = "lmu")))]
#[test]
fn decode_cp1252_matches_system_acp() {
    let acp = unsafe { windows_sys::Win32::Globalization::GetACP() };
    let expected_encoding = match acp {
        1251 => encoding_rs::WINDOWS_1251,
        1252 => encoding_rs::WINDOWS_1252,
        1250 => encoding_rs::WINDOWS_1250,
        1253 => encoding_rs::WINDOWS_1253,
        1254 => encoding_rs::WINDOWS_1254,
        1255 => encoding_rs::WINDOWS_1255,
        1256 => encoding_rs::WINDOWS_1256,
        1257 => encoding_rs::WINDOWS_1257,
        1258 => encoding_rs::WINDOWS_1258,
        874 => encoding_rs::WINDOWS_874,
        932 => encoding_rs::SHIFT_JIS,
        936 => encoding_rs::GBK,
        949 => encoding_rs::EUC_KR,
        950 => encoding_rs::BIG5,
        _ => encoding_rs::WINDOWS_1252,
    };

    // A high byte (>= 0x80) whose meaning depends entirely on the active code
    // page: 'é' in cp1252, 'щ' in cp1251, etc.
    let bytes = &[0xE9u8];
    let (expected, _, _) = expected_encoding.decode(bytes);
    assert_eq!(decode_cp1252(bytes), expected.into_owned());
}

// A single-byte ANSI code page (cp1251, cp1252, ...) can only represent one
// script at a time. Characters outside that script (e.g. CJK on a Cyrillic
// or Western system) have no valid byte encoding in it — iRacing itself
// substitutes '?' for them server-side before the bytes ever reach shared
// memory, so no client-side decoding fix can recover them. This test
// documents that '?' bytes pass through unchanged rather than being
// misinterpreted.
#[test]
fn decode_cp1252_unrepresentable_chars_stay_as_question_marks() {
    let bytes = b"?? ?";
    assert_eq!(decode_cp1252(bytes), "?? ?");
}

// Real-world driver names, each encoded in the ANSI code page their own
// Windows locale would use. `decode_cp1252` reads the *local* system's ACP
// (see `decode_cp1252_matches_system_acp`), so it can only recover a name
// correctly when the running machine's locale happens to match the encoding
// the name was originally written in. Everyone else sees mojibake or '?' —
// this is not something a single-byte-codepage API can fix client-side.
#[cfg(all(windows, any(feature = "iracing", feature = "ac-evo", feature = "lmu")))]
#[test]
fn decode_cp1252_names_only_roundtrip_on_matching_locale() {
    // Mirrors decode_cp1252's internal system_acp_encoding() mapping,
    // including its "unrecognized ACP falls back to cp1252" default (this
    // matters on machines with ACP 65001 / "Beta: UTF-8 system locale").
    let acp = unsafe { windows_sys::Win32::Globalization::GetACP() };
    let active_encoding = match acp {
        1251 => encoding_rs::WINDOWS_1251,
        1252 => encoding_rs::WINDOWS_1252,
        1250 => encoding_rs::WINDOWS_1250,
        1253 => encoding_rs::WINDOWS_1253,
        1254 => encoding_rs::WINDOWS_1254,
        1255 => encoding_rs::WINDOWS_1255,
        1256 => encoding_rs::WINDOWS_1256,
        1257 => encoding_rs::WINDOWS_1257,
        1258 => encoding_rs::WINDOWS_1258,
        874 => encoding_rs::WINDOWS_874,
        932 => encoding_rs::SHIFT_JIS,
        936 => encoding_rs::GBK,
        949 => encoding_rs::EUC_KR,
        950 => encoding_rs::BIG5,
        _ => encoding_rs::WINDOWS_1252,
    };

    let cases: &[(&str, &encoding_rs::Encoding)] = &[
        ("José Müller", encoding_rs::WINDOWS_1252),
        ("Алексей Мальков", encoding_rs::WINDOWS_1251),
        ("梦涛 鄂", encoding_rs::GBK),
        ("유준 김", encoding_rs::EUC_KR),
    ];

    for (name, encoding) in cases {
        let (bytes, _, had_unmappable) = encoding.encode(name);
        assert!(
            !had_unmappable,
            "{name} should be fully representable in its own code page"
        );

        let decoded = decode_cp1252(&bytes);

        if std::ptr::eq(*encoding, active_encoding) {
            assert_eq!(
                decoded, *name,
                "should roundtrip when system ACP matches the source code page"
            );
        } else {
            assert_ne!(
                decoded, *name,
                "decoding {name}'s bytes under the active system code page ({acp}) \
                 should not accidentally produce the original text"
            );
        }
    }
}
