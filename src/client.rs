//! Main Apple Music API client

use crate::{
    auth::{AuthBuilder, AuthConfig},
    config::{ClientConfig, MediaType, SearchOptions},
    error::{AppleMusicError, Result},
    http::HttpClient,
    models::{catalog::*, common::*, library::*, search::*},
    utils::SearchParamsBuilder,
};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Main Apple Music API client
#[derive(Clone)]
pub struct AppleMusicClient {
    http_client: Arc<HttpClient>,
    auth: Arc<Mutex<AuthBuilder>>,
    config: ClientConfig,
}

impl AppleMusicClient {
    /// Create a new Apple Music client with the given configuration
    pub async fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        // For now, we'll use simple auth with the developer token directly
        // In a real implementation, you'd want to set up proper JWT auth
        let auth =
            AuthBuilder::Simple(crate::auth::SimpleAuth::new(config.developer_token.clone()));

        let mut http_client = HttpClient::new(&config)?;
        if let Some(user_token) = &config.user_token {
            http_client.set_user_token(Some(user_token.clone()));
        }

        Ok(Self {
            http_client: Arc::new(http_client),
            auth: Arc::new(Mutex::new(auth)),
            config,
        })
    }

    /// Create a client with JWT authentication
    pub async fn with_jwt_auth(
        config: ClientConfig,
        team_id: String,
        key_id: String,
        private_key: String,
    ) -> Result<Self> {
        config.validate()?;

        let auth_config = AuthConfig::jwt(team_id, key_id, private_key);
        let auth = auth_config.build()?;

        let mut http_client = HttpClient::new(&config)?;
        if let Some(user_token) = &config.user_token {
            http_client.set_user_token(Some(user_token.clone()));
        }

        Ok(Self {
            http_client: Arc::new(http_client),
            auth: Arc::new(Mutex::new(auth)),
            config,
        })
    }

    /// Create a client with JWT authentication from private key file
    pub async fn with_jwt_from_file(
        team_id: String,
        key_id: String,
        private_key_path: String,
    ) -> Result<Self> {
        let config = ClientConfig::new(team_id, key_id, private_key_path)?;
        config.validate()?;

        // For file-based auth, we use simple auth since we already generated the token
        let auth =
            AuthBuilder::Simple(crate::auth::SimpleAuth::new(config.developer_token.clone()));

        let mut http_client = HttpClient::new(&config)?;
        if let Some(user_token) = &config.user_token {
            http_client.set_user_token(Some(user_token.clone()));
        }

        Ok(Self {
            http_client: Arc::new(http_client),
            auth: Arc::new(Mutex::new(auth)),
            config,
        })
    }

    /// Set the user token for personalized requests
    pub async fn set_user_token(&mut self, user_token: Option<String>) -> Result<()> {
        self.auth.lock().await.set_user_token(user_token.clone());
        Arc::get_mut(&mut self.http_client)
            .ok_or_else(|| AppleMusicError::config("Cannot modify HTTP client while in use"))?
            .set_user_token(user_token);
        Ok(())
    }

    /// Get the current user token
    pub async fn user_token(&self) -> Option<String> {
        self.auth.lock().await.user_token().map(|s| s.to_string())
    }

    /// Check if user token is available
    pub async fn has_user_token(&self) -> bool {
        self.auth.lock().await.has_user_token()
    }

    // ===== CATALOG API METHODS =====

    /// Search the Apple Music catalog
    pub async fn search(&self, term: &str, types: &[MediaType]) -> Result<SearchResponse> {
        self.search_with_options(term, types, &SearchOptions::default())
            .await
    }

    /// Search with additional options
    pub async fn search_with_options(
        &self,
        term: &str,
        types: &[MediaType],
        options: &SearchOptions,
    ) -> Result<SearchResponse> {
        let mut params = SearchParamsBuilder::new().term(term).types(types.to_vec());

        if let Some(limit) = options.limit {
            params = params.limit(limit);
        }

        if let Some(offset) = options.offset {
            params = params.offset(offset);
        }

        let query_params = params.build_refs();
        let query_params: Vec<(String, String)> = query_params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let response: SearchResponse = self
            .http_client
            .request("v1/catalog/{storefront}/search")
            .query_params(query_params)
            .get_json()
            .await?;

        Ok(response)
    }

    /// Get an album by ID
    pub async fn get_album(&self, id: &str) -> Result<Album> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/catalog/{}/albums/{}", self.config.storefront, id);
        let response: ApiResponse<Album> = self.http_client.get_json(&path).await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Album not found".to_string(),
            })
    }

    /// Get an artist by ID
    pub async fn get_artist(&self, id: &str) -> Result<Artist> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/catalog/{}/artists/{}", self.config.storefront, id);
        let response: ApiResponse<Artist> = self.http_client.get_json(&path).await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Artist not found".to_string(),
            })
    }

    /// Get a song by ID
    pub async fn get_song(&self, id: &str) -> Result<Song> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/catalog/{}/songs/{}", self.config.storefront, id);
        let response: ApiResponse<Song> = self.http_client.get_json(&path).await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Song not found".to_string(),
            })
    }

    /// Get a playlist by ID
    pub async fn get_playlist(&self, id: &str) -> Result<Playlist> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/catalog/{}/playlists/{}", self.config.storefront, id);
        let response: ApiResponse<Playlist> = self.http_client.get_json(&path).await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Playlist not found".to_string(),
            })
    }

    /// Get multiple albums by IDs
    pub async fn get_albums(&self, ids: &[&str]) -> Result<Vec<Album>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let ids_param = ids.join(",");
        let path = format!(
            "v1/catalog/{}/albums?ids={}",
            self.config.storefront, ids_param
        );
        let response: ApiResponse<Album> = self.http_client.get_json(&path).await?;

        Ok(response.data)
    }

    /// Get multiple artists by IDs
    pub async fn get_artists(&self, ids: &[&str]) -> Result<Vec<Artist>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let ids_param = ids.join(",");
        let path = format!(
            "v1/catalog/{}/artists?ids={}",
            self.config.storefront, ids_param
        );
        let response: ApiResponse<Artist> = self.http_client.get_json(&path).await?;

        Ok(response.data)
    }

    /// Get multiple songs by IDs
    pub async fn get_songs(&self, ids: &[&str]) -> Result<Vec<Song>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let ids_param = ids.join(",");
        let path = format!(
            "v1/catalog/{}/songs?ids={}",
            self.config.storefront, ids_param
        );
        let response: ApiResponse<Song> = self.http_client.get_json(&path).await?;

        Ok(response.data)
    }

    // ===== LIBRARY API METHODS =====
    // These require a user token

    /// Get the user's library albums
    pub async fn get_library_albums(&self) -> Result<LibraryAlbumsResponse> {
        self.check_user_token()?;

        let response: LibraryAlbumsResponse =
            self.http_client.get_json("v1/me/library/albums").await?;

        Ok(response)
    }

    /// Get the user's library artists
    pub async fn get_library_artists(&self) -> Result<LibraryArtistsResponse> {
        self.check_user_token()?;

        let response: LibraryArtistsResponse =
            self.http_client.get_json("v1/me/library/artists").await?;

        Ok(response)
    }

    /// Get the user's library songs
    pub async fn get_library_songs(&self) -> Result<LibrarySongsResponse> {
        self.check_user_token()?;

        let response: LibrarySongsResponse =
            self.http_client.get_json("v1/me/library/songs").await?;

        Ok(response)
    }

    /// Get the user's library playlists
    pub async fn get_library_playlists(&self) -> Result<LibraryPlaylistsResponse> {
        self.check_user_token()?;

        let response: LibraryPlaylistsResponse =
            self.http_client.get_json("v1/me/library/playlists").await?;

        Ok(response)
    }

    /// Add songs to the user's library
    pub async fn add_songs_to_library(&self, ids: &[&str]) -> Result<AddToLibraryResponse> {
        self.check_user_token()?;

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let request = AddToLibraryRequest {
            ids: ids.iter().map(|s| s.to_string()).collect(),
            media_type: "songs".to_string(),
        };

        let response: AddToLibraryResponse = self
            .http_client
            .post_json("v1/me/library", &request)
            .await?;

        Ok(response)
    }

    /// Add albums to the user's library
    pub async fn add_albums_to_library(&self, ids: &[&str]) -> Result<AddToLibraryResponse> {
        self.check_user_token()?;

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let request = AddToLibraryRequest {
            ids: ids.iter().map(|s| s.to_string()).collect(),
            media_type: "albums".to_string(),
        };

        let response: AddToLibraryResponse = self
            .http_client
            .post_json("v1/me/library", &request)
            .await?;

        Ok(response)
    }

    /// Add playlists to the user's library
    pub async fn add_playlists_to_library(&self, ids: &[&str]) -> Result<AddToLibraryResponse> {
        self.check_user_token()?;

        for id in ids {
            crate::utils::validate_resource_id(id)?;
        }

        let request = AddToLibraryRequest {
            ids: ids.iter().map(|s| s.to_string()).collect(),
            media_type: "playlists".to_string(),
        };

        let response: AddToLibraryResponse = self
            .http_client
            .post_json("v1/me/library", &request)
            .await?;

        Ok(response)
    }

    // ===== UTILITY METHODS =====

    /// Get the current storefront
    pub fn storefront(&self) -> &str {
        &self.config.storefront
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Check if user token is required but not available
    fn check_user_token(&self) -> Result<()> {
        if !self.http_client.has_user_token() {
            return Err(AppleMusicError::auth(
                "This operation requires a user token. Call set_user_token() first.",
            ));
        }
        Ok(())
    }

    /// Get search hints for a partial search term
    pub async fn get_search_hints(&self, term: &str) -> Result<SearchHintsResponse> {
        let params = vec![("term", term.to_string())];
        let params: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let response: SearchHintsResponse = self
            .http_client
            .request("v1/catalog/{storefront}/search/hints")
            .query_params(params)
            .get_json()
            .await?;

        Ok(response)
    }

    /// Get search suggestions
    pub async fn get_search_suggestions(&self, term: &str) -> Result<SearchSuggestionsResponse> {
        let params = vec![("term", term.to_string())];
        let params: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let response: SearchSuggestionsResponse = self
            .http_client
            .request("v1/catalog/{storefront}/search/suggestions")
            .query_params(params)
            .get_json()
            .await?;

        Ok(response)
    }

    /// Get storefront information
    pub async fn get_storefront(&self) -> Result<Storefront> {
        let path = format!("v1/storefronts/{}", self.config.storefront);
        let response: ApiResponse<Storefront> = self.http_client.get_json(&path).await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Storefront not found".to_string(),
            })
    }

    /// Get all available storefronts
    pub async fn get_storefronts(&self) -> Result<Vec<Storefront>> {
        let response: ApiResponse<Storefront> = self.http_client.get_json("v1/storefronts").await?;

        Ok(response.data)
    }
}
