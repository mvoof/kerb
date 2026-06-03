//! iRacing SDK low-level C-compatible types.
//! These structs are `#[repr(C)]` so they can be reinterpreted directly from
//! the shared-memory region written by the iRacing sim.

pub const IRSDK_MAX_BUFS: usize = 4;
pub const IRSDK_MAX_STRING: usize = 32;
pub const IRSDK_MAX_DESC: usize = 64;

/// One of up to `IRSDK_MAX_BUFS` double-buffered telemetry data slots.
/// The sim writes to alternating buffers so readers can always access a consistent frame.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct irsdk_varBuf {
    pub tick_count: i32,
    pub buf_offset: i32,
    pub pad: [i32; 2],
}

/// Top-level header at offset 0 of the iRacing shared-memory region.
/// Contains version info, session-info location, variable table offset, and the double-buffer array.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct irsdk_header {
    pub ver: i32,
    pub status: i32,
    pub tick_rate: i32,
    pub session_info_update: i32,
    pub session_info_len: i32,
    pub session_info_offset: i32,
    pub num_vars: i32,
    pub var_header_offset: i32,
    pub num_buf: i32,
    pub buf_len: i32,
    pub pad: [i32; 2],
    pub var_buf: [irsdk_varBuf; IRSDK_MAX_BUFS],
}

/// Describes a single telemetry variable: its type, byte offset within a data buffer,
/// element count (>1 for arrays), and human-readable name/description/unit strings.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct irsdk_varHeader {
    pub type_: i32,
    pub offset: i32,
    pub count: i32,
    pub count_as_char: u8,
    pub pad: [u8; 3],
    pub name: [u8; IRSDK_MAX_STRING],
    pub desc: [u8; IRSDK_MAX_DESC],
    pub unit: [u8; IRSDK_MAX_STRING],
}

/// Variable data types matching the iRacing C SDK `irsdk_VarType` enum.
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarType {
    Char = 0,
    Bool = 1,
    Int = 2,
    BitField = 3,
    Float = 4,
    Double = 5,
}

impl VarType {
    /// Convert a raw `i32` discriminant from shared memory into a `VarType`.
    pub fn from_i32(val: i32) -> Option<Self> {
        match val {
            0 => Some(VarType::Char),
            1 => Some(VarType::Bool),
            2 => Some(VarType::Int),
            3 => Some(VarType::BitField),
            4 => Some(VarType::Float),
            5 => Some(VarType::Double),
            _ => None,
        }
    }
}
