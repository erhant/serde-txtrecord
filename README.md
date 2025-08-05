<p align="center">
  <h1 align="center">
    Serde TXT Record
  </h1>
  <p align="center">
    <i>A Rust serialization and deserialization library for TXT record format using Serde.</i>
  </p>
</p>

<p align="center">
    <a href="https://opensource.org/license/mit" target="_blank">
        <img alt="License: MIT" src="https://img.shields.io/badge/license-MIT-7CB9E8.svg">
    </a>
    <a href="./.github/workflows/tests.yml" target="_blank">
        <img alt="Workflow: Tests" src="https://github.com/erhant/serde-txtrecord/actions/workflows/tests.yml/badge.svg?branch=main">
    </a>
    <a href="https://github.com/foresterre/cargo-msrv" target="_blank">
        <img alt="MSRV" src="https://img.shields.io/badge/1.78.0-F74B01?logo=rust&logoColor=white&label=msrv"/>
    </a>
</p>

This library provides custom serde support for converting Rust data structures to and from TXT record format commonly used in DNS TXT records and configuration files.

- [x] **Simple key-value pairs**: `key: value` → `key=value`
- [x] **Arrays**: `key: ["val", "bal"]` → `key_0=val, key_1=bal, key_len=2`
- [x] **Objects**: `key: { foo: "val", bar: "bal" }` → `key.foo=val, key.bar=bal`
- [x] **Record length limits**: Each `key=value` record can be limited to a maximum length (default: 255 characters)
- [x] **Configurable separators and suffixes**: Customize array separators, object separators, and array length suffixes
- [x] **All Rust primitive types**: Support for strings, numbers, booleans, options, and more

This library is particularly useful for:

- **DNS TXT Records**: Storing structured configuration in DNS TXT records
- **Configuration Files**: Simple key-value configuration format
- **Environment Variables**: Flattening complex structures for environment variable storage
- **Log Structured Data**: Converting structured data to searchable key-value pairs
- **API Parameters**: Flattening nested objects for form-encoded or query parameters

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde-txtrecord = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
```

## Usage

You can use the `to_txt_records` and `from_txt_records` functions on a struct that has the `Serialize` and `Deserialize` traits, respectively.

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

    // serialize to TXT records
    let records = to_txt_records(&person)?;
    // > name=Alice
    // > age=30
    // > active=true

    // Deserialize back from TXT records
    let deserialized: Person = from_txt_records(records)?;
    assert_eq!(person, deserialized);

    Ok(())
}
```

You can provide a custom configuration as well, to avoid clashing names; use with `to_txt_records_with_config` and `from_txt_records_with_config`.

```rust
use serde_txtrecord::{to_txt_records_with_config, TxtRecordConfig};

let config = TxtRecordConfig {
    array_separator: "-".to_string(),         // use "-" instead of "_" for arrays
    object_separator: "/".to_string(),        // use "/" instead of "." for objects
    record_len: 100,                          // limit records to 100 characters
    array_len_suffix: ".count".to_string(),   // use ".count" instead of "_len"
};

let data = vec!["item1", "item2", "item3"];
let records = to_txt_records_with_config(&data, config)?;
// 0-0=item1
// 0-1=item2
// 0-2=item3
// 0.count=3
```

## Testing

Run the test suite:

```bash
cargo test

# run an example
cargo run --example complex
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the [MIT](./LICENSE) License.
