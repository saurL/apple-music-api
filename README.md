# Apple Music API Client

A comprehensive Rust client for the Apple Music API with rustls for secure HTTP requests. This library provides a type-safe, async interface to interact with Apple's Music API, supporting both catalog and library operations.

## Features

- 🔒 **Secure by default**: Uses rustls for TLS connections
- 🎵 **Full API coverage**: Catalog search, library management, personalized content
- 🏗️ **Type-safe**: Strongly typed responses and requests
- 🚀 **Async/await**: Built on tokio for high performance
- 🔧 **Flexible authentication**: Support for both JWT and simple token authentication
- 📚 **Comprehensive**: Search, playlists, albums, artists, songs, and more
- 🛡️ **Error handling**: Detailed error types with retry logic

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
apple-music-api = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Quick Start

### Basic Usage

```rust
use apple_music_api::{AppleMusicClient, ClientConfig, MediaType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client configuration with your developer token
    let config = ClientConfig::new("your-developer-token-here")?;

    // Create the API client
    let client = AppleMusicClient::new(config).await?;

    // Search for songs
    let search_results = client.search("Bohemian Rhapsody", &[MediaType::Songs]).await?;

    if let Some(songs) = &search_results.results.songs {
        for song in &songs.data {
            println!("Found: {} by {}", song.attributes.name, song.attributes.artist_name);
        }
    }

    Ok(())
}
```

### JWT Authentication (Recommended for Production)

```rust
use apple_music_api::{AppleMusicClient, ClientConfig};

// For production use, set up JWT authentication
let config = ClientConfig::new("your-developer-token")?;
let client = AppleMusicClient::with_jwt_auth(
    config,
    "your-team-id".to_string(),
    "your-key-id".to_string(),
    "your-private-key-pem-content".to_string(),
).await?;
```

### Library Access (Requires User Token)

```rust
// Set user token for personalized requests
client.set_user_token(Some("user-music-token".to_string())).await?;

// Access user's library
let library_songs = client.get_library_songs().await?;
println!("User has {} songs in their library", library_songs.data.len());
```

## API Coverage

### Catalog Operations

- **Search**: Full-text search across songs, albums, artists, playlists
- **Resources**: Get detailed information about songs, albums, artists, playlists
- **Multiple resources**: Batch requests for multiple items
- **Hints & Suggestions**: Search autocomplete and suggestions

### Library Operations (Requires User Token)

- **Library content**: Access user's saved songs, albums, artists, playlists
- **Add to library**: Add songs, albums, or playlists to user's library
- **Personalized content**: User-specific recommendations and history

### Utility Operations

- **Storefronts**: Get available storefronts and current storefront info
- **Search helpers**: Hints, suggestions, and advanced search options

## Configuration

### Client Configuration

```rust
use apple_music_api::ClientConfig;
use std::time::Duration;

let config = ClientConfig::new("your-developer-token")?
    .with_base_url("https://api.music.apple.com")?
    .with_timeout(Duration::from_secs(30))?
    .with_max_retries(3)?
    .with_storefront("us")?;
```

### Search Options

```rust
use apple_music_api::{MediaType, SearchOptions};

let options = SearchOptions::new()
    .with_limit(10)?
    .with_offset(0)?;

let results = client.search_with_options("query", &[MediaType::Songs], &options).await?;
```

## Error Handling

The library provides comprehensive error handling:

```rust
use apple_music_api::AppleMusicError;

match client.get_album("invalid-id").await {
    Ok(album) => println!("Album: {}", album.attributes.name),
    Err(AppleMusicError::Api { status, message }) => {
        println!("API Error {}: {}", status, message);
    }
    Err(AppleMusicError::Auth(msg)) => {
        println!("Authentication error: {}", msg);
    }
    Err(e) => println!("Other error: {}", e),
}
```

## Authentication

### Developer Token

Required for all API requests. Can be obtained from:

1. **Simple token**: Pre-generated token from Apple Developer Console
2. **JWT token**: Generated from your private key (recommended for production)

### User Token (Optional)

Required for library operations and personalized content:

- Obtained through MusicKit on iOS/macOS
- Allows access to user's library and personalized recommendations

## Examples

See `src/main.rs` for comprehensive examples including:

- Basic search operations
- Resource retrieval (albums, artists, songs)
- Error handling
- Library access (when user token is available)
- Storefront operations

## Building and Testing

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run the example
cargo run
```

## Requirements

- Rust 1.70 or later
- Valid Apple Music API developer token
- For library access: User token from MusicKit

## Security

- All HTTP requests use rustls for secure TLS connections
- JWT tokens are properly signed and validated
- No sensitive data is logged or stored

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This is an unofficial Apple Music API client. Please ensure compliance with Apple's API terms of service and developer guidelines.
