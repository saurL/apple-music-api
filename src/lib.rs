//! # Apple Music API Client
//!
//! A Rust client for interacting with the Apple Music API using rustls for secure HTTP requests.
//!
//! ## Features
//!
//! - Secure HTTP client with rustls
//! - JWT-based developer token authentication
//! - User token support for personalized requests
//! - Strongly typed API responses
//! - Comprehensive error handling
//!
//! ## Example
//!
//! ```rust,no_run
//! use apple_music_api::{AppleMusicClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig::new("your-developer-token")?;
//!     let client = AppleMusicClient::new(config)?;
//!
//!     // Search for songs
//!     let results = client.search("Hello", &["songs"]).await?;
//!     println!("Found {} songs", results.songs.len());
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

// Re-export main types for convenience
pub use client::AppleMusicClient;
pub use config::ClientConfig;
pub use error::AppleMusicError;
pub use models::*;

// Re-export common types
pub use reqwest;
pub use serde_json;
