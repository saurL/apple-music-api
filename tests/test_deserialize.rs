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

        let json_data = r#"{"results":{"songs":{"href":"/v1/catalog/us/search?limit=5&term=Phonky+Tribu+-+Radio+Edit+Funk+Tribu&types=songs","next":"/v1/catalog/us/search?offset=5&term=Phonky+Tribu+-+Radio+Edit+Funk+Tribu&types=songs","data":[{"id":"1671014864","type":"songs","href":"/v1/catalog/us/songs/1671014864","attributes":{"albumName":"The Midnight Club GT - Single","artistName":"Funk Tribu","artwork":{"bgColor":"e8e8eb","height":1440,"textColor1":"040467","textColor2":"12136f","textColor3":"323281","textColor4":"3d3d87","url":"https://is1-ssl.mzstatic.com/image/thumb/Music116/v4/00/aa/b6/00aab6c0-3e0a-2d32-e75b-ccb0a353a214/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Eduardo Jose Montañez Sanchez","discNumber":1,"durationInMillis":200661,"genreNames":["Dance","Music"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"DEQ022276059","name":"Phonky Tribu (Radio Edit)","playParams":{"id":"1671014864","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview116/v4/f1/a1/69/f1a1695c-2d14-41ae-0163-d263399cdfc6/mzaf_4199160351049765233.plus.a0ac.p.m4a"}],"releaseDate":"2021-10-21","trackNumber":1,"url":"https://music.apple.com/us/album/phonky-tribu-radio-edit/1671014855?i=1671014864"}},{"id":"1671019702","type":"songs","href":"/v1/catalog/us/songs/1671019702","attributes":{"albumName":"The Midnight Club GT - Single","artistName":"Funk Tribu","artwork":{"bgColor":"e8e8eb","height":1440,"textColor1":"040467","textColor2":"12136f","textColor3":"323281","textColor4":"3d3d87","url":"https://is1-ssl.mzstatic.com/image/thumb/Music126/v4/b6/2f/9b/b62f9b0d-cccc-488d-a447-86b39ce907f4/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Eduardo Jose Montañez Sanchez","discNumber":1,"durationInMillis":286145,"genreNames":["Dance","Music"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"QZMHN2270801","name":"Phonky Tribu","playParams":{"id":"1671019702","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview116/v4/f8/64/6e/f8646e4c-4689-7b70-c3cf-5e4a65bccdc5/mzaf_6751058878512816685.plus.aac.p.m4a"}],"releaseDate":"2021-10-21","trackNumber":1,"url":"https://music.apple.com/us/album/phonky-tribu/1671019558?i=1671019702"}},{"id":"1671015353","type":"songs","href":"/v1/catalog/us/songs/1671015353","attributes":{"albumName":"The Midnight Club GT - Single","artistName":"Funk Tribu","artwork":{"bgColor":"e8e8eb","height":1440,"textColor1":"040467","textColor2":"12136f","textColor3":"323281","textColor4":"3d3d87","url":"https://is1-ssl.mzstatic.com/image/thumb/Music116/v4/00/aa/b6/00aab6c0-3e0a-2d32-e75b-ccb0a353a214/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Eduardo Jose Montañez Sanchez","discNumber":1,"durationInMillis":286145,"genreNames":["Dance","Music"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"QZMHN2270801","name":"Phonky Tribu","playParams":{"id":"1671015353","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview116/v4/27/9e/03/279e0359-f7b8-02f4-ed6f-757888b24532/mzaf_8957810221213680443.plus.aac.p.m4a"}],"releaseDate":"2021-10-21","trackNumber":2,"url":"https://music.apple.com/us/album/phonky-tribu/1671014855?i=1671015353"}},{"id":"1670921017","type":"songs","href":"/v1/catalog/us/songs/1670921017","attributes":{"albumName":"Phonky Tribu (DJ HEARTSTRING Remix) - Single","artistName":"Funk Tribu","artwork":{"bgColor":"e7e9e5","height":1440,"textColor1":"350467","textColor2":"411d66","textColor3":"593280","textColor4":"62467f","url":"https://is1-ssl.mzstatic.com/image/thumb/Music126/v4/c3/d1/b0/c3d1b0b2-f12c-b8fa-4aeb-060f130ae8f4/cover.jpg/{w}x{h}bb.jpg","width":1440},"composerName":"Eduardo Jose Montañez Sanchez, Leonard Brede & Jonas Hellberg","discNumber":1,"durationInMillis":250348,"genreNames":["Dance","Music"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"DEQ022292200","name":"Phonky Tribu (DJ HEARTSTRING Remix)","playParams":{"id":"1670921017","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview126/v4/d4/30/d1/d430d10b-4ea0-3c8f-0fa8-72a52b0b1789/mzaf_17345217758110795880.plus.aac.p.m4a"}],"releaseDate":"2023-01-13","trackNumber":1,"url":"https://music.apple.com/us/album/phonky-tribu-dj-heartstring-remix/1670920754?i=1670921017"}},{"id":"1722313880","type":"songs","href":"/v1/catalog/us/songs/1722313880","attributes":{"albumName":"Phonky Tribu (FØSS Hard Remix) - Single","artistName":"Funk Tribu","artwork":{"bgColor":"2a15eb","height":3000,"textColor1":"e9e7fc","textColor2":"ec64fd","textColor3":"c3bdf9","textColor4":"c554f9","url":"https://is1-ssl.mzstatic.com/image/thumb/Music116/v4/21/f6/24/21f624c8-c1c1-7342-594b-ee46e9d4face/cover.jpg/{w}x{h}bb.jpg","width":3000},"composerName":"Eduardo Jose Montañez Sanchez & Maarten Fossey","discNumber":1,"durationInMillis":388600,"genreNames":["Dance","Music"],"hasLyrics":false,"isAppleDigitalMaster":false,"isrc":"DGA0R2304040","name":"Phonky Tribu (FØSS Hard Remix)","playParams":{"id":"1722313880","kind":"song"},"previews":[{"url":"https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview116/v4/3b/d1/57/3bd1579d-43cf-d532-68f5-a6c075d87ff4/mzaf_15220596566839265834.plus.aac.p.m4a"}],"releaseDate":"2024-01-12","trackNumber":1,"url":"https://music.apple.com/us/album/phonky-tribu-f%C3%B8ss-hard-remix/1722313555?i=1722313880"}}]}},"meta":{"results":{"order":["songs"],"rawOrder":["songs"]}}}"#;

        // Tentative de désérialisation
        let parsed: SearchResponse =
            serde_json::from_str(json_data).expect("Failed to deserialize previews");

        // Vérifie qu'on a bien 1 preview et que l'URL correspond
        assert_eq!(
            parsed.results.songs.unwrap().data[0].attributes.previews[0].url,
            "https://audio-ssl.itunes.apple.com/itunes-assets/AudioPreview116/v4/f1/a1/69/f1a1695c-2d14-41ae-0163-d263399cdfc6/mzaf_4199160351049765233.plus.a0ac.p.m4a"
        );
    }
}
