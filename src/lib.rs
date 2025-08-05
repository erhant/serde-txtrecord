//! # Serde TXT Record
//!
//! A serialization and deserialization library for TXT records.
//!
//! This library provides custom serde support for converting Rust data structures
//! to and from TXT record format with the following patterns:
//!
//! - Simple key-value pairs: `key: value` → `key=value`
//! - Arrays: `key: ["val", "bal"]` → `key_0=val, key_1=bal, key_len=2`
//! - Objects: `key: { foo: "val", bar: "bal" }` → `key.foo=val, key.bar=bal`
//! - Record length limits: Each `key=value` record can be limited to a maximum length (default: 255 characters)
//! - Configurable separators and suffixes: Customize array separators, object separators, and array length suffixes
//!
//! ## Example
//!
//! ```rust
//! use serde::{Serialize, Deserialize};
//! use serde_txtrecord::{to_txt_records, from_txt_records};
//!
//! #[derive(Serialize, Deserialize, Debug, PartialEq)]
//! struct Person {
//!     name: String,
//!     age: u32,
//! }
//!
//! let person = Person {
//!     name: "Alice".to_string(),
//!     age: 30,
//! };
//!
//! // Serialize to TXT records
//! let records = to_txt_records(&person).unwrap();
//!
//! // Deserialize back from TXT records
//! let deserialized: Person = from_txt_records(records).unwrap();
//! assert_eq!(person, deserialized);
//! ```

pub mod de;
pub mod ser;

// Re-export main functionality
pub use de::{
    DeserializeError, TxtRecordDeserializer, from_txt_records, from_txt_records_with_config,
};
pub use ser::{
    TxtRecordConfig, TxtRecordError, TxtRecordSerializer, to_txt_records,
    to_txt_records_with_config,
};

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct TestStruct {
        name: String,
        age: u32,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct NestedStruct {
        person: TestStruct,
        tags: Vec<String>,
    }

    #[test]
    fn test_simple_key_value_roundtrip() {
        let mut map = HashMap::new();
        map.insert("key", "value");

        let records = to_txt_records(&map).unwrap();
        let result: HashMap<String, String> = from_txt_records(records).unwrap();

        let mut expected = HashMap::new();
        expected.insert("key".to_string(), "value".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_struct_roundtrip() {
        let test = TestStruct {
            name: "Alice".to_string(),
            age: 30,
        };

        let records = to_txt_records(&test).unwrap();
        let result: TestStruct = from_txt_records(records).unwrap();
        assert_eq!(test, result);
    }

    #[test]
    fn test_array_roundtrip() {
        let mut map = HashMap::new();
        map.insert("items", vec!["a", "b", "c"]);

        let records = to_txt_records(&map).unwrap();
        let result: HashMap<String, Vec<String>> = from_txt_records(records).unwrap();

        let mut expected = HashMap::new();
        expected.insert(
            "items".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_nested_object_roundtrip() {
        let nested = NestedStruct {
            person: TestStruct {
                name: "Bob".to_string(),
                age: 25,
            },
            tags: vec!["developer".to_string(), "rust".to_string()],
        };

        let records = to_txt_records(&nested).unwrap();
        let result: NestedStruct = from_txt_records(records).unwrap();
        assert_eq!(nested, result);
    }

    #[test]
    fn test_custom_separators_roundtrip() {
        let config = TxtRecordConfig {
            array_separator: "-".to_string(),
            object_separator: "/".to_string(),
            record_len: 255,
            array_len_suffix: "_len".to_string(),
        };

        let mut map = HashMap::new();
        map.insert("items", vec!["x", "y"]);

        let records = to_txt_records_with_config(&map, config.clone()).unwrap();
        let result: HashMap<String, Vec<String>> =
            from_txt_records_with_config(records, config).unwrap();

        let mut expected = HashMap::new();
        expected.insert("items".to_string(), vec!["x".to_string(), "y".to_string()]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_numeric_types_roundtrip() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Numbers {
            byte: u8,
            small: i16,
            large: i64,
            float: f32,
            double: f64,
            flag: bool,
        }

        let nums = Numbers {
            byte: 255,
            small: -1000,
            large: 1234567890,
            float: 3.14,
            double: 2.718281828,
            flag: true,
        };

        let records = to_txt_records(&nums).unwrap();
        let result: Numbers = from_txt_records(records).unwrap();
        assert_eq!(nums, result);
    }

    #[test]
    fn test_optional_fields() {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct WithOptional {
            required: String,
            optional: Option<String>,
        }

        let with_some = WithOptional {
            required: "always".to_string(),
            optional: Some("sometimes".to_string()),
        };

        let records = to_txt_records(&with_some).unwrap();
        let result: WithOptional = from_txt_records(records).unwrap();
        assert_eq!(with_some, result);

        let with_none = WithOptional {
            required: "always".to_string(),
            optional: None,
        };

        let records = to_txt_records(&with_none).unwrap();
        let result: WithOptional = from_txt_records(records).unwrap();
        assert_eq!(with_none, result);
    }

    #[test]
    fn test_record_length_limit() {
        #[derive(Serialize, Debug)]
        struct LongValue {
            short_key: String,
        }

        // Test with default 255 character limit
        // "short_key=" is 10 chars, so we can have 245 chars in value
        let long_data = LongValue {
            short_key: "a".repeat(245), // short_key=aaa... should be exactly 255
        };

        let result = to_txt_records(&long_data);
        assert!(result.is_ok(), "Should succeed with 255 char limit");

        // Test exceeding the limit
        let too_long_data = LongValue {
            short_key: "a".repeat(246), // short_key=aaa... will be 256 > 255
        };

        let result = to_txt_records(&too_long_data);
        assert!(result.is_err(), "Should fail when exceeding 255 char limit");

        match result.unwrap_err() {
            TxtRecordError::RecordTooLong {
                max_len,
                actual_len,
                ..
            } => {
                assert_eq!(max_len, 255);
                assert_eq!(actual_len, 256); // 10 + 246 = 256
            }
            _ => panic!("Expected RecordTooLong error"),
        }
    }

    #[test]
    fn test_custom_record_length_limit() {
        let config = TxtRecordConfig {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            record_len: 20, // Very short limit for testing
            array_len_suffix: "_len".to_string(),
        };

        let mut map = HashMap::new();
        map.insert("key", "short");

        // This should work: "key=short" is 9 characters
        let result = to_txt_records_with_config(&map, config.clone());
        assert!(result.is_ok());

        // This should fail: "key=very_long_value" is 20 characters (at limit)
        map.insert("key", "very_long_value");
        let result = to_txt_records_with_config(&map, config.clone());
        assert!(result.is_ok()); // exactly at limit should work

        // This should fail: "key=very_long_value_that_exceeds" is > 20 characters
        map.insert("key", "very_long_value_that_exceeds");
        let result = to_txt_records_with_config(&map, config);
        assert!(result.is_err());

        match result.unwrap_err() {
            TxtRecordError::RecordTooLong {
                max_len,
                actual_len,
                ..
            } => {
                assert_eq!(max_len, 20);
                assert!(actual_len > 20);
            }
            _ => panic!("Expected RecordTooLong error"),
        }
    }

    #[test]
    fn test_custom_array_length_suffix() {
        let config = TxtRecordConfig {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            record_len: 255,
            array_len_suffix: ".count".to_string(), // Custom suffix
        };

        let mut map = HashMap::new();
        map.insert("items", vec!["apple", "banana", "cherry"]);

        // Serialize with custom length suffix
        let records = to_txt_records_with_config(&map, config.clone()).unwrap();

        // Verify that the length is stored with custom suffix
        let records_map: HashMap<String, String> = records.iter().cloned().collect();
        assert_eq!(records_map.get("items.count"), Some(&"3".to_string()));
        assert_eq!(records_map.get("items_0"), Some(&"apple".to_string()));
        assert_eq!(records_map.get("items_1"), Some(&"banana".to_string()));
        assert_eq!(records_map.get("items_2"), Some(&"cherry".to_string()));

        // Deserialize with the same config should work
        let result: HashMap<String, Vec<String>> =
            from_txt_records_with_config(records, config).unwrap();

        let mut expected = HashMap::new();
        expected.insert(
            "items".to_string(),
            vec![
                "apple".to_string(),
                "banana".to_string(),
                "cherry".to_string(),
            ],
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_length_suffix_roundtrip_compatibility() {
        // Test that different configurations can't accidentally cross-deserialize
        let config1 = TxtRecordConfig {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            record_len: 255,
            array_len_suffix: "_len".to_string(),
        };

        let config2 = TxtRecordConfig {
            array_separator: "_".to_string(),
            object_separator: ".".to_string(),
            record_len: 255,
            array_len_suffix: ".size".to_string(),
        };

        let mut map = HashMap::new();
        map.insert("data", vec!["x", "y"]);

        // Serialize with config1
        let records1 = to_txt_records_with_config(&map, config1.clone()).unwrap();

        // Serialize with config2
        let records2 = to_txt_records_with_config(&map, config2.clone()).unwrap();

        // Verify they produce different length keys
        let map1: HashMap<String, String> = records1.iter().cloned().collect();
        let map2: HashMap<String, String> = records2.iter().cloned().collect();

        assert!(map1.contains_key("data_len"));
        assert!(!map1.contains_key("data.size"));

        assert!(map2.contains_key("data.size"));
        assert!(!map2.contains_key("data_len"));

        // Each should deserialize correctly with its own config
        let result1: HashMap<String, Vec<String>> =
            from_txt_records_with_config(records1, config1).unwrap();
        let result2: HashMap<String, Vec<String>> =
            from_txt_records_with_config(records2, config2).unwrap();

        assert_eq!(result1, result2); // Both should produce the same logical result
    }
}
