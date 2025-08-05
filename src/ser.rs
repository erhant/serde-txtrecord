use serde::ser;
use serde::{Serialize, Serializer};
use std::fmt;

use crate::TxtRecordConfig;

/// A serializer that converts Rust data structures to TXT record format
pub struct TxtRecordSerializer {
    config: TxtRecordConfig,
    output: Vec<(String, String)>,
    current_key: String,
}

impl TxtRecordSerializer {
    pub fn new() -> Self {
        Self::with_config(TxtRecordConfig::default())
    }

    pub fn with_config(config: TxtRecordConfig) -> Self {
        Self {
            config,
            output: Vec::new(),
            current_key: String::new(),
        }
    }

    pub fn finish(self) -> Vec<(String, String)> {
        self.output
    }

    fn push_record(&mut self, key: String, value: String) -> Result<(), TxtRecordError> {
        let record = format!("{}={}", key, value);
        let record_len = record.len();

        if record_len > self.config.record_len {
            return Err(TxtRecordError::RecordTooLong {
                key,
                value,
                max_len: self.config.record_len,
                actual_len: record_len,
            });
        }

        self.output.push((key, value));
        Ok(())
    }
}

impl Default for TxtRecordSerializer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum TxtRecordError {
    Custom(String),
    UnsupportedType(String),
    RecordTooLong {
        key: String,
        value: String,
        max_len: usize,
        actual_len: usize,
    },
}

impl fmt::Display for TxtRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TxtRecordError::Custom(msg) => write!(f, "{}", msg),
            TxtRecordError::UnsupportedType(typ) => write!(f, "Unsupported type: {}", typ),
            TxtRecordError::RecordTooLong {
                key,
                value,
                max_len,
                actual_len,
            } => {
                write!(
                    f,
                    "Record '{}={}' is too long: {} characters exceeds maximum of {}",
                    key, value, actual_len, max_len
                )
            }
        }
    }
}

impl std::error::Error for TxtRecordError {}

impl ser::Error for TxtRecordError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        TxtRecordError::Custom(msg.to_string())
    }
}

impl<'a> Serializer for &'a mut TxtRecordSerializer {
    type Ok = ();
    type Error = TxtRecordError;

    type SerializeSeq = SeqSerializer<'a>;
    type SerializeTuple = SeqSerializer<'a>;
    type SerializeTupleStruct = SeqSerializer<'a>;
    type SerializeTupleVariant = SeqSerializer<'a>;
    type SerializeMap = MapSerializer<'a>;
    type SerializeStruct = MapSerializer<'a>;
    type SerializeStructVariant = MapSerializer<'a>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.push_record(self.current_key.clone(), v.to_string())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let s = String::from_utf8_lossy(v);
        self.serialize_str(&s)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(SeqSerializer::new(self, len))
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(MapSerializer::new(self))
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.serialize_map(Some(len))
    }
}

pub struct SeqSerializer<'a> {
    ser: &'a mut TxtRecordSerializer,
    base_key: String,
    index: usize,
    len: Option<usize>,
}

impl<'a> SeqSerializer<'a> {
    fn new(ser: &'a mut TxtRecordSerializer, len: Option<usize>) -> Self {
        let base_key = ser.current_key.clone();
        Self {
            ser,
            base_key,
            index: 0,
            len,
        }
    }
}

impl<'a> ser::SerializeSeq for SeqSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let key = format!(
            "{}{}{}",
            self.base_key, self.ser.config.array_separator, self.index
        );
        self.ser.current_key = key;
        value.serialize(&mut *self.ser)?;
        self.index += 1;
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // Add length metadata
        if let Some(len) = self.len.or(Some(self.index)) {
            let len_key = format!("{}{}", self.base_key, self.ser.config.array_len_suffix);
            self.ser.push_record(len_key, len.to_string())?;
        }
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for SeqSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleStruct for SeqSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

impl<'a> ser::SerializeTupleVariant for SeqSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeSeq::end(self)
    }
}

pub struct MapSerializer<'a> {
    ser: &'a mut TxtRecordSerializer,
    base_key: String,
}

impl<'a> MapSerializer<'a> {
    fn new(ser: &'a mut TxtRecordSerializer) -> Self {
        let base_key = ser.current_key.clone();
        Self { ser, base_key }
    }
}

impl<'a> ser::SerializeMap for MapSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let mut key_ser = TxtRecordSerializer::with_config(self.ser.config.clone());
        key.serialize(&mut key_ser)?;

        if let Some((_, key_str)) = key_ser.finish().into_iter().next() {
            if self.base_key.is_empty() {
                self.ser.current_key = key_str;
            } else {
                self.ser.current_key = format!(
                    "{}{}{}",
                    self.base_key, self.ser.config.object_separator, key_str
                );
            }
        }
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for MapSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        if self.base_key.is_empty() {
            self.ser.current_key = key.to_string();
        } else {
            self.ser.current_key = format!(
                "{}{}{}",
                self.base_key, self.ser.config.object_separator, key
            );
        }
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for MapSerializer<'a> {
    type Ok = ();
    type Error = TxtRecordError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        ser::SerializeStruct::serialize_field(self, key, value)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        ser::SerializeStruct::end(self)
    }
}

/// Serialize a value to TXT record format
pub fn to_txt_records<T>(value: &T) -> Result<Vec<(String, String)>, TxtRecordError>
where
    T: Serialize,
{
    to_txt_records_with_config(value, TxtRecordConfig::default())
}

/// Serialize a value to TXT record format with custom configuration
pub fn to_txt_records_with_config<T>(
    value: &T,
    config: TxtRecordConfig,
) -> Result<Vec<(String, String)>, TxtRecordError>
where
    T: Serialize,
{
    let mut serializer = TxtRecordSerializer::with_config(config);
    value.serialize(&mut serializer)?;
    Ok(serializer.finish())
}
