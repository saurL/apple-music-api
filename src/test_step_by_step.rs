#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_parse_play_params() {
        // Testons juste les playParams
        let json = r#"{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"}"#;

        let result: Result<crate::models::common::PlayParameters, _> = serde_json::from_str(json);
        match result {
            Ok(params) => {
                println!("✅ PlayParameters OK");
                println!("  - id: {}", params.id);
                println!("  - kind: {}", params.kind);
                println!("  - reporting: {:?}", params.reporting);
                println!("  - reportingId: {:?}", params.reporting_id);
            }
            Err(e) => {
                panic!("❌ Erreur PlayParameters: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_library_song() {
        let json = r#"{"id":"i.YJMz65AFe6QOMk","type":"library-songs","href":"/v1/me/library/songs/i.YJMz65AFe6QOMk","attributes":{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","playParams":{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"},"releaseDate":"2024-09-27","trackNumber":1}}"#;

        let result: Result<crate::models::library::LibrarySong, _> = serde_json::from_str(json);
        match result {
            Ok(song) => {
                println!("✅ LibrarySong OK");
                println!("  - id: {}", song.id);
            }
            Err(e) => {
                panic!("❌ Erreur LibrarySong: {}", e);
            }
        }
    }
}
