#[cfg(test)]
mod tests {
    use serde_json;

    #[test]
    fn test_parse_artwork() {
        let json = r#"{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200}"#;

        let result: Result<apple_music_api::models::common::Artwork, _> = serde_json::from_str(json);
        match result {
            Ok(artwork) => {
                println!("✅ Artwork OK");
                println!("  - url: {}", artwork.url);
                println!("  - height: {:?}", artwork.height);
                println!("  - width: {:?}", artwork.width);
            }
            Err(e) => {
                panic!("❌ Erreur Artwork: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_play_params() {
        // Testons juste les playParams
        let json = r#"{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"}"#;

        let result: Result<apple_music_api::models::common::PlayParameters, _> = serde_json::from_str(json);
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
    fn test_parse_library_song_attributes_without_playparams() {
        let json = r#"{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","releaseDate":"2024-09-27","trackNumber":1}"#;

        let result: Result<apple_music_api::models::library::LibrarySongAttributes, _> = serde_json::from_str(json);
        match result {
            Ok(attrs) => {
                println!("✅ LibrarySongAttributes (sans playParams) OK");
                println!("  - name: {:?}", attrs.name);
                println!("  - artistName: {:?}", attrs.artist_name);
                println!("  - albumName: {:?}", attrs.album_name);
                println!("  - hasLyrics: {}", attrs.has_lyrics);
                println!("  - genreNames: {:?}", attrs.genre_names);
            }
            Err(e) => {
                panic!("❌ Erreur LibrarySongAttributes (sans playParams): {}", e);
            }
        }
    }

    #[test]
    fn test_parse_library_song_attributes() {
        let json = r#"{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","playParams":{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"},"releaseDate":"2024-09-27","trackNumber":1}"#;

        let result: Result<apple_music_api::models::library::LibrarySongAttributes, _> = serde_json::from_str(json);
        match result {
            Ok(attrs) => {
                println!("✅ LibrarySongAttributes OK");
                println!("  - name: {:?}", attrs.name);
                println!("  - artistName: {:?}", attrs.artist_name);
                println!("  - albumName: {:?}", attrs.album_name);
                println!("  - hasLyrics: {}", attrs.has_lyrics);
                println!("  - genreNames: {:?}", attrs.genre_names);
            }
            Err(e) => {
                panic!("❌ Erreur LibrarySongAttributes: {}", e);
            }
        }
    }

    #[test]
    fn test_parse_library_song() {
        let json = r#"{"id":"i.YJMz65AFe6QOMk","type":"library-songs","href":"/v1/me/library/songs/i.YJMz65AFe6QOMk","attributes":{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://test.com/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","playParams":{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"},"releaseDate":"2024-09-27","trackNumber":1}}"#;

        let result: Result<apple_music_api::models::library::LibrarySong, _> = serde_json::from_str(json);
        match result {
            Ok(song) => {
                println!("✅ LibrarySong OK");
                println!("  - id: {}", song.id);
                println!("  - type: {}", song.resource_type);
            }
            Err(e) => {
                panic!("❌ Erreur LibrarySong: {}", e);
            }
        }
    }
}
