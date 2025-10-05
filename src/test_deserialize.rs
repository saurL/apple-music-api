#[cfg(test)]
mod tests {
    use crate::models::{common::ApiResponse, library::LibraryPlaylist};

    #[test]
    fn test_deserialize_playlist_with_tracks() {
        let json = r#"{"data":[{"id":"p.O1kz87VF84ebBl","type":"library-playlists","href":"/v1/me/library/playlists/p.O1kz87VF84ebBl","attributes":{"artwork":{"height":null,"url":"https://is1-ssl.mzstatic.com/image/thumb/gen/600x600AM.PDCXS01.jpg","width":null},"canEdit":true,"dateAdded":"2025-10-02T21:31:15Z","description":{"standard":""},"hasCatalog":false,"isPublic":false,"lastModifiedDate":"2025-10-04T10:39:12Z","name":"test","playParams":{"id":"p.O1kz87VF84ebBl","isLibrary":true,"kind":"playlist"}},"relationships":{"tracks":{"href":"/v1/me/library/playlists/p.O1kz87VF84ebBl/tracks","data":[{"id":"i.YJMz65AFe6QOMk","type":"library-songs","href":"/v1/me/library/songs/i.YJMz65AFe6QOMk","attributes":{"albumName":"A LA VIE A LA MORT","artistName":"SDM","artwork":{"height":1200,"url":"https://is1-ssl.mzstatic.com/image/thumb/Music211/v4/64/46/95/64469515-0230-b26c-69cf-5db61f5ea879/24UMGIM72197.rgb.jpg/{w}x{h}bb.jpg","width":1200},"contentRating":"explicit","discNumber":1,"durationInMillis":135693,"genreNames":["Hip-Hop/Rap"],"hasLyrics":true,"name":"DRAGO MALEFOY","playParams":{"catalogId":"1757803058","id":"i.YJMz65AFe6QOMk","isLibrary":true,"kind":"song","reporting":true,"reportingId":"1757803058"},"releaseDate":"2024-09-27","trackNumber":1}}],"meta":{"total":14}}}}]}"#;

        // D'abord, vérifier que le JSON est valide
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON invalide");
        println!("✅ JSON est valide");

        // Maintenant essayer de désérialiser dans notre structure
        let result: Result<ApiResponse<LibraryPlaylist>, _> = serde_json::from_str(json);

        match result {
            Ok(response) => {
                println!("✅ Désérialisation réussie !");
                assert_eq!(response.data.len(), 1);
                let playlist = &response.data[0];
                assert_eq!(playlist.id, "p.O1kz87VF84ebBl");
                assert_eq!(playlist.attributes.name, "test");

                // Vérifier les tracks
                if let Some(ref relationships) = playlist.relationships {
                    if let Some(ref tracks) = relationships.tracks {
                        assert_eq!(tracks.data.len(), 1);
                        assert_eq!(tracks.meta.as_ref().unwrap().total.unwrap(), 14);
                        println!("✅ Nombre de tracks : {}", tracks.data.len());
                        println!("✅ Total meta : {}", tracks.meta.as_ref().unwrap().total.unwrap());
                    }
                }
            }
            Err(e) => {
                eprintln!("Structure JSON attendue:\n{}", serde_json::to_string_pretty(&value).unwrap());
                panic!("❌ Erreur de désérialisation : {}", e);
            }
        }
    }
}
