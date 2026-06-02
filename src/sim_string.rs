use serde::ser::Serializer;

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
}

impl<const N: usize> std::fmt::Display for SimString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string_lossy())
    }
}

impl<const N: usize> std::fmt::Debug for SimString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string_lossy())
    }
}

impl<const N: usize> PartialEq<&str> for SimString<N> {
    fn eq(&self, other: &&str) -> bool {
        self.to_string_lossy() == *other
    }
}

impl<const N: usize> serde::Serialize for SimString<N> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string_lossy())
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
        f.write_str(&self.to_string_lossy())
    }
}

impl<const N: usize> std::fmt::Debug for SimStringU16<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.to_string_lossy())
    }
}

impl<const N: usize> PartialEq<&str> for SimStringU16<N> {
    fn eq(&self, other: &&str) -> bool {
        self.to_string_lossy() == *other
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
