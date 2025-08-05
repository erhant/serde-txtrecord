use serde::{Deserialize, Serialize};
use serde_txtrecord::{from_txt_records_with_config, to_txt_records_with_config, TxtRecordConfig};

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

    let custom_config = TxtRecordConfig {
        array_len_suffix: ".count".to_string(), // instead of "_len"
        ..Default::default()
    };
    let records = to_txt_records_with_config(&book, custom_config.clone())?;

    // sort records by key for better readability
    let mut sorted_records = records.clone();
    sorted_records.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in &sorted_records {
        println!("{}={}", key, value);
    }

    // sanity check
    let deserialized_book: Book = from_txt_records_with_config(records, custom_config)?;
    assert_eq!(
        book, deserialized_book,
        "deserialized book does not match original"
    );

    Ok(())
}
