use dataclass_macro::dataclass;

#[test]
fn test_basic_dataclass() {
    #[dataclass(
        init = true,
        repr = true,
        eq = true,
        order = true,
        unsafe_hash = true,
        frozen = false,
        match_args = true,
        kw_only = false,
        slots = true,
        weakref_slot = false
    )]
    struct Person {
        name: String,
        age: i32,
        email: Option<String>,
    }

    let person1 = Person::new(
        String::from("Alice"),
        30,
        Some(String::from("alice@example.com"))
    );
    
    let person2 = Person::new(
        String::from("Alice"),
        30,
        Some(String::from("alice@example.com"))
    );
    
    // Debug (repr)
    println!("{:?}", person1);
    
    // (eq)
    assert_eq!(person1, person2);
    
    // (order)
    let person3 = Person::new(
        String::from("Bob"),
        25,
        Some(String::from("bob@example.com"))
    );
    assert!(person1 < person3);
    
    // (unsafe_hash)
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(person1);
    assert!(set.contains(&person2));
}

#[test]
fn test_frozen_dataclass() {
    #[dataclass(frozen = true)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point::new(10, 20);
    // point.x = 30; // This should fail to compile
    assert_eq!(point.x, 10);
}

#[test]
fn test_no_order_dataclass() {
    #[dataclass(order = false)]
    struct Config {
        name: String,
        value: i32,
    }

    let config1 = Config::new(String::from("test"), 42);
    let config2 = Config::new(String::from("test"), 42);
    
    assert_eq!(config1, config2);
    
    // not allowed to compare Config
    // assert!(config1 < config2);
}

#[test]
fn test_default_options() {
    #[dataclass]
    struct Simple {
        value: i32,
    }

    let simple = Simple::new(42);
    println!("{:?}", simple); // Should work due to default repr = true
    assert_eq!(simple, Simple::new(42)); // Should work due to default eq = true
}