#[cfg(test)]
mod tests {
    #[test]
    fn test_json_is_valid() {
        let json = r#"{"id":"i.YJMz65AFe6QOMk","type":"library-songs","href":"/v1/me/library/songs/i.YJMz65AFe6QOMk","attributes":{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","playParams":{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"},"releaseDate":"2024-09-27","trackNumber":1}}"#;

        let value: serde_json::Value = serde_json::from_str(json).expect("Invalid JSON");
        println!("JSON is valid!");
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    }
}
