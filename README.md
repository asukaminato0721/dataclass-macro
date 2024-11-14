# Rust Dataclass Macro

[![Rust CI](https://github.com/asukaminato0721/dataclass-macro/actions/workflows/ci.yml/badge.svg)](https://github.com/asukaminato0721/dataclass-macro/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/asukaminato0721/dataclass-macro/branch/main/graph/badge.svg)](https://codecov.io/gh/asukaminato0721/dataclass-macro)
[![crates.io](https://img.shields.io/crates/v/dataclass-macro.svg)](https://crates.io/crates/dataclass-macro)
[![Documentation](https://docs.rs/dataclass-macro/badge.svg)](https://docs.rs/dataclass-macro)

A Rust procedural macro that implements Python-style dataclasses. This macro helps reduce boilerplate code by automatically implementing common traits and generating constructors for your structs.

## Features

- Similar API to Python's `@dataclass` decorator
- Customizable trait implementations
- Supports frozen (immutable) classes
- Memory layout optimization options
- Automatic constructor generation
- Optional serde support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
dataclass-macro = "0.1.0"  # Replace with actual version
```

## Usage

Basic usage:

```rust
use dataclass_macro::dataclass;

#[dataclass]  // Use all default options
struct Point {
    x: i32,
    y: i32,
}

// With custom options
#[dataclass(
    init = true,
    repr = true,
    eq = true,
    order = true,
    unsafe_hash = true,
    frozen = false,
    slots = false
)]
struct Person {
    name: String,
    age: i32,
    email: Option<String>,
}

fn main() {
    let person = Person::new(
        String::from("Alice"),
        30,
        Some(String::from("alice@example.com"))
    );
    
    println!("{:?}", person);  // Debug output thanks to repr=true
    
    let clone = person.clone();  // Clone is always implemented
    assert_eq!(person, clone);   // PartialEq is implemented when eq=true
}
```

## Options

| Option | Default | Description |
|--------|---------|-------------|
| `init` | `true` | Generate a constructor |
| `repr` | `true` | Implement Debug trait |
| `eq` | `true` | Implement PartialEq and Eq traits |
| `order` | `false` | Implement Ord and PartialOrd traits |
| `unsafe_hash` | `false` | Implement Hash trait |
| `frozen` | `false` | Make fields immutable (pub(crate)) |
| `match_args` | `true` | Enable pattern matching support |
| `kw_only` | `false` | Constructor requires named arguments |
| `slots` | `false` | Optimize memory layout |
| `weakref_slot` | `false` | Reserved for future use |

## Generated Code

For a basic struct with default options, the macro generates:

```rust
// Your code
#[dataclass]
struct Point {
    x: i32,
    y: i32,
}

// Generated code
#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}
```

## Feature Flags

- `serde`: Enable serde support for serialization/deserialization

```toml
[dependencies]
dataclass-macro = { version = "0.1.0", features = ["serde"] }
```

## Examples

### Basic Point Structure

```rust
use dataclass_macro::dataclass;

#[dataclass]
struct Point {
    x: i32,
    y: i32,
}

let point = Point::new(10, 20);
println!("{:?}", point);  // Point { x: 10, y: 20 }
```

### Ordered Data Structure

```rust
#[dataclass(order = true)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

let v1 = Version::new(1, 0, 0);
let v2 = Version::new(2, 0, 0);
assert!(v1 < v2);  // Comparison works
```

### Immutable Structure

```rust
#[dataclass(frozen = true)]
struct Config {
    name: String,
    value: i32,
}

let config = Config::new(String::from("test"), 42);
// config.value = 43;  // This would cause a compilation error
```

### With Serde Support

```rust
use dataclass_macro::dataclass;
use serde::{Serialize, Deserialize};

#[dataclass]
struct User {
    #[serde(rename = "userName")]
    name: String,
    age: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::new(
        String::from("Alice"),
        30,
        Some(String::from("alice@example.com"))
    );

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&user)?;
    println!("JSON:\n{}", json);
    
    // Deserialize from JSON
    let deserialized: User = serde_json::from_str(&json)?;
    assert_eq!(user, deserialized);
    
    Ok(())
}
```

For more detailed serde integration examples, including custom serialization, working with complex types, and different formats, see [SERDE.md](SERDE.md).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Comparison with Python's Dataclass

This macro aims to provide similar functionality to Python's dataclass decorator while remaining true to Rust's patterns and safety guarantees. The main differences are:

- No default values in struct definition (use Default trait instead)
- No post-init processing (use custom impl blocks)
- No field order specification (follows struct definition order)
- Additional memory optimization options
- Rust-specific features like pub/pub(crate) visibility

## Known Limitations

- Limited support for generic types (work in progress)
- No support for custom derive implementations
- Field attributes are not processed
