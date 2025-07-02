#[cfg(test)]
mod tests {
    use rovkit::jsonkit::*;
    #[test]
    fn test_parse_and_stringify() {
        let json = r#"{"name": "Alice", "age": 30}"#;
        let v = parse(json).unwrap();
        assert_eq!(get(&v, "name").unwrap(), "Alice");
        let s = to_string(&v).unwrap();
        assert!(s.contains("Alice"));
    }

    #[test]
    fn test_set_and_pretty_string() {
        let json = r#"{"name": "Alice"}"#;
        let v = parse(json).unwrap();
        let v2 = set(v, "age", Value::from(25));
        assert_eq!(get(&v2, "age").unwrap(), 25);
        let pretty = to_pretty_string(&v2).unwrap();
        assert!(pretty.contains("\n"));
    }

    #[test]
    fn test_type_checking() {
        let obj = parse("{}").unwrap();
        let arr = parse("[]").unwrap();
        let s = Value::from("hello");
        assert!(is_object(&obj));
        assert!(is_array(&arr));
        assert!(is_string(&s));
    }

    use rovkit::jsonkit::{from_json, is_object, parse, to_pretty_string};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct User {
        name: String,
        age: u32,
    }

    #[test]
    fn test_struct_serialization() {
        let user = User {
            name: "Bob".to_string(),
            age: 28,
        };
        let json = to_json(&user).unwrap();
        let json2 = to_pretty_json(&user).unwrap();

        println!("user.json = {}", json);
        println!("user.json2 = {}", json2);
        assert!(json.contains("Bob"));

        let user2: User = from_json(&json).unwrap();

        println!("user2: = {:?}", user2);
        assert_eq!(user, user2);
    }
}
