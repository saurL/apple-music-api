//! Authentication handling for Apple Music API

use crate::error::{AppleMusicError, Result};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use serde::{Deserialize, Serialize};
use std::time::{Duration as StdDuration, SystemTime, UNIX_EPOCH};

/// Claims for Apple Music developer token
#[derive(Debug, Serialize, Deserialize)]
struct DeveloperTokenClaims {
    /// Issuer (your developer account ID)
    iss: String,

    /// Issued at time
    iat: u64,

    /// Expiration time
    exp: u64,
}

/// Authentication manager for Apple Music API
#[derive(Debug, Clone)]
pub struct AuthManager {
    /// Developer account ID (Team ID)
    team_id: String,

    /// Key ID from App Store Connect
    key_id: String,

    /// Private key for signing tokens
    private_key: String,

    /// Current developer token
    current_token: Option<String>,

    /// Token expiration time
    token_expires_at: Option<SystemTime>,

    /// User token for personalized requests
    user_token: Option<String>,
}

impl AuthManager {
    /// Create a new authentication manager
    pub fn new(team_id: String, key_id: String, private_key: String) -> Self {
        Self {
            team_id,
            key_id,
            private_key,
            current_token: None,
            token_expires_at: None,
            user_token: None,
        }
    }

    /// Create authentication manager from private key PEM content
    pub fn from_pem(team_id: String, key_id: String, pem_content: String) -> Result<Self> {
        // Validate PEM format (basic check)
        if !pem_content.contains("-----BEGIN PRIVATE KEY-----") {
            return Err(AppleMusicError::auth(
                "Invalid PEM format: missing BEGIN PRIVATE KEY header",
            ));
        }

        if !pem_content.contains("-----END PRIVATE KEY-----") {
            return Err(AppleMusicError::auth(
                "Invalid PEM format: missing END PRIVATE KEY header",
            ));
        }

        Ok(Self::new(team_id, key_id, pem_content))
    }

    /// Generate a new developer token
    pub fn generate_developer_token(&mut self) -> Result<String> {
        let now = SystemTime::now();
        let iat = now
            .duration_since(UNIX_EPOCH)
            .map_err(|_| AppleMusicError::auth("System time is before UNIX epoch"))?
            .as_secs();

        // Apple Music tokens expire after 6 months (15777000 seconds)
        let exp = iat + 15777000;

        let claims = DeveloperTokenClaims {
            iss: self.team_id.clone(),
            iat,
            exp,
        };

        let header = Header {
            alg: Algorithm::ES256,
            kid: Some(self.key_id.clone()),
            ..Default::default()
        };

        let encoding_key = EncodingKey::from_ec_pem(self.private_key.as_bytes())
            .map_err(|e| AppleMusicError::auth(format!("Failed to create encoding key: {}", e)))?;

        let token = encode(&header, &claims, &encoding_key)
            .map_err(|e| AppleMusicError::auth(format!("Failed to encode JWT: {}", e)))?;

        self.current_token = Some(token.clone());
        self.token_expires_at = Some(now + StdDuration::from_secs(15777000));

        Ok(token)
    }

    /// Get the current developer token, generating a new one if necessary
    pub fn get_developer_token(&mut self) -> Result<&str> {
        // Check if we need to generate a new token
        let should_generate = match (&self.current_token, self.token_expires_at) {
            (Some(_), Some(expires_at)) => {
                // Generate new token if it expires within 24 hours
                SystemTime::now() + StdDuration::from_secs(86400) > expires_at
            }
            _ => true,
        };

        if should_generate {
            self.generate_developer_token()?;
        }

        self.current_token
            .as_deref()
            .ok_or_else(|| AppleMusicError::auth("No developer token available"))
    }

    /// Set the user token for personalized requests
    pub fn set_user_token(&mut self, user_token: Option<String>) {
        self.user_token = user_token;
    }

    /// Get the current user token
    pub fn get_user_token(&self) -> Option<&str> {
        self.user_token.as_deref()
    }

    /// Check if user token is available
    pub fn has_user_token(&self) -> bool {
        self.user_token.is_some()
    }

    /// Clear all tokens
    pub fn clear_tokens(&mut self) {
        self.current_token = None;
        self.token_expires_at = None;
        self.user_token = None;
    }

    /// Get token expiration time
    pub fn token_expires_at(&self) -> Option<SystemTime> {
        self.token_expires_at
    }

    /// Check if developer token is expired
    pub fn is_token_expired(&self) -> bool {
        match self.token_expires_at {
            Some(expires_at) => SystemTime::now() >= expires_at,
            None => true,
        }
    }

    /// Get time until token expires
    pub fn time_until_expiry(&self) -> Option<StdDuration> {
        self.token_expires_at?
            .duration_since(SystemTime::now())
            .ok()
    }
}

/// Simplified authentication for cases where you already have a developer token
#[derive(Debug, Clone)]
pub struct SimpleAuth {
    /// Pre-generated developer token
    developer_token: String,

    /// User token for personalized requests
    user_token: Option<String>,
}

impl SimpleAuth {
    /// Create a new simple authentication with a developer token
    pub fn new(developer_token: String) -> Self {
        Self {
            developer_token,
            user_token: None,
        }
    }

    /// Set the user token
    pub fn with_user_token(mut self, user_token: String) -> Self {
        self.user_token = Some(user_token);
        self
    }

    /// Get the developer token
    pub fn developer_token(&self) -> &str {
        &self.developer_token
    }

    /// Get the user token
    pub fn user_token(&self) -> Option<&str> {
        self.user_token.as_deref()
    }

    /// Set the user token
    pub fn set_user_token(&mut self, user_token: Option<String>) {
        self.user_token = user_token;
    }

    /// Check if user token is available
    pub fn has_user_token(&self) -> bool {
        self.user_token.is_some()
    }
}

/// Authentication configuration
#[derive(Debug, Clone)]
pub enum AuthConfig {
    /// Full JWT authentication with private key
    Jwt {
        team_id: String,
        key_id: String,
        private_key: String,
    },

    /// Simple authentication with pre-generated token
    Simple { developer_token: String },
}

impl AuthConfig {
    /// Create JWT authentication configuration
    pub fn jwt(team_id: String, key_id: String, private_key: String) -> Self {
        Self::Jwt {
            team_id,
            key_id,
            private_key,
        }
    }

    /// Create simple authentication configuration
    pub fn simple(developer_token: String) -> Self {
        Self::Simple { developer_token }
    }

    /// Build the appropriate authentication manager
    pub fn build(self) -> Result<AuthBuilder> {
        match self {
            Self::Jwt {
                team_id,
                key_id,
                private_key,
            } => {
                let auth_manager = AuthManager::from_pem(team_id, key_id, private_key)?;
                Ok(AuthBuilder::Jwt(auth_manager))
            }
            Self::Simple { developer_token } => {
                let simple_auth = SimpleAuth::new(developer_token);
                Ok(AuthBuilder::Simple(simple_auth))
            }
        }
    }
}

/// Authentication builder result
#[derive(Debug)]
pub enum AuthBuilder {
    /// JWT-based authentication
    Jwt(AuthManager),

    /// Simple token-based authentication
    Simple(SimpleAuth),
}

impl AuthBuilder {
    /// Get the developer token
    pub fn developer_token(&mut self) -> Result<&str> {
        match self {
            Self::Jwt(manager) => manager.get_developer_token(),
            Self::Simple(simple) => Ok(simple.developer_token()),
        }
    }

    /// Get the user token
    pub fn user_token(&self) -> Option<&str> {
        match self {
            Self::Jwt(manager) => manager.get_user_token(),
            Self::Simple(simple) => simple.user_token(),
        }
    }

    /// Set the user token
    pub fn set_user_token(&mut self, user_token: Option<String>) {
        match self {
            Self::Jwt(manager) => manager.set_user_token(user_token),
            Self::Simple(simple) => simple.set_user_token(user_token),
        }
    }

    /// Check if user token is available
    pub fn has_user_token(&self) -> bool {
        match self {
            Self::Jwt(manager) => manager.has_user_token(),
            Self::Simple(simple) => simple.has_user_token(),
        }
    }
}

/// Claims for Apple Music developer token (using chrono for timestamp handling)
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer (your developer account ID)
    pub iss: String,

    /// Issued at time
    pub iat: i64,

    /// Expiration time
    pub exp: i64,
}

/// Create a developer token by reading the private key from a file
pub fn create_developer_token(team_id: &str, key_id: &str, private_key: &str) -> Result<String> {
    // Create the JWT header
    let mut header = Header::new(Algorithm::ES256);
    header.kid = Some(key_id.to_string());

    let now = Utc::now().timestamp();

    let claims = Claims {
        iss: team_id.to_string(),
        iat: now,
        exp: now + Duration::days(180).num_seconds(), // max 6 months
    };

    // Generate the token
    let token = encode(
        &header,
        &claims,
        &EncodingKey::from_ec_pem(&private_key.as_bytes())
            .map_err(|e| AppleMusicError::auth(format!("Failed to create encoding key: {}", e)))?,
    )
    .map_err(|e| AppleMusicError::auth(format!("Failed to encode JWT: {}", e)))?;

    Ok(token)
}
