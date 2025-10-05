//! Authentication handling for Apple Music API
//!
//! This module provides functions to generate developer tokens required for
//! authenticating with the Apple Music API.
//!
//! ## Developer Token
//!
//! The developer token is a JWT (JSON Web Token) that you generate using your:
//! - **Team ID**: Your Apple Developer Team ID
//! - **Key ID**: The ID of your MusicKit private key from App Store Connect
//! - **Private Key**: The ES256 private key (.p8 file) downloaded from App Store Connect
//!
//! For more information, see Apple's documentation:
//! <https://developer.apple.com/documentation/applemusicapi/generating_developer_tokens>
//!
//! ## User Token
//!
//! For personalized requests (accessing a user's library, creating playlists, etc.),
//! you also need a **user token**. The user token must be obtained from MusicKit JS
//! on the client side and passed to your backend.
//!
//! See: <https://developer.apple.com/documentation/musickit/musickit_js/getting_keys_and_creating_tokens>
//!
//! ## Example
//!
//! ```rust,no_run
//! use apple_music_api::create_developer_token;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let team_id = "YOUR_TEAM_ID";
//! let key_id = "YOUR_KEY_ID";
//! let private_key = include_str!("../certs/AuthKey.p8");
//!
//! let developer_token = create_developer_token(team_id, key_id, private_key)?;
//! println!("Developer token: {}", developer_token);
//! # Ok(())
//! # }
//! ```

use crate::error::{AppleMusicError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

/// Claims for Apple Music developer token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer (your Team ID from Apple Developer account)
    pub iss: String,

    /// Issued at time (UNIX timestamp)
    pub iat: i64,

    /// Expiration time (UNIX timestamp)
    ///
    /// Apple Music developer tokens can have a maximum lifetime of 6 months (180 days)
    pub exp: i64,
}

/// Generate a developer token for the Apple Music API
///
/// Creates a JWT (JSON Web Token) signed with the ES256 algorithm using your private key.
/// The token is valid for 180 days (6 months), which is the maximum allowed by Apple.
///
/// # Arguments
///
/// * `team_id` - Your Apple Developer Team ID (10 characters, e.g., "A1B2C3D4E5")
/// * `key_id` - Your MusicKit key ID from App Store Connect (10 characters)
/// * `private_key` - The content of your .p8 private key file (ES256 format)
///
/// # Returns
///
/// Returns a JWT string that can be used as the developer token for API requests.
///
/// # Errors
///
/// * Returns an error if the private key format is invalid
/// * Returns an error if the JWT encoding fails
///
/// # Example
///
/// ```rust,no_run
/// use apple_music_api::create_developer_token;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Load credentials from environment or configuration
/// let team_id = std::env::var("APPLE_TEAM_ID")?;
/// let key_id = std::env::var("APPLE_KEY_ID")?;
/// let private_key = include_str!("../certs/AuthKey_apple_music.p8");
///
/// // Generate the developer token
/// let token = create_developer_token(&team_id, &key_id, private_key)?;
///
/// // Use the token with the API client
/// println!("Generated token: {}", token);
/// # Ok(())
/// # }
/// ```
///
/// # Apple Documentation
///
/// For more information about developer tokens, see:
/// <https://developer.apple.com/documentation/applemusicapi/generating_developer_tokens>
pub fn create_developer_token(team_id: &str, key_id: &str, private_key: &str) -> Result<String> {
    // Create the JWT header with ES256 algorithm and key ID
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(key_id.to_string());

    let now = Utc::now().timestamp();

    // Create claims with 180 days (6 months) expiration - the maximum allowed
    let claims = Claims {
        iss: team_id.to_string(),
        iat: now,
        exp: now + Duration::days(180).num_seconds(),
    };

    // Create encoding key from the ES256 private key
    let encoding_key = EncodingKey::from_ec_pem(private_key.as_bytes())
        .map_err(|e| AppleMusicError::auth(format!("Failed to create encoding key: {}", e)))?;

    // Generate and encode the JWT
    let token = encode(&header, &claims, &encoding_key)
        .map_err(|e| AppleMusicError::auth(format!("Failed to encode JWT: {}", e)))?;

    Ok(token)
}
