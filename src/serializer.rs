use crate::TelemetryValue;
use serde::ser::{self, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub enum SerializerError {
    Custom(String),
}

impl std::fmt::Display for SerializerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerializerError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for SerializerError {}

impl ser::Error for SerializerError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        SerializerError::Custom(msg.to_string())
    }
}

enum PathElement {
    Field,
    Index,
}

pub struct TelemetrySerializer {
    map: HashMap<String, TelemetryValue>,
    path_stack: Vec<PathElement>,
    seq_stack: Vec<Vec<TelemetryValue>>,
    current_seq_indices: Vec<usize>,
    current_key: String,
    key_lengths: Vec<usize>,
}

impl Default for TelemetrySerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetrySerializer {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            path_stack: Vec::new(),
            seq_stack: Vec::new(),
            current_seq_indices: Vec::new(),
            current_key: String::new(),
            key_lengths: Vec::new(),
        }
    }

    pub fn into_map(self) -> HashMap<String, TelemetryValue> {
        self.map
    }

    fn should_accumulate(&self) -> bool {
        if self.seq_stack.is_empty() {
            return false;
        }
        let last_index = self
            .path_stack
            .iter()
            .rposition(|x| matches!(x, PathElement::Index));
        if let Some(idx) = last_index {
            let has_field_after = self.path_stack[idx + 1..]
                .iter()
                .any(|x| matches!(x, PathElement::Field));
            !has_field_after
        } else {
            true
        }
    }

    fn push_field_key(&mut self, name: &str) {
        self.key_lengths.push(self.current_key.len());
        if !self.current_key.is_empty() {
            self.current_key.push('.');
        }
        self.current_key.push_str(name);
        self.path_stack.push(PathElement::Field);
    }

    fn push_index_key(&mut self, idx: usize) {
        use std::fmt::Write as _;
        self.key_lengths.push(self.current_key.len());
        write!(self.current_key, "[{}]", idx).ok();
        self.path_stack.push(PathElement::Index);
    }

    fn pop_key(&mut self) {
        if let Some(len) = self.key_lengths.pop() {
            self.current_key.truncate(len);
        }
        self.path_stack.pop();
    }
}

impl ser::Serializer for &mut TelemetrySerializer {
    type Ok = ();
    type Error = SerializerError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = ser::Impossible<(), Self::Error>;
    type SerializeMap = ser::Impossible<(), Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<(), Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::Bool(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::Bool(v));
        }
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::Int(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::Int(v));
        }
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::Char(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::Char(v));
        }
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i32(v as i32)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::BitField(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::BitField(v));
        }
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::Float(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::Float(v));
        }
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::Double(v));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::Double(v));
        }
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.push(TelemetryValue::String(v.to_string()));
            }
        } else {
            let key = self.current_key.clone();
            self.map.insert(key, TelemetryValue::String(v.to_string()));
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let vec = v.to_vec();
        if self.should_accumulate() {
            if let Some(last) = self.seq_stack.last_mut() {
                for b in vec {
                    last.push(TelemetryValue::Char(b));
                }
            }
        } else {
            let key = self.current_key.clone();
            self.map
                .insert(key, TelemetryValue::String(crate::decode_cp1252(&vec)));
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.current_seq_indices.push(0);
        self.seq_stack.push(Vec::new());
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.current_seq_indices.push(0);
        self.seq_stack.push(Vec::new());
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.current_seq_indices.push(0);
        self.seq_stack.push(Vec::new());
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(ser::Error::custom("tuple variants not supported"))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(ser::Error::custom("maps not supported"))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(ser::Error::custom("struct variants not supported"))
    }
}

impl ser::SerializeSeq for &mut TelemetrySerializer {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let index = self.current_seq_indices.last().copied().unwrap_or(0);
        self.push_index_key(index);
        let res = value.serialize(&mut **self);
        self.pop_key();

        if let Some(idx) = self.current_seq_indices.last_mut() {
            *idx += 1;
        }
        res
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.current_seq_indices.pop();
        if let Some(seq) = self.seq_stack.pop() {
            if let Some(last) = self.seq_stack.last_mut() {
                last.extend(seq);
            } else {
                let key = self.current_key.clone();
                if !seq.is_empty() {
                    match &seq[0] {
                        TelemetryValue::Bool(_) => {
                            let vec = seq
                                .into_iter()
                                .map(|v| match v {
                                    TelemetryValue::Bool(b) => b,
                                    _ => false,
                                })
                                .collect();
                            self.map.insert(key, TelemetryValue::BoolArray(vec));
                        }
                        TelemetryValue::Int(_) | TelemetryValue::BitField(_) => {
                            let vec = seq
                                .into_iter()
                                .map(|v| match v {
                                    TelemetryValue::Int(i) => i,
                                    TelemetryValue::BitField(u) => u as i32,
                                    TelemetryValue::Char(c) => c as i32,
                                    _ => 0,
                                })
                                .collect();
                            self.map.insert(key, TelemetryValue::IntArray(vec));
                        }
                        TelemetryValue::Float(_) => {
                            let vec = seq
                                .into_iter()
                                .map(|v| match v {
                                    TelemetryValue::Float(f) => f,
                                    _ => 0.0,
                                })
                                .collect();
                            self.map.insert(key, TelemetryValue::FloatArray(vec));
                        }
                        TelemetryValue::Double(_) => {
                            let vec = seq
                                .into_iter()
                                .map(|v| match v {
                                    TelemetryValue::Double(d) => d,
                                    _ => 0.0,
                                })
                                .collect();
                            self.map.insert(key, TelemetryValue::DoubleArray(vec));
                        }
                        TelemetryValue::Char(_) => {
                            let vec = seq
                                .into_iter()
                                .map(|v| match v {
                                    TelemetryValue::Char(c) => c as i32,
                                    _ => 0,
                                })
                                .collect();
                            self.map.insert(key, TelemetryValue::IntArray(vec));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

impl ser::SerializeStruct for &mut TelemetrySerializer {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        self.push_field_key(key);
        let res = value.serialize(&mut **self);
        self.pop_key();
        res
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl ser::SerializeTuple for &mut TelemetrySerializer {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let index = self.current_seq_indices.last().copied().unwrap_or(0);
        self.push_index_key(index);
        let res = value.serialize(&mut **self);
        self.pop_key();

        if let Some(idx) = self.current_seq_indices.last_mut() {
            *idx += 1;
        }
        res
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}

impl ser::SerializeTupleStruct for &mut TelemetrySerializer {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error> {
        let index = self.current_seq_indices.last().copied().unwrap_or(0);
        self.push_index_key(index);
        let res = value.serialize(&mut **self);
        self.pop_key();

        if let Some(idx) = self.current_seq_indices.last_mut() {
            *idx += 1;
        }
        res
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        <Self as ser::SerializeSeq>::end(self)
    }
}
