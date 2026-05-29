/// Primitive type tag for a telemetry variable.
///
/// Discriminant values match the iRacing SDK constants so we can transmute
/// directly from the header without a lookup table.
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
    /// Convert a raw `i32` discriminant back into a [`VarType`], returning `None` for unknown values.
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

/// A typed telemetry reading — scalars and fixed-length arrays.
#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryValue {
    Char(u8),
    String(String),
    Text(String),
    Bool(bool),
    Int(i32),
    BitField(u32),
    Float(f32),
    Double(f64),
    BoolArray(Vec<bool>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
    DoubleArray(Vec<f64>),
}

impl From<f32> for TelemetryValue {
    fn from(v: f32) -> Self {
        TelemetryValue::Float(v)
    }
}
impl From<f64> for TelemetryValue {
    fn from(v: f64) -> Self {
        TelemetryValue::Double(v)
    }
}
impl From<i32> for TelemetryValue {
    fn from(v: i32) -> Self {
        TelemetryValue::Int(v)
    }
}
impl From<u32> for TelemetryValue {
    fn from(v: u32) -> Self {
        TelemetryValue::BitField(v)
    }
}
impl From<u8> for TelemetryValue {
    fn from(v: u8) -> Self {
        TelemetryValue::Int(v as i32)
    }
}
impl From<i8> for TelemetryValue {
    fn from(v: i8) -> Self {
        TelemetryValue::Int(v as i32)
    }
}
impl From<u16> for TelemetryValue {
    fn from(v: u16) -> Self {
        TelemetryValue::Int(v as i32)
    }
}
impl From<i16> for TelemetryValue {
    fn from(v: i16) -> Self {
        TelemetryValue::Int(v as i32)
    }
}
impl From<i64> for TelemetryValue {
    fn from(v: i64) -> Self {
        TelemetryValue::Double(v as f64)
    }
}
impl From<u64> for TelemetryValue {
    fn from(v: u64) -> Self {
        TelemetryValue::Double(v as f64)
    }
}
impl From<bool> for TelemetryValue {
    fn from(v: bool) -> Self {
        TelemetryValue::Bool(v)
    }
}

impl std::fmt::Display for TelemetryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelemetryValue::Char(c) => write!(f, "'{}'", *c as char),
            TelemetryValue::String(s) | TelemetryValue::Text(s) => write!(f, "{}", s),
            TelemetryValue::Bool(b) => write!(f, "{}", b),
            TelemetryValue::Int(i) => write!(f, "{}", i),
            TelemetryValue::BitField(u) => write!(f, "0x{:08X}", u),
            TelemetryValue::Float(v) => write!(f, "{:.4}", v),
            TelemetryValue::Double(v) => write!(f, "{:.6}", v),
            TelemetryValue::BoolArray(a) => write!(f, "{:?}", a),
            TelemetryValue::IntArray(a) => write!(f, "{:?}", a),
            TelemetryValue::FloatArray(a) => write!(f, "{:?}", a),
            TelemetryValue::DoubleArray(a) => write!(f, "{:?}", a),
        }
    }
}

/// Metadata for a single telemetry variable (name, type, unit, description).
#[derive(Debug, Clone)]
pub struct VarMeta {
    pub name: String,
    pub type_name: String,
    pub unit: String,
    pub desc: String,
    pub count: u32,
}
