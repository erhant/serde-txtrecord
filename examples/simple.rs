use serde::{Deserialize, Serialize};
use serde_txtrecord::{from_txt_records, to_txt_records};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple person struct
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        active: true,
    };

    // Serialize to TXT records
    let records = to_txt_records(&person)?;

    // Verify the expected records are generated
    assert_eq!(records.len(), 3, "Should generate exactly 3 records");

    // Check each expected record exists
    let records_map: std::collections::HashMap<String, String> = records.iter().cloned().collect();
    assert_eq!(records_map.get("name"), Some(&"Alice".to_string()));
    assert_eq!(records_map.get("age"), Some(&"30".to_string()));
    assert_eq!(records_map.get("active"), Some(&"true".to_string()));

    // Deserialize back from TXT records
    let deserialized: Person = from_txt_records(records)?;

    // Verify roundtrip serialization worked perfectly
    assert_eq!(
        person, deserialized,
        "Roundtrip serialization must preserve all data"
    );

    // Verify individual fields match
    assert_eq!(deserialized.name, "Alice");
    assert_eq!(deserialized.age, 30);
    assert_eq!(deserialized.active, true);

    Ok(())
}
