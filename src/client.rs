//! Main Apple Music API client

use crate::{
    config::{ClientConfig, MediaType, SearchOptions},
    error::{AppleMusicError, Result},
    http::HttpClient,
    models::{catalog::*, common::*, library::*, search::*},
    utils::SearchParamsBuilder,
};
use std::sync::Arc;

/// Main Apple Music API client
///
/// The client handles all interactions with the Apple Music API, including catalog search,
/// library management, and personalized user requests.
///
/// # Examples
///
/// Basic usage with developer token:
///
/// ```rust,no_run
/// use apple_music_api::{AppleMusicClient, ClientConfig, create_developer_token};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let team_id = "YOUR_TEAM_ID";
/// let key_id = "YOUR_KEY_ID";
/// let private_key = include_str!("../certs/AuthKey.p8");
///
/// let developer_token = create_developer_token(team_id, key_id, private_key)?;
///
/// let config = ClientConfig::builder()
///     .developer_token(developer_token)
///     .build()?;
///
/// let client = AppleMusicClient::new(config).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct AppleMusicClient {
    http_client: Arc<HttpClient>,
    config: ClientConfig,
}

impl AppleMusicClient {
    /// Create a new Apple Music client with the given configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Client configuration including developer token and optional user token
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid
    pub async fn new(config: ClientConfig) -> Result<Self> {
        config.validate()?;

        let mut http_client = HttpClient::new(&config)?;
        if let Some(user_token) = &config.user_token {
            http_client.set_user_token(Some(user_token.clone()));
        }

        Ok(Self {
            http_client: Arc::new(http_client),
            config,
        })
    }

    /// Set the user token for personalized requests
    ///
    /// The user token should be obtained from MusicKit JS on the client side.
    ///
    /// # Arguments
    ///
    /// * `user_token` - Optional user token from MusicKit
    pub async fn set_user_token(&mut self, user_token: Option<String>) -> Result<()> {
        Arc::get_mut(&mut self.http_client)
            .ok_or_else(|| AppleMusicError::config("Cannot modify HTTP client while in use"))?
            .set_user_token(user_token);
        Ok(())
    }

    /// Get the current user token
    pub async fn user_token(&self) -> Option<String> {
        self.http_client.user_token().map(|s| s.to_string())
    }

    /// Check if user token is available
    pub async fn has_user_token(&self) -> bool {
        self.http_client.has_user_token()
    }

    // ===== CATALOG API METHODS =====

    /// Search for songs by name in the Apple Music catalog
    ///
    /// This is a convenience method that searches specifically for songs.
    /// For more control over search parameters, use [`search`](Self::search) or
    /// [`search_with_options`](Self::search_with_options).
    ///
    /// # Arguments
    ///
    /// * `name` - The search term (song name, artist, etc.)
    ///
    /// # Returns
    ///
    /// Returns a vector of [`Song`] objects matching the search term.
    /// If no songs are found, returns an empty vector.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # let client = AppleMusicClient::new(config).await?;
    /// let songs = client.search_songs("Bohemian Rhapsody").await?;
    /// for song in songs {
    ///     println!("Found: {} by {}", song.attributes.name, song.attributes.artist_name);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_songs(&self, name: &str) -> Result<Vec<Song>> {
        let response = self.search(name, &[MediaType::Songs]).await?;
        Ok(response.results.songs.map(|s| s.data).unwrap_or_default())
    }

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
    pub async fn get_catalog_playlist(&self, id: &str) -> Result<Playlist> {
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

    /// Get a playlist by ID
    pub async fn get_library_playlist(&self, id: &str) -> Result<Playlist> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/me/library/playlists/{}", id);
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

    /// Get a library playlist by ID with tracks included
    pub async fn get_library_playlist_with_tracks(&self, id: &str) -> Result<LibraryPlaylist> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/me/library/playlists/{}", id);
        let params = vec![("include", "tracks".to_string())];
        let params: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        let response: ApiResponse<LibraryPlaylist> = self
            .http_client
            .request(&path)
            .query_params(params)
            .get_json()
            .await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: "Playlist not found".to_string(),
            })
    }

    /// Get a playlist by ID with its tracks included
    pub async fn get_playlist_with_tracks(&self, id: &str) -> Result<Playlist> {
        crate::utils::validate_resource_id(id)?;

        let path = format!("v1/catalog/{}/playlists/{}", self.config.storefront, id);
        let params = vec![("include", "tracks".to_string())];
        let params: Vec<(String, String)> = params
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        let response: ApiResponse<Playlist> = self
            .http_client
            .request(&path)
            .query_params(params)
            .get_json()
            .await?;

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

    /// Create a new playlist in the user's library
    ///
    /// Creates a new, empty playlist in the authenticated user's library.
    /// After creation, you can add tracks using [`add_tracks_to_playlist`](Self::add_tracks_to_playlist).
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the playlist to create
    /// * `description` - Optional description for the playlist
    ///
    /// # Returns
    ///
    /// Returns the newly created [`LibraryPlaylist`] object containing the playlist ID
    /// and other metadata.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the playlist creation fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// // Get the root folder and create a subfolder
    /// let root = client.get_root_library_folder().await?;
    /// let folder = client.create_library_folder("My Playlists", &root.id).await?;
    ///
    /// // Create a private playlist in that folder
    /// let playlist = client.create_library_playlist(
    ///     "My Awesome Playlist",
    ///     &folder.id,
    ///     Some("A collection of my favorite tracks"),
    ///     Some(false)
    /// ).await?;
    /// println!("Created playlist with ID: {}", playlist.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_library_playlist(
        &self,
        name: &str,
        folder_id: &str,
        description: Option<&str>,
        is_public: Option<bool>,
    ) -> Result<LibraryPlaylist> {
        self.check_user_token()?;

        let request = CreatePlaylistRequest::new(
            name,
            folder_id,
            description.map(|s| s.to_string()),
            is_public,
        );

        let response: CreatePlaylistResponse = self
            .http_client
            .post_json("v1/me/library/playlists", &request)
            .await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 500,
                message: "Failed to create playlist".to_string(),
            })
    }

    /// Create a new library playlist folder
    ///
    /// Creates a new folder in the user's Apple Music library to organize playlists.
    ///
    /// # Arguments
    ///
    /// * `folder_name` - The name of the folder to create
    ///
    /// # Returns
    ///
    /// Returns a `LibraryPlaylistFolder` containing the folder's ID, type, and attributes.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the folder creation fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// // Get the root folder
    /// let root = client.get_root_library_folder().await?;
    ///
    /// // Create a folder in the root
    /// let folder = client.create_library_folder("Mes Playlists API", &root.id).await?;
    /// println!("Created folder with ID: {} and name: {}", folder.id, folder.attributes.name);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_library_folder(
        &self,
        folder_name: &str,
        parent_folder_id: &str,
    ) -> Result<LibraryPlaylistFolder> {
        self.check_user_token()?;

        let request = CreatePlaylistFolderRequest::new(folder_name, parent_folder_id);

        let response: CreatePlaylistFolderResponse = self
            .http_client
            .post_json("v1/me/library/playlist-folders", &request)
            .await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 500,
                message: "Failed to create folder".to_string(),
            })
    }

    /// Get all library playlist folders
    ///
    /// Retrieves all playlist folders from the user's Apple Music library.
    ///
    /// # Returns
    ///
    /// Returns a `LibraryPlaylistFoldersResponse` containing all folders.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the request fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// let folders = client.get_library_folders().await?;
    /// for folder in folders.data {
    ///     println!("Folder: {} (ID: {})", folder.attributes.name, folder.id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_library_folders(&self) -> Result<LibraryPlaylistFoldersResponse> {
        self.check_user_token()?;

        let response: LibraryPlaylistFoldersResponse = self
            .http_client
            .get_json("v1/me/library/playlist-folders")
            .await?;

        Ok(response)
    }

    /// Get the root library playlists folder
    ///
    /// Retrieves the root folder for playlist organization in the user's Apple Music library.
    /// This is the top-level folder that contains all other playlist folders.
    ///
    /// # Returns
    ///
    /// Returns a `LibraryPlaylistFolder` representing the root folder.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the request fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// let root_folder = client.get_root_library_folder().await?;
    /// println!("Root folder ID: {}", root_folder.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_root_library_folder(&self) -> Result<LibraryPlaylistFolder> {
        self.check_user_token()?;

        let response: LibraryPlaylistFoldersResponse = self
            .http_client
            .request("v1/me/library/playlist-folders")
            .query_param("filter[identity]", "playlistsroot")
            .get_json()
            .await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 500,
                message: "Root folder not found".to_string(),
            })
    }

    /// Get a library playlist folder by ID
    ///
    /// Fetches a specific library playlist folder using its unique identifier.
    ///
    /// # Arguments
    ///
    /// * `folder_id` - The unique identifier for the library playlist folder
    ///
    /// # Returns
    ///
    /// Returns a `LibraryPlaylistFolder` for the specified ID.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the folder is not found or the request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// let folder = client.get_library_folder_by_id("p.folder123").await?;
    /// println!("Folder: {} (ID: {})", folder.attributes.name, folder.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_library_folder_by_id(&self, folder_id: &str) -> Result<LibraryPlaylistFolder> {
        self.check_user_token()?;
        crate::utils::validate_resource_id(folder_id)?;

        let path = format!("v1/me/library/playlist-folders/{}", folder_id);
        let response: LibraryPlaylistFoldersResponse = self
            .http_client
            .get_json(&path)
            .await?;

        response
            .data
            .into_iter()
            .next()
            .ok_or_else(|| AppleMusicError::Api {
                status: 404,
                message: format!("Folder with ID '{}' not found", folder_id),
            })
    }

    /// Get or create a library playlist folder
    ///
    /// Checks if a folder with the given name already exists. If it does, returns the existing folder.
    /// If not, creates a new folder with that name in the specified parent folder.
    ///
    /// # Arguments
    ///
    /// * `folder_name` - The name of the folder to get or create
    /// * `parent_folder_id` - The ID of the parent folder (used only if creating a new folder)
    ///
    /// # Returns
    ///
    /// Returns a `LibraryPlaylistFolder` - either the existing one or a newly created one.
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an API error if the request fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// // Get the root folder
    /// let root = client.get_root_library_folder().await?;
    ///
    /// // This will return existing folder or create a new one in the root
    /// let folder = client.get_or_create_library_folder("My Playlists", &root.id).await?;
    /// println!("Folder ID: {}, Name: {}", folder.id, folder.attributes.name);
    ///
    /// // Calling again with same name returns the existing folder
    /// let same_folder = client.get_or_create_library_folder("My Playlists", &root.id).await?;
    /// assert_eq!(folder.id, same_folder.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_or_create_library_folder(
        &self,
        folder_name: &str,
        parent_folder_id: &str,
    ) -> Result<LibraryPlaylistFolder> {
        self.check_user_token()?;

        // Get all existing folders
        let folders = self.get_library_folders().await?;

        // Check if a folder with this name already exists
        if let Some(existing_folder) = folders.data.into_iter().find(|f| f.attributes.name == folder_name) {
            return Ok(existing_folder);
        }

        // Folder doesn't exist, create it in the specified parent folder
        self.create_library_folder(folder_name, parent_folder_id).await
    }

    /// Add tracks to a library playlist
    ///
    /// Adds one or more songs to an existing playlist in the user's library.
    /// The tracks are appended to the end of the playlist.
    ///
    /// # Arguments
    ///
    /// * `playlist_id` - The ID of the library playlist to add tracks to
    /// * `track_ids` - A slice of song IDs to add to the playlist
    ///
    /// # Errors
    ///
    /// * Returns an error if no user token is set. Call [`set_user_token`](Self::set_user_token) first.
    /// * Returns an error if the playlist ID or any track ID is invalid.
    /// * Returns an API error if the operation fails on the server.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::{AppleMusicClient, ClientConfig};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let mut config = ClientConfig::new("team_id".to_string(), "key_id".to_string(), "private_key_path".to_string())?;
    /// # config.user_token = Some("user_token".to_string());
    /// # let client = AppleMusicClient::new(config).await?;
    /// // First, get root and create a folder and a playlist
    /// let root = client.get_root_library_folder().await?;
    /// let folder = client.create_library_folder("Road Trips", &root.id).await?;
    /// let playlist = client.create_library_playlist("Road Trip", &folder.id, None, None).await?;
    ///
    /// // Search for some songs
    /// let songs = client.search_songs("highway").await?;
    /// let song_ids: Vec<&str> = songs.iter().take(5).map(|s| s.id.as_str()).collect();
    ///
    /// // Add the songs to the playlist
    /// client.add_tracks_to_playlist(&playlist.id, &song_ids).await?;
    /// println!("Added {} tracks to playlist", song_ids.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_tracks_to_playlist(
        &self,
        playlist_id: &str,
        track_ids: &[&str],
    ) -> Result<()> {
        self.check_user_token()?;
        crate::utils::validate_resource_id(playlist_id)?;

        for id in track_ids {
            crate::utils::validate_resource_id(id)?;
        }

        let request = AddTracksToPlaylistRequest {
            data: track_ids
                .iter()
                .map(|id| TrackReference {
                    id: id.to_string(),
                    resource_type: "songs".to_string(),
                })
                .collect(),
        };

        let path = format!("v1/me/library/playlists/{}/tracks", playlist_id);
        self.http_client
            .post_json_no_response(&path, &request)
            .await?;

        Ok(())
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
