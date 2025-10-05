//! # Apple Music API Client
//!
//! A Rust client for interacting with the Apple Music API using rustls for secure HTTP requests.
//!
//! ## Features
//!
//! - Secure HTTP client with rustls
//! - JWT-based developer token authentication
//! - User token support for personalized requests (via MusicKit)
//! - Strongly typed API responses
//! - Comprehensive error handling
//!
//! ## Authentication
//!
//! ### Developer Token
//!
//! To use the Apple Music API, you need to generate a developer token using your:
//! - Team ID (from Apple Developer account)
//! - Key ID (from App Store Connect)
//! - Private Key (.p8 file from App Store Connect)
//!
//! See: <https://developer.apple.com/documentation/applemusicapi/generating_developer_tokens>
//!
//! ### User Token
//!
//! For personalized requests (accessing user's library, creating playlists, etc.), you need a user token.
//! The user token must be obtained from MusicKit JS on the client side and passed to your backend.
//!
//! See: <https://developer.apple.com/documentation/musickit/musickit_js/getting_keys_and_creating_tokens>
//!
//! ## Example
//!
//! ```rust,no_run
//! use apple_music_api::{AppleMusicClient, ClientConfig, create_developer_token};
//! use derive_builder::Builder;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load your credentials from environment or config
//!     let team_id = std::env::var("APPLE_TEAM_ID")?;
//!     let key_id = std::env::var("APPLE_KEY_ID")?;
//!     let private_key = include_str!("../certs/AuthKey.p8");
//!
//!     // Generate developer token
//!     let developer_token = create_developer_token(&team_id, &key_id, private_key)?;
//!
//!     // Create client configuration
//!     let config = ClientConfig::builder()
//!         .developer_token(developer_token)
//!         .key_id(key_id)
//!         .team_id(team_id)
//!         .user_token(Some("user-token-from-musickit".to_string())) // Optional: from MusicKit JS
//!         .build()?;
//!
//!     // Create the client
//!     let client = AppleMusicClient::new(config).await?;
//!
//!     // Search for songs
//!     let songs = client.search_songs("Bohemian Rhapsody").await?;
//!     println!("Found {} songs", songs.len());
//!
//!     // Create a playlist (requires user token)
//!     let playlist = client.create_library_playlist("My Playlist", None).await?;
//!
//!     // Add tracks to playlist
//!     let track_ids: Vec<&str> = songs.iter().take(3).map(|s| s.id.as_str()).collect();
//!     client.add_tracks_to_playlist(&playlist.id, &track_ids).await?;
//!
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod http;
pub mod models;
pub mod utils;

#[cfg(test)]
mod test_deserialize;

// Re-export main types for convenience
pub use auth::create_developer_token;
pub use client::AppleMusicClient;
pub use config::ClientConfig;
pub use error::AppleMusicError;
pub use models::*;
// Re-export common types
pub use reqwest;
pub use serde_json;

#[cfg(test)]
mod test_step_by_step;

#[cfg(test)]
mod test_json_validity;
