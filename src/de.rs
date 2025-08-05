use serde::de;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;

use crate::ser::TxtRecordConfig;

#[derive(Debug)]
pub enum DeserializeError {
    Custom(String),
    InvalidFormat(String),
    MissingField(String),
    InvalidValue(String),
}

impl fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeserializeError::Custom(msg) => write!(f, "{}", msg),
            DeserializeError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            DeserializeError::MissingField(field) => write!(f, "Missing field: {}", field),
            DeserializeError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl std::error::Error for DeserializeError {}

impl de::Error for DeserializeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        DeserializeError::Custom(msg.to_string())
    }
}

/// A deserializer that converts TXT record format back to Rust data structures
pub struct TxtRecordDeserializer {
    config: TxtRecordConfig,
    records: HashMap<String, String>,
    current_key: String,
}

impl TxtRecordDeserializer {
    pub fn new(records: Vec<(String, String)>) -> Self {
        Self::with_config(records, TxtRecordConfig::default())
    }

    pub fn with_config(records: Vec<(String, String)>, config: TxtRecordConfig) -> Self {
        let records_map = records.into_iter().collect();
        Self {
            config,
            records: records_map,
            current_key: String::new(),
        }
    }

    fn get_value(&self, key: &str) -> Option<&String> {
        self.records.get(key)
    }

    fn get_array_length(&self, base_key: &str) -> Option<usize> {
        let len_key = format!("{}{}", base_key, self.config.array_len_suffix);
        self.get_value(&len_key).and_then(|s| s.parse().ok())
    }

    fn get_object_keys(&self, base_key: &str) -> Vec<String> {
        let prefix = if base_key.is_empty() {
            String::new()
        } else {
            format!("{}{}", base_key, self.config.object_separator)
        };

        let mut keys = std::collections::HashSet::new();
        for record_key in self.records.keys() {
            if base_key.is_empty() {
                // for root level, any key that doesn't contain separators is a direct key
                if !record_key.contains(&self.config.object_separator)
                    && !record_key.contains(&self.config.array_separator)
                    && !record_key.ends_with(&self.config.array_len_suffix)
                {
                    keys.insert(record_key.clone());
                } else if let Some(dot_pos) = record_key.find(&self.config.object_separator) {
                    // or the first part of a nested key
                    keys.insert(record_key[..dot_pos].to_string());
                } else if let Some(array_pos) = record_key.find(&self.config.array_separator) {
                    // or the base name of an array
                    let base_name = &record_key[..array_pos];
                    if !base_name.is_empty() {
                        keys.insert(base_name.to_string());
                    }
                }
            } else if record_key.starts_with(&prefix) {
                let suffix = &record_key[prefix.len()..];
                if let Some(dot_pos) = suffix.find(&self.config.object_separator) {
                    keys.insert(suffix[..dot_pos].to_string());
                } else if !suffix.contains(&self.config.array_separator)
                    && !suffix.ends_with(&self.config.array_len_suffix)
                {
                    keys.insert(suffix.to_string());
                }
            }
        }
        keys.into_iter().collect()
    }
}

impl<'de> Deserializer<'de> for &mut TxtRecordDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // try to determine the type based on the current key
        if let Some(value) = self.get_value(&self.current_key) {
            // it's a simple value
            visitor.visit_str(value)
        } else if self.get_array_length(&self.current_key).is_some() {
            // it's an array
            self.deserialize_seq(visitor)
        } else if !self.get_object_keys(&self.current_key).is_empty() {
            // it's an object
            self.deserialize_map(visitor)
        } else {
            Err(DeserializeError::MissingField(self.current_key.clone()))
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<bool>() {
                Ok(b) => visitor.visit_bool(b),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as bool",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<i8>() {
                Ok(n) => visitor.visit_i8(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as i8",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<i16>() {
                Ok(n) => visitor.visit_i16(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as i16",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<i32>() {
                Ok(n) => visitor.visit_i32(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as i32",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<i64>() {
                Ok(n) => visitor.visit_i64(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as i64",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<u8>() {
                Ok(n) => visitor.visit_u8(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as u8",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<u16>() {
                Ok(n) => visitor.visit_u16(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as u16",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<u32>() {
                Ok(n) => visitor.visit_u32(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as u32",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<u64>() {
                Ok(n) => visitor.visit_u64(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as u64",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<f32>() {
                Ok(n) => visitor.visit_f32(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as f32",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => match value.parse::<f64>() {
                Ok(n) => visitor.visit_f64(n),
                Err(_) => Err(DeserializeError::InvalidValue(format!(
                    "Cannot parse '{}' as f64",
                    value
                ))),
            },
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => {
                let mut chars = value.chars();
                match (chars.next(), chars.next()) {
                    (Some(c), None) => visitor.visit_char(c),
                    _ => Err(DeserializeError::InvalidValue(format!(
                        "Cannot parse '{}' as char",
                        value
                    ))),
                }
            }
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => visitor.visit_str(value),
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        match self.get_value(&self.current_key) {
            Some(value) => visitor.visit_bytes(value.as_bytes()),
            None => Err(DeserializeError::MissingField(self.current_key.clone())),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        // For options, check if we have either a direct value, an array, or an object
        if self.get_value(&self.current_key).is_some()
            || self.get_array_length(&self.current_key).is_some()
            || !self.get_object_keys(&self.current_key).is_empty()
        {
            visitor.visit_some(self)
        } else {
            visitor.visit_none()
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let len = self.get_array_length(&self.current_key).ok_or_else(|| {
            DeserializeError::MissingField(format!(
                "{}{}",
                self.current_key, self.config.array_len_suffix
            ))
        })?;

        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SeqAccess::new(self, len))
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let keys = self.get_object_keys(&self.current_key);
        visitor.visit_map(MapAccess::new(self, keys))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        let keys = fields.iter().map(|s| s.to_string()).collect();
        visitor.visit_map(MapAccess::new(self, keys))
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_enum(EnumAccess::new(self))
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct SeqAccess<'a> {
    de: &'a mut TxtRecordDeserializer,
    base_key: String,
    index: usize,
    len: usize,
}

impl<'a> SeqAccess<'a> {
    fn new(de: &'a mut TxtRecordDeserializer, len: usize) -> Self {
        let base_key = de.current_key.clone();
        Self {
            de,
            base_key,
            index: 0,
            len,
        }
    }
}

impl<'de, 'a> de::SeqAccess<'de> for SeqAccess<'a> {
    type Error = DeserializeError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.index >= self.len {
            return Ok(None);
        }

        let key = format!(
            "{}{}{}",
            self.base_key, self.de.config.array_separator, self.index
        );
        self.de.current_key = key;
        self.index += 1;

        seed.deserialize(&mut *self.de).map(Some)
    }
}

struct MapAccess<'a> {
    de: &'a mut TxtRecordDeserializer,
    base_key: String,
    keys: Vec<String>,
    key_index: usize,
}

impl<'a> MapAccess<'a> {
    fn new(de: &'a mut TxtRecordDeserializer, keys: Vec<String>) -> Self {
        let base_key = de.current_key.clone();
        Self {
            de,
            base_key,
            keys,
            key_index: 0,
        }
    }
}

impl<'de, 'a> de::MapAccess<'de> for MapAccess<'a> {
    type Error = DeserializeError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.key_index >= self.keys.len() {
            return Ok(None);
        }

        let key = &self.keys[self.key_index];
        // For map keys, we return the key string directly, not deserialize from records
        let key_deserializer = &mut KeyDeserializer { key: key.clone() };
        seed.deserialize(key_deserializer).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let key = &self.keys[self.key_index];
        self.key_index += 1;

        if self.base_key.is_empty() {
            self.de.current_key = key.clone();
        } else {
            self.de.current_key = format!(
                "{}{}{}",
                self.base_key, self.de.config.object_separator, key
            );
        }

        seed.deserialize(&mut *self.de)
    }
}

struct KeyDeserializer {
    key: String,
}

impl<'de> Deserializer<'de> for &mut KeyDeserializer {
    type Error = DeserializeError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(&self.key)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(&self.key)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_string(self.key.clone())
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct enum ignored_any
    }
}

struct EnumAccess<'a> {
    de: &'a mut TxtRecordDeserializer,
}

impl<'a> EnumAccess<'a> {
    fn new(de: &'a mut TxtRecordDeserializer) -> Self {
        Self { de }
    }
}

impl<'de, 'a> de::EnumAccess<'de> for EnumAccess<'a> {
    type Error = DeserializeError;
    type Variant = VariantAccess<'a>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(&mut *self.de)?;
        Ok((variant, VariantAccess::new(self.de)))
    }
}

struct VariantAccess<'a> {
    de: &'a mut TxtRecordDeserializer,
}

impl<'a> VariantAccess<'a> {
    fn new(de: &'a mut TxtRecordDeserializer) -> Self {
        Self { de }
    }
}

impl<'de, 'a> de::VariantAccess<'de> for VariantAccess<'a> {
    type Error = DeserializeError;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: de::Visitor<'de>,
    {
        self.de.deserialize_struct("", fields, visitor)
    }
}

/// Deserialize TXT records back to a Rust data structure
pub fn from_txt_records<T>(records: Vec<(String, String)>) -> Result<T, DeserializeError>
where
    T: for<'de> Deserialize<'de>,
{
    from_txt_records_with_config(records, TxtRecordConfig::default())
}

/// Deserialize TXT records back to a Rust data structure with custom configuration
pub fn from_txt_records_with_config<T>(
    records: Vec<(String, String)>,
    config: TxtRecordConfig,
) -> Result<T, DeserializeError>
where
    T: for<'de> Deserialize<'de>,
{
    let mut deserializer = TxtRecordDeserializer::with_config(records, config);
    T::deserialize(&mut deserializer)
}
