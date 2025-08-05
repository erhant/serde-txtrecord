use serde::{Deserialize, Serialize};
use serde_txtrecord::{from_txt_records, to_txt_records, to_txt_records_with_config, TxtRecordConfig};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Author {
    name: String,
    email: Option<String>,
    biography: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Publisher {
    name: String,
    location: String,
    founded: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Metadata {
    pages: u32,
    word_count: Option<u64>,
    language: String,
    isbn: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Book {
    title: String,
    authors: Vec<Author>,
    publisher: Publisher,
    publication_year: u16,
    price: f64,
    available: bool,
    genres: Vec<String>,
    metadata: Metadata,
    awards: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let book = Book {
        title: "The Rust Programming Language".to_string(),
        authors: vec![
            Author {
                name: "Steve Klabnik".to_string(),
                email: Some("steve@example.com".to_string()),
                biography: "Technical writer and Rust core team member".to_string(),
            },
            Author {
                name: "Carol Nichols".to_string(),
                email: Some("carol@example.com".to_string()),
                biography: "Rust consultant and educator".to_string(),
            },
        ],
        publisher: Publisher {
            name: "No Starch Press".to_string(),
            location: "San Francisco, CA".to_string(),
            founded: 1994,
        },
        publication_year: 2018,
        price: 39.95,
        available: true,
        genres: vec![
            "Programming".to_string(),
            "Computer Science".to_string(),
            "Systems Programming".to_string(),
        ],
        metadata: Metadata {
            pages: 552,
            word_count: Some(180000),
            language: "English".to_string(),
            isbn: "978-1-59327-828-1".to_string(),
        },
        awards: Some(vec![
            "Best Programming Book 2019".to_string(),
            "Developer's Choice Award".to_string(),
        ]),
    };

    let records = to_txt_records(&book)?;

    // sort records by key for better readability
    let mut sorted_records = records.clone();
    sorted_records.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in &sorted_records {
        println!("{}={}", key, value);
    }
    println!("\nTotal records: {}", records.len());

    // sanity check
    let deserialized_book: Book = from_txt_records(records)?;
    assert_eq!(
        book, deserialized_book,
        "deserialized book does not match original"
    );

    println!("\n--- Testing Record Length Limits ---");
    
    // Test with a custom record length limit
    let config = TxtRecordConfig {
        array_separator: "_".to_string(),
        object_separator: ".".to_string(),
        record_len: 50, // Very restrictive limit for demonstration
        array_len_suffix: "_len".to_string(),
    };

    #[derive(Serialize)]
    struct TestRecord {
        short: String,
        medium_length_key: String,
        very_long_key_name_that_will_definitely_exceed_limits: String,
    }

    let test_data = TestRecord {
        short: "ok".to_string(),
        medium_length_key: "this should work".to_string(), 
        very_long_key_name_that_will_definitely_exceed_limits: "fail".to_string(),
    };

    match to_txt_records_with_config(&test_data, config) {
        Ok(records) => {
            println!("Unexpected success with restrictive limits: {:?}", records);
        }
        Err(e) => {
            println!("Expected error with restrictive record length limit: {}", e);
        }
    }

    println!("\n--- Testing Custom Array Length Suffix ---");
    
    // Test with a custom array length suffix
    let custom_config = TxtRecordConfig {
        array_separator: "_".to_string(),
        object_separator: ".".to_string(),
        record_len: 255,
        array_len_suffix: ".count".to_string(), // Custom suffix instead of "_len"
    };

    #[derive(Serialize)]
    struct ArrayTest {
        tags: Vec<String>,
        numbers: Vec<i32>,
    }

    let array_data = ArrayTest {
        tags: vec!["rust".to_string(), "programming".to_string(), "serde".to_string()],
        numbers: vec![1, 2, 3, 4, 5],
    };

    match to_txt_records_with_config(&array_data, custom_config) {
        Ok(records) => {
            println!("Custom array length suffix demo:");
            for (key, value) in &records {
                println!("{}={}", key, value);
            }
            
            // Look for the custom suffix
            let has_custom_suffix = records.iter().any(|(key, _)| key.ends_with(".count"));
            let has_default_suffix = records.iter().any(|(key, _)| key.ends_with("_len"));
            
            println!("Uses custom '.count' suffix: {}", has_custom_suffix);
            println!("Uses default '_len' suffix: {}", has_default_suffix);
        }
        Err(e) => {
            println!("Error with custom array length suffix: {}", e);
        }
    }

    Ok(())
}
