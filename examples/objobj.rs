use serde::{Deserialize, Serialize};
use serde_txtrecord::{from_txt_records, to_txt_records};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Address {
    street: String,
    city: String,
    country: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Contact {
    email: String,
    phone: String,
    address: Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Company {
    name: String,
    headquarters: Address,
    branch_office: Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Employee {
    name: String,
    id: u32,
    contact: Contact,
    company: Company,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct DatabaseConfig {
    host: String,
    port: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct RedisConfig {
    host: String,
    port: u16,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ServiceConfig {
    database: DatabaseConfig,
    cache: RedisConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct ApplicationConfig {
    app_name: String,
    version: String,
    services: ServiceConfig,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Deeply nested employee structure
    let employee = Employee {
        name: "John Doe".to_string(),
        id: 12345,
        contact: Contact {
            email: "john.doe@example.com".to_string(),
            phone: "+1-555-0123".to_string(),
            address: Address {
                street: "123 Main St".to_string(),
                city: "New York".to_string(),
                country: "USA".to_string(),
            },
        },
        company: Company {
            name: "Tech Corp".to_string(),
            headquarters: Address {
                street: "456 Corporate Blvd".to_string(),
                city: "San Francisco".to_string(),
                country: "USA".to_string(),
            },
            branch_office: Address {
                street: "789 Branch Ave".to_string(),
                city: "Austin".to_string(),
                country: "USA".to_string(),
            },
        },
    };

    let records = to_txt_records(&employee)?;

    // Verify correct number of records for deeply nested structure
    assert_eq!(
        records.len(),
        14,
        "Employee should generate exactly 14 records"
    );

    let records_map: std::collections::HashMap<String, String> = records.iter().cloned().collect();

    // Verify top-level fields
    assert_eq!(records_map.get("name"), Some(&"John Doe".to_string()));
    assert_eq!(records_map.get("id"), Some(&"12345".to_string()));

    // Verify nested contact information
    assert_eq!(
        records_map.get("contact.email"),
        Some(&"john.doe@example.com".to_string())
    );
    assert_eq!(
        records_map.get("contact.phone"),
        Some(&"+1-555-0123".to_string())
    );

    // Verify deeply nested contact address
    assert_eq!(
        records_map.get("contact.address.street"),
        Some(&"123 Main St".to_string())
    );
    assert_eq!(
        records_map.get("contact.address.city"),
        Some(&"New York".to_string())
    );
    assert_eq!(
        records_map.get("contact.address.country"),
        Some(&"USA".to_string())
    );

    // Verify company information
    assert_eq!(
        records_map.get("company.name"),
        Some(&"Tech Corp".to_string())
    );

    // Verify company headquarters address
    assert_eq!(
        records_map.get("company.headquarters.street"),
        Some(&"456 Corporate Blvd".to_string())
    );
    assert_eq!(
        records_map.get("company.headquarters.city"),
        Some(&"San Francisco".to_string())
    );

    // Verify company branch office address
    assert_eq!(
        records_map.get("company.branch_office.street"),
        Some(&"789 Branch Ave".to_string())
    );
    assert_eq!(
        records_map.get("company.branch_office.city"),
        Some(&"Austin".to_string())
    );

    // Deserialize back and verify complete roundtrip
    let deserialized_employee: Employee = from_txt_records(records)?;
    assert_eq!(
        employee, deserialized_employee,
        "Employee roundtrip must preserve all data"
    );

    // Example 2: Application configuration with nested services
    let app_config = ApplicationConfig {
        app_name: "MyWebApp".to_string(),
        version: "1.2.3".to_string(),
        services: ServiceConfig {
            database: DatabaseConfig {
                host: "db.example.com".to_string(),
                port: 5432,
            },
            cache: RedisConfig {
                host: "redis.example.com".to_string(),
                port: 6379,
                password: Some("secret123".to_string()),
            },
        },
    };

    let records = to_txt_records(&app_config)?;

    // Verify correct number of records for configuration structure
    assert_eq!(
        records.len(),
        7,
        "App config should generate exactly 7 records"
    );

    let records_map: std::collections::HashMap<String, String> = records.iter().cloned().collect();

    // Verify application-level fields
    assert_eq!(records_map.get("app_name"), Some(&"MyWebApp".to_string()));
    assert_eq!(records_map.get("version"), Some(&"1.2.3".to_string()));

    // Verify nested database configuration
    assert_eq!(
        records_map.get("services.database.host"),
        Some(&"db.example.com".to_string())
    );
    assert_eq!(
        records_map.get("services.database.port"),
        Some(&"5432".to_string())
    );

    // Verify nested cache configuration with optional field
    assert_eq!(
        records_map.get("services.cache.host"),
        Some(&"redis.example.com".to_string())
    );
    assert_eq!(
        records_map.get("services.cache.port"),
        Some(&"6379".to_string())
    );
    assert_eq!(
        records_map.get("services.cache.password"),
        Some(&"secret123".to_string())
    );

    // Deserialize back and verify complete roundtrip
    let deserialized_config: ApplicationConfig = from_txt_records(records)?;
    assert_eq!(
        app_config, deserialized_config,
        "App config roundtrip must preserve all data"
    );

    Ok(())
}
