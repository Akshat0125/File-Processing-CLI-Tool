use crate::models::User;
use crate::errors::{Result, AppError};
use std::path::Path;

pub fn parse(content: &str, extension: &str) -> Result<Vec<User>> {
    match extension {
        "json" => parse_json(content),
        "csv" => parse_csv(content),
        _ => Err(AppError::ParseError(format!("Unsupported file extension: {}", extension))),
    }
}

fn parse_json(content: &str) -> Result<Vec<User>> {
    serde_json::from_str(content).map_err(AppError::SerializationError)
}

fn parse_csv(content: &str) -> Result<Vec<User>> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let mut users = Vec::new();
    for result in reader.deserialize() {
        let user: User = result?;
        users.push(user);
    }
    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json() {
        let json_data = r#"[
            {"id": 1, "name": "Test User", "email": "test@example.com", "active": true}
        ]"#;
        let users = parse(json_data, "json").unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Test User");
    }

    #[test]
    fn test_parse_csv() {
        let csv_data = "id,name,email,active\n1,Test User,test@example.com,true";
        let users = parse(csv_data, "csv").unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].name, "Test User");
    }

    #[test]
    fn test_unsupported_format() {
        let result = parse("some content", "xml");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_json() {
        let result = parse("invalid json", "json");
        assert!(result.is_err());
    }
}
