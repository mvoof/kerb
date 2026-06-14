/// A typed telemetry reading — scalars and fixed-length arrays.
///
/// iRacing uses `Char` / `String` / `Text` for different kinds of character data;
/// other sims map their string fields directly to `String` or `Text`.
#[derive(Debug, Clone, PartialEq)]
pub enum TelemetryValue {
    /// A single raw byte from a `char`-typed iRacing variable (not a Unicode scalar).
    Char(u8),
    /// A null-terminated CP-1252 string from a fixed-size `char[N]` iRacing buffer,
    /// decoded to UTF-8. Also used for arbitrary string fields in other sims.
    String(String),
    /// Free-form text (e.g. iRacing session YAML snippets, car/track names).
    /// Semantically the same storage as `String` but signals the value is human-readable prose.
    Text(String),
    /// `bool` scalar.
    Bool(bool),
    /// Signed 32-bit integer.
    Int(i32),
    /// Unsigned 32-bit bitmask — use bitwise ops to test individual flags.
    BitField(u32),
    /// 32-bit float (most iRacing channels: speed, RPM, temperatures, …).
    Float(f32),
    /// 64-bit float (high-precision channels).
    Double(f64),
    /// Array of `bool` values (e.g. per-wheel ABS active flags).
    BoolArray(Vec<bool>),
    /// Array of `i32` values (e.g. per-car lap counts).
    IntArray(Vec<i32>),
    /// Array of `f32` values (e.g. per-tyre temperatures).
    FloatArray(Vec<f32>),
    /// Array of `f64` values.
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

/// Metadata for a single telemetry variable.
#[derive(Debug, Clone)]
pub struct VarMeta {
    /// Sim-native variable name (e.g. `"Speed"`, `"RPM"`).
    pub name: String,
    /// Rust type name as a string: `"float"`, `"int"`, `"bool"`, `"bitfield"`, `"double"`, `"char"`.
    pub type_name: &'static str,
    /// Physical unit string from the sim (e.g. `"m/s"`, `"rpm"`, `""` if dimensionless).
    pub unit: String,
    /// Human-readable description from the sim (e.g. `"Lap distance percentage"`).
    pub desc: String,
    /// Number of elements — `1` for scalars, `>1` for fixed-length arrays (e.g. `4` for per-tyre data).
    pub count: u32,
}
