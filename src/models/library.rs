//! Data models for Apple Music library API responses

use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Library song resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySong {
    /// The song ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The song href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The song attributes
    #[serde(rename = "attributes")]
    pub attributes: LibrarySongAttributes,
}

/// Library song attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySongAttributes {
    /// The album name
    #[serde(rename = "albumName")]
    pub album_name: Option<String>,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: Option<String>,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The disc number
    #[serde(rename = "discNumber")]
    pub disc_number: Option<u32>,

    /// The duration in milliseconds
    #[serde(rename = "durationInMillis")]
    pub duration_in_millis: Option<u64>,

    /// The genre names
    #[serde(rename = "genreNames", default)]
    pub genre_names: Vec<String>,

    /// Whether the song has lyrics
    #[serde(rename = "hasLyrics", default)]
    pub has_lyrics: bool,

    /// The ISRC
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,

    /// The song name
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,

    /// The release date
    #[serde(
        rename = "releaseDate",
        default,
        deserialize_with = "crate::utils::deserialize_optional_date"
    )]
    pub release_date: Option<DateTime<Utc>>,

    /// The track number
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,
}

/// Library song relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySongRelationships {
    /// The albums relationship
    #[serde(rename = "albums")]
    pub albums: Option<Relationship<LibraryAlbum>>,

    /// The artists relationship
    #[serde(rename = "artists")]
    pub artists: Option<Relationship<LibraryArtist>>,
}

/// Library album resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryAlbum {
    /// The album ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The album href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The album attributes
    #[serde(rename = "attributes")]
    pub attributes: LibraryAlbumAttributes,

    /// The album relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<LibraryAlbumRelationships>,
}

/// Library album attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryAlbumAttributes {
    /// The album name
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: Option<String>,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,

    /// The release date
    #[serde(
        rename = "releaseDate",
        default,
        deserialize_with = "crate::utils::deserialize_optional_date"
    )]
    pub release_date: Option<DateTime<Utc>>,

    /// The track count
    #[serde(rename = "trackCount")]
    pub track_count: Option<u32>,
}

/// Library album relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryAlbumRelationships {
    /// The artists relationship
    #[serde(rename = "artists")]
    pub artists: Option<Relationship<LibraryArtist>>,

    /// The tracks relationship
    #[serde(rename = "tracks")]
    pub tracks: Option<Relationship<LibrarySong>>,
}

/// Library artist resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtist {
    /// The artist ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The artist href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The artist attributes
    #[serde(rename = "attributes")]
    pub attributes: LibraryArtistAttributes,

    /// The artist relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<LibraryArtistRelationships>,
}

/// Library artist attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtistAttributes {
    /// The artist name
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,
}

/// Library artist relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtistRelationships {
    /// The albums relationship
    #[serde(rename = "albums")]
    pub albums: Option<Relationship<LibraryAlbum>>,
}

/// Library playlist resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylist {
    /// The playlist ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The playlist href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The playlist attributes
    #[serde(rename = "attributes")]
    pub attributes: LibraryPlaylistAttributes,

    /// The playlist relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<LibraryPlaylistRelationships>,
}

/// Library playlist attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistAttributes {
    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// Whether the playlist can be edited
    #[serde(rename = "canEdit", default)]
    pub can_edit: bool,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,

    /// The description
    #[serde(rename = "description")]
    pub description: Option<EditorialNotes>,

    /// Whether the playlist has catalog
    #[serde(rename = "hasCatalog", default)]
    pub has_catalog: bool,

    /// Whether the playlist is public
    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,

    /// The last modified date
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: Option<DateTime<Utc>>,

    /// The playlist name
    #[serde(rename = "name")]
    pub name: String,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,
}

/// Library playlist relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistRelationships {
    /// The tracks relationship
    #[serde(rename = "tracks")]
    pub tracks: Option<Relationship<LibrarySong>>,
}

impl Default for LibraryPlaylistRelationships {
    fn default() -> Self {
        Self { tracks: None }
    }
}

/// Library playlist folder resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistFolder {
    /// The folder ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The folder href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The folder attributes
    #[serde(rename = "attributes")]
    pub attributes: LibraryPlaylistFolderAttributes,
}

/// Library playlist folder attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistFolderAttributes {
    /// The folder name
    #[serde(rename = "name")]
    pub name: String,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,
}

/// Response for library playlist folders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistFoldersResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryPlaylistFolder>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Library music video resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMusicVideo {
    /// The music video ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The music video href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The music video attributes
    #[serde(rename = "attributes")]
    pub attributes: LibraryMusicVideoAttributes,
}

/// Library music video attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMusicVideoAttributes {
    /// The album name
    #[serde(rename = "albumName")]
    pub album_name: Option<String>,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: Option<String>,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The date added to library
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,

    /// The duration in milliseconds
    #[serde(rename = "durationInMillis")]
    pub duration_in_millis: Option<u64>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// Whether the music video has 4K
    #[serde(rename = "has4K")]
    pub has_4k: bool,

    /// Whether the music video has HDR
    #[serde(rename = "hasHDR")]
    pub has_hdr: bool,

    /// The music video name
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,

    /// The release date
    #[serde(
        rename = "releaseDate",
        default,
        deserialize_with = "crate::utils::deserialize_optional_date"
    )]
    pub release_date: Option<DateTime<Utc>>,

    /// The track number
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,
}

/// Response for library songs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibrarySongsResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibrarySong>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Response for library albums
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryAlbumsResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryAlbum>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Response for library artists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryArtistsResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryArtist>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Response for library playlists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryPlaylistsResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryPlaylist>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Response for library music videos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryMusicVideosResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryMusicVideo>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,

    /// Meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,
}

/// Add to library request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToLibraryRequest {
    /// The IDs to add
    #[serde(rename = "ids")]
    pub ids: Vec<String>,

    /// The media type
    #[serde(rename = "type")]
    pub media_type: String,
}

/// Add to library response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToLibraryResponse {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<LibraryResource>,
}

/// Library resource (generic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryResource {
    /// The resource ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The resource href
    #[serde(rename = "href")]
    pub href: Option<String>,
}

/// Request body for creating a new library playlist
///
/// This structure is used when creating a new playlist via the Apple Music API.
/// A parent folder ID is required.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistRequest {
    /// Playlist attributes containing the name and optional description
    #[serde(rename = "attributes")]
    pub attributes: CreatePlaylistAttributes,

    /// Relationships (tracks and parent folder)
    #[serde(rename = "relationships")]
    pub relationships: CreatePlaylistRelationships,
}

impl CreatePlaylistRequest {
    /// Create a new playlist request with required folder_id
    ///
    /// Creates a playlist in the specified folder with an empty track list by default.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the playlist
    /// * `folder_id` - The ID of the parent folder
    /// * `description` - Optional description
    /// * `is_public` - Whether the playlist is public (defaults to false)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use apple_music_api::models::library::CreatePlaylistRequest;
    /// let request = CreatePlaylistRequest::new(
    ///     "My Playlist",
    ///     "p.folder123",
    ///     Some("My description".to_string()),
    ///     Some(false)
    /// );
    /// ```
    pub fn new(
        name: impl Into<String>,
        folder_id: impl Into<String>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> Self {
        Self {
            attributes: CreatePlaylistAttributes {
                name: name.into(),
                description,
                is_public: Some(is_public.unwrap_or(false)),
            },
            relationships: CreatePlaylistRelationships {
                tracks: CreatePlaylistTracksRelationship { data: vec![] },
                parent: CreatePlaylistParentRelationship {
                    data: vec![ParentFolderReference::new(folder_id)],
                },
            },
        }
    }

    /// Create a playlist request with initial tracks
    pub fn with_tracks(
        name: impl Into<String>,
        folder_id: impl Into<String>,
        track_ids: Vec<impl Into<String>>,
        description: Option<String>,
        is_public: Option<bool>,
    ) -> Self {
        Self {
            attributes: CreatePlaylistAttributes {
                name: name.into(),
                description,
                is_public: Some(is_public.unwrap_or(false)),
            },
            relationships: CreatePlaylistRelationships {
                tracks: CreatePlaylistTracksRelationship {
                    data: track_ids
                        .into_iter()
                        .map(|id| TrackReference::new(id))
                        .collect(),
                },
                parent: CreatePlaylistParentRelationship {
                    data: vec![ParentFolderReference::new(folder_id)],
                },
            },
        }
    }
}

/// Relationships for creating a playlist
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistRelationships {
    /// Tracks to add to the playlist upon creation
    #[serde(rename = "tracks")]
    pub tracks: CreatePlaylistTracksRelationship,

    /// Parent folder for the playlist
    #[serde(rename = "parent")]
    pub parent: CreatePlaylistParentRelationship,
}

/// Tracks relationship for playlist creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistTracksRelationship {
    /// Array of track references
    #[serde(rename = "data")]
    pub data: Vec<TrackReference>,
}

/// Parent folder relationship for playlist creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistParentRelationship {
    /// Array of parent folder references
    #[serde(rename = "data")]
    pub data: Vec<ParentFolderReference>,
}

/// Reference to a parent folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentFolderReference {
    /// The folder ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type (must be "library-playlist-folders")
    #[serde(rename = "type", default = "default_parent_folder_type")]
    pub resource_type: String,
}

/// Default value for parent folder resource type
fn default_parent_folder_type() -> String {
    "library-playlist-folders".to_string()
}

impl ParentFolderReference {
    /// Create a new parent folder reference with just an ID
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            resource_type: "library-playlist-folders".to_string(),
        }
    }
}

/// Attributes for creating a playlist
///
/// Contains the required and optional fields for playlist creation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistAttributes {
    /// The name of the playlist (required)
    #[serde(rename = "name")]
    pub name: String,

    /// Optional description for the playlist
    ///
    /// This field is omitted from the JSON if None.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Whether the playlist is public
    ///
    /// This field is omitted from the JSON if None.
    #[serde(rename = "isPublic", skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

/// Response received when creating a playlist
///
/// Contains the newly created playlist data from the Apple Music API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistResponse {
    /// The created playlist data
    ///
    /// Usually contains a single playlist, but is returned as a vector
    /// to match the API response format.
    #[serde(rename = "data")]
    pub data: Vec<LibraryPlaylist>,
}

/// Request body for adding tracks to a playlist
///
/// This structure is used to add one or more songs to an existing playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTracksToPlaylistRequest {
    /// Array of track references to add to the playlist
    #[serde(rename = "data")]
    pub data: Vec<TrackReference>,
}

/// Reference to a track for adding to a playlist
///
/// Contains the minimal information needed to identify a song
/// when adding it to a playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackReference {
    /// The song/track ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type (typically "songs" or "library-music-videos")
    #[serde(rename = "type", default = "default_track_type")]
    pub resource_type: String,
}

/// Default value for track resource type
fn default_track_type() -> String {
    "songs".to_string()
}

impl TrackReference {
    /// Create a new track reference for a song with just an ID
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            resource_type: "songs".to_string(),
        }
    }

    /// Create a new track reference with a custom type (e.g., "library-music-videos")
    pub fn with_type(id: impl Into<String>, resource_type: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            resource_type: resource_type.into(),
        }
    }
}

/// Request body for creating a new library playlist folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistFolderRequest {
    /// Folder attributes containing the name
    #[serde(rename = "attributes")]
    pub attributes: CreatePlaylistFolderAttributes,
}

/// Attributes for creating a playlist folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistFolderAttributes {
    /// The name of the folder (required)
    #[serde(rename = "name")]
    pub name: String,
}

/// Response received when creating a playlist folder
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistFolderResponse {
    /// The created folder data
    #[serde(rename = "data")]
    pub data: Vec<LibraryPlaylistFolder>,
}
