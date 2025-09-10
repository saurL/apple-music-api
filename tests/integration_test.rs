//! Integration tests for the Apple Music API client
//!
//! These tests require valid API credentials to run.
//! Set the following environment variables:
//! - APPLE_MUSIC_DEVELOPER_TOKEN: Your Apple Music developer token
//! - APPLE_MUSIC_USER_TOKEN: Your Apple Music user token (optional)

use apple_music_api::config::MediaType;
use apple_music_api::{AppleMusicClient, ClientConfig};
use p256::ecdsa::SigningKey;
use p256::elliptic_curve::rand_core::OsRng;
use p256::pkcs8::EncodePrivateKey;
use pem::Pem;
/// Test basic client creation
#[tokio::test]
async fn test_client_creation() {
    let config = generate_fake_config();
    let client = AppleMusicClient::new(config).await;
    assert!(client.is_ok());
}

/// Test media type enum
#[test]
fn test_media_types() {
    assert_eq!(MediaType::Songs.as_str(), "songs");
    assert_eq!(MediaType::Albums.as_str(), "albums");
    assert_eq!(MediaType::Artists.as_str(), "artists");
    assert_eq!(MediaType::Playlists.as_str(), "playlists");
}

/// Test storefront parsing
#[test]
fn test_storefront_parsing() {
    use apple_music_api::utils::parse_storefront;

    assert!(parse_storefront("us").is_ok());
    assert!(parse_storefront("fr").is_ok());
    assert!(parse_storefront("jp").is_ok());
    assert!(parse_storefront("invalid").is_err());
    assert!(parse_storefront("").is_err());
}

/// Test resource ID validation
#[test]
fn test_resource_id_validation() {
    use apple_music_api::utils::validate_resource_id;

    assert!(validate_resource_id("123456789").is_ok());
    assert!(validate_resource_id("abc123").is_ok());
    assert!(validate_resource_id("valid-id_123").is_ok());
    assert!(validate_resource_id("").is_err());
    assert!(validate_resource_id(&"a".repeat(101)).is_err());
    assert!(validate_resource_id("invalid@id").is_err());
}

/// Integration test that requires valid credentials
/// This test is ignored by default - remove #[ignore] to run with real credentials
#[tokio::test]
#[ignore]
async fn test_real_api_search() {
    let team_id = std::env::var("APPLE_MUSIC_TEAM_ID").expect("APPLE_MUSIC_TEAM_ID must be set");
    let key_id = std::env::var("APPLE_MUSIC_KEY_ID").expect("APPLE_MUSIC_KEY_ID must be set");
    let private_key =
        std::env::var("APPLE_MUSIC_PRIVATE_KEY").expect("APPLE_MUSIC_PRIVATE_KEY must be set");

    let config = ClientConfig::new(team_id, key_id, private_key).unwrap();
    let client = AppleMusicClient::new(config).await.unwrap();

    // Test search
    let result = client.search("Hello", &[MediaType::Songs]).await;
    assert!(result.is_ok());

    let search_response = result.unwrap();
    // Should have some results
    assert!(search_response.results.songs.is_some() || search_response.results.albums.is_some());
}

/// Integration test for storefront information
/// This test is ignored by default - remove #[ignore] to run with real credentials
#[tokio::test]
#[ignore]
async fn test_real_api_storefront() {
    let team_id = std::env::var("APPLE_MUSIC_TEAM_ID").expect("APPLE_MUSIC_TEAM_ID must be set");
    let key_id = std::env::var("APPLE_MUSIC_KEY_ID").expect("APPLE_MUSIC_KEY_ID must be set");
    let private_key =
        std::env::var("APPLE_MUSIC_PRIVATE_KEY").expect("APPLE_MUSIC_PRIVATE_KEY must be set");

    let config = ClientConfig::new(team_id, key_id, private_key).unwrap();
    let client = AppleMusicClient::new(config).await.unwrap();

    // Test storefront retrieval
    let result = client.get_storefront().await;
    assert!(result.is_ok());

    let storefront = result.unwrap();
    if let Some(attrs) = &storefront.attributes {
        assert!(!attrs.name.is_empty());
    } else {
        panic!("Storefront attributes should be present");
    }
}

/// Test error handling with invalid requests
#[tokio::test]
async fn test_error_handling() {
    let client = AppleMusicClient::new(generate_fake_config()).await.unwrap();

    // This should fail with an authentication error
    let result = client.get_album("invalid-id").await;
    assert!(result.is_err());

    let error = result.unwrap_err();
    // Should be an API error
    match error {
        apple_music_api::AppleMusicError::Api { .. } => {}
        _ => panic!("Expected API error, got {:?}", error),
    }
}

#[tokio::test]
async fn test_invalid_private_key() {
    let team_id = "FAKE_ID";
    let key_id = "FAKE_KEY_ID";
    let private_key = "FAKE_PRIVATE_KEY";

    let config = ClientConfig::new(team_id, key_id, private_key);
    assert!(config.is_err());
}

fn generate_fake_config() -> ClientConfig {
    let team_id = "FAKE_ID";
    let key_id = "FAKE_KEY_ID";
    let mut rng = OsRng;
    let signing_key = SigningKey::random(&mut rng);

    // Encode la clé en PEM (PKCS#8)
    let private_key_bytes = signing_key.to_pkcs8_der().unwrap();
    let pem = Pem::new("PRIVATE KEY", private_key_bytes.as_bytes().to_vec());

    let pem_str = pem::encode(&pem);

    ClientConfig::new(team_id, key_id, &pem_str).unwrap()
}
