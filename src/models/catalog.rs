//! Data models for Apple Music catalog API responses

use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Song resource from the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
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
    pub attributes: SongAttributes,

    /// The song relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<SongRelationships>,
}

/// Song attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongAttributes {
    /// The album name
    #[serde(rename = "albumName")]
    pub album_name: String,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: String,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Artwork,

    /// The composer name
    #[serde(rename = "composerName")]
    pub composer_name: Option<String>,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The disc number
    #[serde(rename = "discNumber")]
    pub disc_number: Option<u32>,

    /// The duration in milliseconds
    #[serde(rename = "durationInMillis")]
    pub duration_in_millis: Option<u64>,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// Whether the song has lyrics
    #[serde(rename = "hasLyrics")]
    pub has_lyrics: bool,

    /// Whether the song is Apple Digital Master
    #[serde(rename = "isAppleDigitalMaster")]
    pub is_apple_digital_master: bool,

    /// Whether the song is available in Hi-Res Lossless
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,

    /// The song name
    #[serde(rename = "name")]
    pub name: String,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,

    /// The preview URL
    #[serde(rename = "previews")]
    pub previews: Vec<Preview>,

    /// The release date
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,

    /// The track number
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The work name
    #[serde(rename = "workName")]
    pub work_name: Option<String>,
}

/// Song relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongRelationships {
    /// The albums relationship
    #[serde(rename = "albums")]
    pub albums: Option<Relationship<Album>>,

    /// The artists relationship
    #[serde(rename = "artists")]
    pub artists: Option<Relationship<Artist>>,

    /// The genres relationship
    #[serde(rename = "genres")]
    pub genres: Option<Relationship<Genre>>,
}

/// Preview information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preview {
    /// The preview URL
    #[serde(rename = "url")]
    pub url: String,
}

/// Album resource from the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Album {
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
    pub attributes: AlbumAttributes,

    /// The album relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<AlbumRelationships>,
}

/// Album attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumAttributes {
    /// The album name
    #[serde(rename = "name")]
    pub name: String,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: String,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Artwork,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The copyright
    #[serde(rename = "copyright")]
    pub copyright: Option<String>,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// Whether the album is complete
    #[serde(rename = "isComplete")]
    pub is_complete: bool,

    /// Whether the album is a compilation
    #[serde(rename = "isCompilation")]
    pub is_compilation: bool,

    /// Whether the album is a single
    #[serde(rename = "isSingle")]
    pub is_single: bool,

    /// The record label
    #[serde(rename = "recordLabel")]
    pub record_label: Option<String>,

    /// The release date
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,

    /// The track count
    #[serde(rename = "trackCount")]
    pub track_count: u32,

    /// The UPC
    #[serde(rename = "upc")]
    pub upc: Option<String>,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,
}

/// Album relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlbumRelationships {
    /// The artists relationship
    #[serde(rename = "artists")]
    pub artists: Option<Relationship<Artist>>,

    /// The genres relationship
    #[serde(rename = "genres")]
    pub genres: Option<Relationship<Genre>>,

    /// The tracks relationship
    #[serde(rename = "tracks")]
    pub tracks: Option<Relationship<Song>>,
}

/// Artist resource from the catalog
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
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
    pub attributes: ArtistAttributes,

    /// The artist relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<ArtistRelationships>,
}

/// Artist attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistAttributes {
    /// The artist name
    #[serde(rename = "name")]
    pub name: String,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,
}

/// Artist relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtistRelationships {
    /// The albums relationship
    #[serde(rename = "albums")]
    pub albums: Option<Relationship<Album>>,

    /// The genres relationship
    #[serde(rename = "genres")]
    pub genres: Option<Relationship<Genre>>,

    /// The music videos relationship
    #[serde(rename = "music-videos")]
    pub music_videos: Option<Relationship<MusicVideo>>,

    /// The playlists relationship
    #[serde(rename = "playlists")]
    pub playlists: Option<Relationship<Playlist>>,
}

/// Music video resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideo {
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
    pub attributes: MusicVideoAttributes,
}

/// Music video attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicVideoAttributes {
    /// The album name
    #[serde(rename = "albumName")]
    pub album_name: Option<String>,

    /// The artist name
    #[serde(rename = "artistName")]
    pub artist_name: String,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Artwork,

    /// The content rating
    #[serde(rename = "contentRating")]
    pub content_rating: Option<String>,

    /// The duration in milliseconds
    #[serde(rename = "durationInMillis")]
    pub duration_in_millis: Option<u64>,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The genre names
    #[serde(rename = "genreNames")]
    pub genre_names: Vec<String>,

    /// Whether the music video has 4K
    #[serde(rename = "has4K")]
    pub has_4k: bool,

    /// Whether the music video has HDR
    #[serde(rename = "hasHDR")]
    pub has_hdr: bool,

    /// The ISRC
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,

    /// The music video name
    #[serde(rename = "name")]
    pub name: String,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,

    /// The previews
    #[serde(rename = "previews")]
    pub previews: Vec<Preview>,

    /// The release date
    #[serde(rename = "releaseDate")]
    pub release_date: DateTime<Utc>,

    /// The track number
    #[serde(rename = "trackNumber")]
    pub track_number: Option<u32>,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The video subtype
    #[serde(rename = "videoSubType")]
    pub video_sub_type: Option<String>,
}

/// Playlist resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
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
    pub attributes: PlaylistAttributes,

    /// The playlist relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<PlaylistRelationships>,
}

/// Playlist attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistAttributes {
    /// The curator name
    #[serde(rename = "curatorName")]
    pub curator_name: Option<String>,

    /// The description
    #[serde(rename = "description")]
    pub description: Option<EditorialNotes>,

    /// The last modified date
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: DateTime<Utc>,

    /// The playlist name
    #[serde(rename = "name")]
    pub name: String,

    /// The playlist type
    #[serde(rename = "playlistType")]
    pub playlist_type: Option<String>,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The play parameters
    #[serde(rename = "playParams")]
    pub play_params: Option<PlayParameters>,
}

/// Playlist relationships
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistRelationships {
    /// The curator relationship
    #[serde(rename = "curator")]
    pub curator: Option<Relationship<Curator>>,

    /// The tracks relationship
    #[serde(rename = "tracks")]
    pub tracks: Option<Relationship<Song>>,
}

impl Default for PlaylistRelationships {
    fn default() -> Self {
        PlaylistRelationships {
            curator: None,
            tracks: None,
        }
    }
}

/// Curator resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Curator {
    /// The curator ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The curator href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The curator attributes
    #[serde(rename = "attributes")]
    pub attributes: CuratorAttributes,
}

/// Curator attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuratorAttributes {
    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The curator name
    #[serde(rename = "name")]
    pub name: String,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,
}
