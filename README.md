# Serde TXT Record

[![Crates.io](https://img.shields.io/crates/v/serde-txtrecord.svg)](https://crates.io/crates/serde-txtrecord)
[![Documentation](https://docs.rs/serde-txtrecord/badge.svg)](https://docs.rs/serde-txtrecord)
[![License](https://img.shields.io/crates/l/serde-txtrecord.svg)](LICENSE)

A Rust serialization and deserialization library for TXT record format using Serde. This library provides custom serde support for converting Rust data structures to and from TXT record format commonly used in DNS TXT records and configuration files.

## Features

- **Simple key-value pairs**: `key: value` → `key=value`
- **Arrays**: `key: ["val", "bal"]` → `key_0=val, key_1=bal, key_len=2`
- **Objects**: `key: { foo: "val", bar: "bal" }` → `key.foo=val, key.bar=bal`
- **Record length limits**: Each `key=value` record can be limited to a maximum length (default: 255 characters)
- **Configurable separators and suffixes**: Customize array separators, object separators, and array length suffixes
- **All Rust primitive types**: Support for strings, numbers, booleans, options, and more

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-txtrecord = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

### Basic Usage

```rust
use serde::{Serialize, Deserialize};
use serde_txtrecord::{to_txt_records, from_txt_records};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        active: true,
    };

    // Serialize to TXT records
    let records = to_txt_records(&person)?;
    for (key, value) in &records {
        println!("{}={}", key, value);
    }
    // Output:
    // name=Alice
    // age=30
    // active=true

    // Deserialize back from TXT records
    let deserialized: Person = from_txt_records(records)?;
    assert_eq!(person, deserialized);

    Ok(())
}
```

### Working with Arrays

```rust
use serde::{Serialize, Deserialize};
use serde_txtrecord::to_txt_records;

#[derive(Serialize, Deserialize)]
struct Config {
    servers: Vec<String>,
    ports: Vec<u16>,
}

let config = Config {
    servers: vec!["web1.example.com".to_string(), "web2.example.com".to_string()],
    ports: vec![80, 443, 8080],
};

let records = to_txt_records(&config)?;
// Output:
// servers_0=web1.example.com
// servers_1=web2.example.com
// servers_len=2
// ports_0=80
// ports_1=443
// ports_2=8080
// ports_len=3
```

### Working with Nested Objects

```rust
use serde::{Serialize, Deserialize};
use serde_txtrecord::to_txt_records;

#[derive(Serialize, Deserialize)]
struct Database {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize)]
struct AppConfig {
    app_name: String,
    database: Database,
    features: Vec<String>,
}

let config = AppConfig {
    app_name: "MyApp".to_string(),
    database: Database {
        host: "localhost".to_string(),
        port: 5432,
    },
    features: vec!["auth".to_string(), "logging".to_string()],
};

let records = to_txt_records(&config)?;
// Output:
// app_name=MyApp
// database.host=localhost
// database.port=5432
// features_0=auth
// features_1=logging
// features_len=2
```

### Custom Configuration

```rust
use serde_txtrecord::{to_txt_records_with_config, TxtRecordConfig};

let config = TxtRecordConfig {
    array_separator: "-".to_string(),           // Use "-" instead of "_" for arrays
    object_separator: "/".to_string(),          // Use "/" instead of "." for objects
    record_len: 100,                           // Limit records to 100 characters
    array_len_suffix: ".count".to_string(),    // Use ".count" instead of "_len"
};

let data = vec!["item1", "item2", "item3"];
let records = to_txt_records_with_config(&data, config)?;
// Output with custom config:
// 0-0=item1
// 0-1=item2
// 0-2=item3
// 0.count=3
```

### Record Length Limits

```rust
use serde_txtrecord::{to_txt_records_with_config, TxtRecordConfig, TxtRecordError};

let config = TxtRecordConfig {
    record_len: 20, // Very short limit
    ..Default::default()
};

let data = std::collections::HashMap::from([
    ("short", "ok"),
    ("very_long_key_name", "this will exceed the limit"),
]);

match to_txt_records_with_config(&data, config) {
    Ok(records) => println!("Success: {:?}", records),
    Err(TxtRecordError::RecordTooLong { key, value, max_len, actual_len }) => {
        println!("Record '{}={}' is {} chars, exceeds limit of {}",
                 key, value, actual_len, max_len);
    }
    Err(e) => println!("Other error: {:?}", e),
}
```

## Configuration Options

The `TxtRecordConfig` struct allows you to customize the serialization format:

```rust
pub struct TxtRecordConfig {
    /// Separator for array indices (default: "_")
    pub array_separator: String,

    /// Separator for object fields (default: ".")
    pub object_separator: String,

    /// Maximum length for each record in format "key=value" (default: 255)
    pub record_len: usize,

    /// Suffix for array length metadata keys (default: "_len")
    pub array_len_suffix: String,
}
```

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_custom_array_length_suffix

# Run the example
cargo run --example book
```

### Test Coverage

The library includes comprehensive tests for:

- ✅ Basic serialization and deserialization roundtrips
- ✅ All primitive types (strings, integers, floats, booleans)
- ✅ Complex nested structures with objects and arrays
- ✅ Optional fields and `None` values
- ✅ Custom separator configurations
- ✅ Record length limits and error handling
- ✅ Custom array length suffixes
- ✅ Configuration compatibility and isolation

## Examples

Check out the [examples directory](examples/) for more detailed usage examples:

```bash
# Simple usage demonstration
cargo run --example simple

# Complex nested data structures
cargo run --example book

# Multiple nested vectors (matrices, game boards)
cargo run --example vecvec

# Multiple nested objects (employee data, configurations)
cargo run --example objobj
```

### Example Descriptions

- **`simple.rs`**: Basic usage with a simple struct, perfect for getting started. Uses comprehensive assertions to verify serialization correctness.
- **`book.rs`**: Comprehensive demonstration with complex nested structures, custom configuration, record length limits, and custom array length suffixes.
- **`vecvec.rs`**: Demonstrates deeply nested vectors (Vec<Vec<T>>), useful for matrices, game boards, and tabular data. Includes thorough validation of multi-level indexing.
- **`objobj.rs`**: Shows multiple levels of nested objects, ideal for configuration files and hierarchical data structures. Validates dot notation and optional fields.

All examples use assertions to verify correctness and can serve as integration tests for the library's functionality.

## Error Handling

The library provides detailed error types:

```rust
pub enum TxtRecordError {
    Custom(String),
    UnsupportedType(String),
    RecordTooLong {
        key: String,
        value: String,
        max_len: usize,
        actual_len: usize
    },
}

pub enum DeserializeError {
    Custom(String),
    InvalidFormat(String),
    MissingField(String),
    InvalidValue(String),
}
```

## Use Cases

This library is particularly useful for:

- **DNS TXT Records**: Storing structured configuration in DNS TXT records
- **Configuration Files**: Simple key-value configuration format
- **Environment Variables**: Flattening complex structures for environment variable storage
- **Log Structured Data**: Converting structured data to searchable key-value pairs
- **API Parameters**: Flattening nested objects for form-encoded or query parameters

## Performance Considerations

- The library is designed for correctness and flexibility rather than maximum performance
- For high-throughput scenarios, consider caching serialized results
- Record length validation happens during serialization, adding minimal overhead
- Memory usage scales linearly with the number of fields in your data structures

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Changelog

### v0.1.0

- Initial release
- Basic serialization and deserialization support
- Configurable separators and array length suffixes
- Record length limits
- Comprehensive test suite
