use apple_music_api::{config::MediaType, AppleMusicClient, AppleMusicError, ClientConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Basic usage with developer token
    println!("=== Apple Music API Client Examples ===\n");

    // You'll need to replace this with your actual developer token
    let team_id = "YOUR_TEAM_ID";
    let key_id = "YOUR_KEY_ID";
    let private_key_path = "YOUR_PRIVATE_KEY_PATH";

    // Create client configuration
    let config = ClientConfig::new(team_id, key_id, private_key_path)?;

    // Create the API client
    let client = AppleMusicClient::new(config).await?;

    println!("✅ Client created successfully!");

    // Example 2: Search for songs
    println!("\n--- Example 2: Search ---");
    match client.search("Hello", &[MediaType::Songs]).await {
        Ok(search_response) => {
            if let Some(songs) = &search_response.results.songs {
                println!("Found {} songs:", songs.data.len());
                for song in &songs.data[..std::cmp::min(3, songs.data.len())] {
                    println!(
                        "  - {} by {}",
                        song.attributes.name, song.attributes.artist_name
                    );
                }
            }
        }
        Err(e) => println!("Search failed: {}", e),
    }

    // Example 3: Get specific album
    println!("\n--- Example 3: Get Album ---");
    match client.get_album("310730204").await {
        // Example album ID
        Ok(album) => {
            println!("Album: {}", album.attributes.name);
            println!("Artist: {}", album.attributes.artist_name);
            println!("Track count: {}", album.attributes.track_count);
        }
        Err(e) => println!("Failed to get album: {}", e),
    }

    // Example 4: Get multiple songs
    println!("\n--- Example 4: Get Multiple Songs ---");
    let song_ids = ["203709340", "203709341"]; // Example song IDs
    match client.get_songs(&song_ids).await {
        Ok(songs) => {
            println!("Retrieved {} songs:", songs.len());
            for song in &songs {
                println!("  - {}", song.attributes.name);
            }
        }
        Err(e) => println!("Failed to get songs: {}", e),
    }

    // Example 5: Get storefront information
    println!("\n--- Example 5: Storefront Info ---");
    match client.get_storefront().await {
        Ok(storefront) => {
            if let Some(attrs) = &storefront.attributes {
                println!("Storefront: {}", attrs.name);
                println!("Language: {}", attrs.default_language_tag);
            }
        }
        Err(e) => println!("Failed to get storefront: {}", e),
    }

    // Example 7: Error handling
    println!("\n--- Example 7: Error Handling ---");
    match client.get_album("invalid-id").await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Expected error: {}", e);
            match e {
                AppleMusicError::Api { status, message } => {
                    println!("API Error {}: {}", status, message);
                }
                _ => println!("Other error type: {:?}", e),
            }
        }
    }

    println!("\n=== Examples completed ===");
    Ok(())
}
