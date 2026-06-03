use serde::ser::Serializer;
use std::fmt::Write as _;

/// Fixed-length ASCII/CP-1252 string read from simulator shared memory.
///
/// Stores bytes inline — no heap allocation. Implements `Display`, `Debug`,
/// `Serialize` (as a plain JSON string), and `PartialEq<&str>`.
///
/// Used for byte-string fields in LMU and iRacing shared-memory layouts.
#[derive(Clone, Copy)]
pub struct SimString<const N: usize>(pub(crate) [u8; N]);

impl<const N: usize> SimString<N> {
    /// Wrap a fixed-size byte array from shared memory (by reference).
    pub fn from_bytes(src: &[u8; N]) -> Self {
        Self(*src)
    }

    /// Wrap a fixed-size byte array passed by value.
    ///
    /// Used when copying fields out of `#[repr(C, packed)]` structs to avoid
    /// taking an unaligned reference.
    pub fn from_u8_array(src: [u8; N]) -> Self {
        Self(src)
    }

    /// Decode to an owned `String` (CP-1252, truncated at the first null byte).
    ///
    /// Allocates — prefer `Display` / `Serialize` when you don't need ownership.
    pub fn to_string_lossy(&self) -> String {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        crate::decode_cp1252(&self.0[..len])
    }

    fn write_decoded(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        let bytes = &self.0[..len];
        if bytes.iter().all(|&b| b < 0x80) {
            // SAFETY: all bytes are valid ASCII, a subset of UTF-8
            return f.write_str(unsafe { std::str::from_utf8_unchecked(bytes) });
        }
        f.write_str(&crate::decode_cp1252(bytes))
    }
}

impl<const N: usize> std::fmt::Display for SimString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.write_decoded(f)
    }
}

impl<const N: usize> std::fmt::Debug for SimString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        self.write_decoded(f)?;
        f.write_char('"')
    }
}

impl<const N: usize> PartialEq<&str> for SimString<N> {
    fn eq(&self, other: &&str) -> bool {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        let bytes = &self.0[..len];
        if bytes.iter().all(|&b| b < 0x80) {
            return bytes == other.as_bytes();
        }
        crate::decode_cp1252(bytes) == *other
    }
}

impl<const N: usize> serde::Serialize for SimString<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let len = self.0.iter().position(|&b| b == 0).unwrap_or(N);
        let bytes = &self.0[..len];
        if bytes.iter().all(|&b| b < 0x80) {
            // SAFETY: all bytes are valid ASCII, a subset of UTF-8
            serializer.serialize_str(unsafe { std::str::from_utf8_unchecked(bytes) })
        } else {
            serializer.serialize_str(&crate::decode_cp1252(bytes))
        }
    }
}

impl<const N: usize> Default for SimString<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

// ──────────────────────────────────────────────────────────────────────────────

/// Fixed-length UTF-16 LE string read from simulator shared memory.
///
/// Stores `u16` code units inline — no heap allocation. Implements `Display`,
/// `Debug`, `Serialize` (as a plain JSON string), and `PartialEq<&str>`.
///
/// Used for string fields in Assetto Corsa and AC Evo shared-memory pages.
#[derive(Clone, Copy)]
pub struct SimStringU16<const N: usize>(pub(crate) [u16; N]);

impl<const N: usize> SimStringU16<N> {
    /// Wrap a fixed-size UTF-16 array from shared memory (by reference).
    pub fn from_u16(src: &[u16; N]) -> Self {
        Self(*src)
    }

    /// Wrap a fixed-size UTF-16 array passed by value.
    ///
    /// Used when copying fields out of `#[repr(C, packed)]` structs to avoid
    /// taking an unaligned reference.
    pub fn from_u16_array(src: [u16; N]) -> Self {
        Self(src)
    }

    /// Decode to an owned `String` (UTF-16 LE, truncated at the first null code unit).
    ///
    /// Allocates — prefer `Display` / `Serialize` when you don't need ownership.
    pub fn to_string_lossy(&self) -> String {
        let len = self.0.iter().position(|&c| c == 0).unwrap_or(N);
        String::from_utf16_lossy(&self.0[..len])
    }
}

impl<const N: usize> std::fmt::Display for SimStringU16<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.0.iter().position(|&c| c == 0).unwrap_or(N);
        let units = &self.0[..len];
        if units.iter().all(|&c| c < 0x80) {
            // All code units are ASCII — write directly without allocating
            for &c in units {
                f.write_char(c as u8 as char)?;
            }
            return Ok(());
        }
        f.write_str(&String::from_utf16_lossy(units))
    }
}

impl<const N: usize> std::fmt::Debug for SimStringU16<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('"')?;
        std::fmt::Display::fmt(self, f)?;
        f.write_char('"')
    }
}

impl<const N: usize> PartialEq<&str> for SimStringU16<N> {
    fn eq(&self, other: &&str) -> bool {
        let len = self.0.iter().position(|&c| c == 0).unwrap_or(N);
        let units = &self.0[..len];
        if units.iter().all(|&c| c < 0x80) && other.is_ascii() {
            return units.len() == other.len()
                && units.iter().zip(other.bytes()).all(|(&u, b)| u == b as u16);
        }
        String::from_utf16_lossy(units) == *other
    }
}

impl<const N: usize> Default for SimStringU16<N> {
    fn default() -> Self {
        Self([0u16; N])
    }
}

impl<const N: usize> serde::Serialize for SimStringU16<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string_lossy())
    }
}
