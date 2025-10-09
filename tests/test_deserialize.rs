#[cfg(test)]
mod tests {
    use apple_music_api::catalog::Preview;
    use apple_music_api::models::{common::ApiResponse, library::LibraryPlaylist};
    use apple_music_api::search::SearchResponse;
    use serde::{Deserialize, Serialize};
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
                        println!(
                            "✅ Total meta : {}",
                            tracks.meta.as_ref().unwrap().total.unwrap()
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!(
                    "Structure JSON attendue:n{}",
                    serde_json::to_string_pretty(&value).unwrap()
                );
                panic!("❌ Erreur de désérialisation : {}", e);
            }
        }
    }

    #[test]
    fn test_deserialize_previews() {
        // JSON simulant un extrait de réponse Apple Music
        #[derive(Debug, Serialize, Deserialize)]
        struct TestStruct {
            #[serde(rename = "previews")]
            pub previews: Vec<Preview>,
        }
        let json_data = r#"
        {
            "previews": [
                {
                    "url": "https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview221/v4/09/c4/7a/09c47af0-d3ac-79fe-41eb-e7bb651dd738/mzaf_15998617947319918760.plus.aac.p.m4a"
                }
            ]
        }
        "#;

        // Tentative de désérialisation
        let parsed: TestStruct =
            serde_json::from_str(json_data).expect("Failed to deserialize previews");

        // Vérifie qu'on a bien 1 preview et que l'URL correspond
        assert_eq!(
            parsed.previews[0].url,
            "https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview221/v4/09/c4/7a/09c47af0-d3ac-79fe-41eb-e7bb651dd738/mzaf_15998617947319918760.plus.aac.p.m4a"
        );
    }

    #[test]
    fn test_deserialize_search_response() {
        // JSON simulant un extrait de réponse Apple Music

        let json_data = r#"{"results":{"songs":{"href":"/v1/catalog/us/search?limit=5&term=Hero+Dance+Grand+V&types=songs","next":"/v1/catalog/us/search?offset=5&term=Hero+Dance+Grand+V&types=songs","data":[{"id":"1541381908","type":"songs","href":"/v1/catalog/us/songs/1541381908","attributes":{"albumName":"Hero Dance  - EP","artistName":"Grand V","artwork":{"bgColor":"040404","height":1440,"textColor1":"c7c7c7","textColor2":"787ff0","textColor3":"a0a0a0","textColor4":"6066c0","url":"https://is1-ssl.mzstatic.com/image/thumb/Music114/v4/b1/de/54/b1de544d-c677-fec4-2284-f7d464095290/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Victor Tescaro","discNumber":1,"durationInMillis":434000,"genreNames":["Trance","Music","Dance"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"FR26V2088562","name":"Hero Dance","playParams":{"id":"1541381908","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview115/v4/a0/0c/ce/a00cce73-59ac-039b-838a-5fe78f306a51/mzaf_11197761378492441764.plus.aac.p.m4a"}],"releaseDate":"2020-11-30","trackNumber":2,"url":"https://music.apple.com/us/album/hero-dance/1541381887?i=1541381908"}},{"id":"1541381915","type":"songs","href":"/v1/catalog/us/songs/1541381915","attributes":{"albumName":"Hero Dance  - EP","artistName":"Grand V","artwork":{"bgColor":"040404","height":1440,"textColor1":"c7c7c7","textColor2":"787ff0","textColor3":"a0a0a0","textColor4":"6066c0","url":"https://is1-ssl.mzstatic.com/image/thumb/Music114/v4/b1/de/54/b1de544d-c677-fec4-2284-f7d464095290/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Victor Tescaro","discNumber":1,"durationInMillis":427125,"genreNames":["Trance","Music","Dance"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"FR26V2088563","name":"Sonar","playParams":{"id":"1541381915","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview124/v4/76/36/80/76368080-565a-d304-14a7-7ff326d8dcd2/mzaf_18160807612029062648.plus.aac.p.m4a"}],"releaseDate":"2020-11-30","trackNumber":3,"url":"https://music.apple.com/us/album/sonar/1541381887?i=1541381915"}},{"id":"1541381906","type":"songs","href":"/v1/catalog/us/songs/1541381906","attributes":{"albumName":"Hero Dance  - EP","artistName":"Grand V","artwork":{"bgColor":"040404","height":1440,"textColor1":"c7c7c7","textColor2":"787ff0","textColor3":"a0a0a0","textColor4":"6066c0","url":"https://is1-ssl.mzstatic.com/image/thumb/Music114/v4/b1/de/54/b1de544d-c677-fec4-2284-f7d464095290/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Victor Tescaro","discNumber":1,"durationInMillis":423000,"genreNames":["Trance","Music","Dance"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"FR26V2088561","name":"F1","playParams":{"id":"1541381906","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview124/v4/c8/4d/79/c84d7918-7181-f66d-6586-a72870feac6b/mzaf_4734802626057784168.plus.aac.p.m4a"}],"releaseDate":"2020-11-30","trackNumber":1,"url":"https://music.apple.com/us/album/f1/1541381887?i=1541381906"}},{"id":"1541382188","type":"songs","href":"/v1/catalog/us/songs/1541382188","attributes":{"albumName":"Hero Dance  - EP","artistName":"Grand V","artwork":{"bgColor":"040404","height":1440,"textColor1":"c7c7c7","textColor2":"787ff0","textColor3":"a0a0a0","textColor4":"6066c0","url":"https://is1-ssl.mzstatic.com/image/thumb/Music114/v4/b1/de/54/b1de544d-c677-fec4-2284-f7d464095290/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Victor Tescaro","discNumber":1,"durationInMillis":415890,"genreNames":["Trance","Music","Dance"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"FR26V2088564","name":"Razor","playParams":{"id":"1541382188","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview114/v4/b3/56/8e/b3568ee2-6e3f-3229-8142-5c5b230a8db3/mzaf_18088376758284496985.plus.aac.p.m4a"}],"releaseDate":"2020-11-30","trackNumber":4,"url":"https://music.apple.com/us/album/razor/1541381887?i=1541382188"}},{"id":"1347316153","type":"songs","href":"/v1/catalog/us/songs/1347316153","attributes":{"albumName":"Hero (Satellite) [Radio Edit] - Single","artistName":"Breathe Carolina & Y&V","artwork":{"bgColor":"ffffff","height":1500,"textColor1":"001444","textColor2":"0f345a","textColor3":"33436a","textColor4":"3f5d7b","url":"https://is1-ssl.mzstatic.com/image/thumb/Music62/v4/12/ae/bd/12aebdd5-4ffd-1967-9009-416d1225a30d/8712944486179.jpg/{w}x{h}bb.jpg","width":1500},"composerName":"D. Schmitt, J. Mitchell, K. Madden, T. Cooperman, V. Verpillat & Y. Bargain","discNumber":1,"durationInMillis":183906,"genreNames":["Dance","Music"],"hasLyrics":true,"isAppleDigitalMaster":false,"isrc":"NLZ541500611","name":"Hero (Satellite) [Radio Edit]","playParams":{"id":"1347316153","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview125/v4/5c/ca/10/5cca1048-2c28-728b-5511-f0f663010767/mzaf_11174736781504204001.plus.aac.p.m4a"}],"releaseDate":"2015-09-14","trackNumber":1,"url":"https://music.apple.com/us/album/hero-satellite-radio-edit/1347316018?i=1347316153"}}]}},"meta":{"results":{"order":["songs"],"rawOrder":["songs"]}}}"#;

        // Tentative de désérialisation
        let parsed: SearchResponse =
            serde_json::from_str(json_data).expect("Failed to deserialize previews");
        assert!(parsed.results.songs.is_some());
    }
}
