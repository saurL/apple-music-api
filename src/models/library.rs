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
    #[serde(rename = "releaseDate", default, deserialize_with = "crate::utils::deserialize_optional_date")]
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
    #[serde(rename = "releaseDate", default, deserialize_with = "crate::utils::deserialize_optional_date")]
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
    #[serde(rename = "releaseDate", default, deserialize_with = "crate::utils::deserialize_optional_date")]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlaylistRequest {
    /// Playlist attributes containing the name and optional description
    #[serde(rename = "attributes")]
    pub attributes: CreatePlaylistAttributes,
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

    /// The resource type (typically "songs")
    #[serde(rename = "type")]
    pub resource_type: String,
}
