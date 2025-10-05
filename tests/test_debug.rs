#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn debug_library_song_attrs_structure() {
        let json = r#"{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","releaseDate":"2024-09-27","trackNumber":1}"#;

        let value: serde_json::Value = serde_json::from_str(json).expect("JSON invalide");
        println!("Structure complète:");
        println!("{}", serde_json::to_string_pretty(&value).unwrap());

        println!("\n\nChamps présents:");
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                println!("  - {}: {:?}", key, val);
            }
        }
    }
}
