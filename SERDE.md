# Serde Integration Guide

## Setup

First, add the required dependencies to your `Cargo.toml`:

```toml
[dependencies]
dataclass-macro = { version = "0.1.0", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"  # for JSON serialization/deserialization
```

## Basic Example

```rust
use dataclass_macro::dataclass;
use serde_json::json;

#[dataclass]
struct User {
    name: String,
    age: i32,
    email: Option<String>,
    roles: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new instance
    let user = User::new(
        String::from("Alice"),
        30,
        Some(String::from("alice@example.com")),
        vec![String::from("admin"), String::from("user")]
    );

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&user)?;
    println!("Serialized JSON:\n{}", json);

    // Deserialize from JSON
    let deserialized: User = serde_json::from_str(&json)?;
    assert_eq!(user, deserialized);

    Ok(())
}
```

Output:
```json
{
  "name": "Alice",
  "age": 30,
  "email": "alice@example.com",
  "roles": [
    "admin",
    "user"
  ]
}
```

## Custom Serialization

You can customize the serialization behavior using serde attributes:

```rust
use dataclass_macro::dataclass;
use serde::{Serialize, Deserialize};

#[dataclass]
struct Configuration {
    #[serde(rename = "serverName")]
    server_name: String,
    
    #[serde(default = "default_port")]
    port: u16,
    
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    
    #[serde(skip)]
    internal_data: String,
}

fn default_port() -> u16 {
    8080
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Configuration::new(
        String::from("production-server"),
        3000,
        vec![],
        String::from("internal")
    );

    let json = serde_json::to_string_pretty(&config)?;
    println!("Config JSON:\n{}", json);

    // The JSON will not include empty tags and internal_data
    // The server_name field will be serialized as "serverName"

    Ok(())
}
```

## Working with Complex Types

```rust
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use dataclass_macro::dataclass;
use serde::{Serialize, Deserialize};

#[dataclass]
struct AuditLog {
    #[serde(with = "chrono::serde::ts_seconds")]
    timestamp: DateTime<Utc>,
    
    user_id: String,
    
    #[serde(flatten)]
    metadata: HashMap<String, String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut metadata = HashMap::new();
    metadata.insert(String::from("ip"), String::from("192.168.1.1"));
    metadata.insert(String::from("browser"), String::from("Firefox"));

    let log = AuditLog::new(
        Utc::now(),
        String::from("user123"),
        metadata,
        Some(String::from("Login successful"))
    );

    let json = serde_json::to_string_pretty(&log)?;
    println!("Audit Log:\n{}", json);

    Ok(())
}
```

## Custom Enum Support

```rust
use dataclass_macro::dataclass;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Active,
    Inactive,
    Pending,
}

#[dataclass]
struct Account {
    id: String,
    status: Status,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    last_login: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account = Account::new(
        String::from("acc_123"),
        Status::Active,
        Some(String::from("2024-01-01"))
    );

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&account)?;
    println!("Account JSON:\n{}", json);

    // Deserialize from JSON string
    let json_str = r#"{
        "id": "acc_456",
        "status": "pending",
        "last_login": null
    }"#;
    
    let deserialized: Account = serde_json::from_str(json_str)?;
    println!("Deserialized account: {:?}", deserialized);

    Ok(())
}
```

## Integration with Different Formats

```rust
use dataclass_macro::dataclass;
use std::fs::File;

#[dataclass]
struct Settings {
    app_name: String,
    debug_mode: bool,
    max_connections: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::new(
        String::from("MyApp"),
        true,
        100
    );

    // JSON
    let json = serde_json::to_string_pretty(&settings)?;
    
    // YAML
    let yaml = serde_yaml::to_string(&settings)?;
    
    // TOML
    let toml = toml::to_string_pretty(&settings)?;
    
    // Write to files
    serde_json::to_writer_pretty(
        File::create("settings.json")?,
        &settings
    )?;
    
    serde_yaml::to_writer(
        File::create("settings.yaml")?,
        &settings
    )?;

    println!("Settings in different formats:");
    println!("\nJSON:\n{}", json);
    println!("\nYAML:\n{}", yaml);
    println!("\nTOML:\n{}", toml);

    Ok(())
}
```
