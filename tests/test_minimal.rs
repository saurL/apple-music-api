#[cfg(test)]
mod tests {
    use apple_music_api::models::library::LibrarySongAttributes;

    #[test]
    fn test_minimal_attrs() {
        let json = r#"{"name":"TEST"}"#;
        let result: Result<LibrarySongAttributes, _> = serde_json::from_str(json);
        match result {
            Ok(attrs) => println!("✅ Minimal OK: {:?}", attrs.name),
            Err(e) => panic!("❌ Minimal: {}", e),
        }
    }

    #[test]
    fn test_with_bools_and_vec() {
        let json = r#"{"name":"TEST","genreNames":["Rock"],"hasLyrics":true}"#;
        let result: Result<LibrarySongAttributes, _> = serde_json::from_str(json);
        match result {
            Ok(attrs) => println!("✅ With bools OK: {:?}", attrs.name),
            Err(e) => panic!("❌ With bools: {}", e),
        }
    }

    #[test]
    fn test_with_date() {
        let json = r#"{"name":"TEST","releaseDate":"2024-09-27"}"#;
        let result: Result<LibrarySongAttributes, _> = serde_json::from_str(json);
        match result {
            Ok(attrs) => println!("✅ With date OK: {:?}", attrs.name),
            Err(e) => panic!("❌ With date: {}", e),
        }
    }
}
