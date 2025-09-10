//! Data models for Apple Music search API responses

use super::catalog::*;
use super::common::{Artwork, EditorialNotes};
use serde::{Deserialize, Serialize};

/// Search response from the Apple Music API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    /// Search results organized by media type
    #[serde(rename = "results")]
    pub results: SearchResults,

    /// Meta information about the search
    #[serde(rename = "meta")]
    pub meta: Option<SearchMeta>,
}

/// Search results organized by media type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    /// Song search results
    #[serde(rename = "songs")]
    pub songs: Option<SearchResultData<Song>>,

    /// Album search results
    #[serde(rename = "albums")]
    pub albums: Option<SearchResultData<Album>>,

    /// Artist search results
    #[serde(rename = "artists")]
    pub artists: Option<SearchResultData<Artist>>,

    /// Playlist search results
    #[serde(rename = "playlists")]
    pub playlists: Option<SearchResultData<Playlist>>,

    /// Music video search results
    #[serde(rename = "music-videos")]
    pub music_videos: Option<SearchResultData<MusicVideo>>,

    /// Station search results
    #[serde(rename = "stations")]
    pub stations: Option<SearchResultData<Station>>,

    /// Curator search results
    #[serde(rename = "curators")]
    pub curators: Option<SearchResultData<Curator>>,

    /// Apple curator search results
    #[serde(rename = "apple-curators")]
    pub apple_curators: Option<SearchResultData<Curator>>,
}

/// Generic search result data container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultData<T> {
    /// The search result data
    #[serde(rename = "data")]
    pub data: Vec<T>,

    /// The URL for the full results
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,
}

/// Search meta information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMeta {
    /// Search results information
    #[serde(rename = "results")]
    pub results: Option<SearchResultsMeta>,
}

/// Search results meta information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultsMeta {
    /// Order of search results
    #[serde(rename = "order")]
    pub order: Vec<String>,

    /// Raw order of search results
    #[serde(rename = "rawOrder")]
    pub raw_order: Vec<String>,
}

/// Station resource for search results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    /// The station ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The station href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The station attributes
    #[serde(rename = "attributes")]
    pub attributes: StationAttributes,
}

/// Station attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StationAttributes {
    /// The artwork
    #[serde(rename = "artwork")]
    pub artwork: Artwork,

    /// The duration in milliseconds
    #[serde(rename = "durationInMillis")]
    pub duration_in_millis: Option<u64>,

    /// The editorial notes
    #[serde(rename = "editorialNotes")]
    pub editorial_notes: Option<EditorialNotes>,

    /// The episode number
    #[serde(rename = "episodeNumber")]
    pub episode_number: Option<u32>,

    /// Whether the station is live
    #[serde(rename = "isLive")]
    pub is_live: bool,

    /// The station name
    #[serde(rename = "name")]
    pub name: String,

    /// The URL
    #[serde(rename = "url")]
    pub url: String,

    /// The media kind
    #[serde(rename = "mediaKind")]
    pub media_kind: String,
}

/// Hints response for search suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHintsResponse {
    /// Search hints
    #[serde(rename = "results")]
    pub results: SearchHintsResults,
}

/// Search hints results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHintsResults {
    /// Search hint terms
    #[serde(rename = "terms")]
    pub terms: Vec<String>,
}

/// Search suggestions response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionsResponse {
    /// Search suggestions
    #[serde(rename = "results")]
    pub results: SearchSuggestionsResults,
}

/// Search suggestions results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionsResults {
    /// Suggested search terms
    #[serde(rename = "suggestions")]
    pub suggestions: Vec<SearchSuggestion>,
}

/// Individual search suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestion {
    /// The suggested search term
    #[serde(rename = "content")]
    pub content: SearchSuggestionContent,

    /// The kind of suggestion
    #[serde(rename = "kind")]
    pub kind: String,
}

/// Search suggestion content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchSuggestionContent {
    /// The display term
    #[serde(rename = "displayTerm")]
    pub display_term: String,

    /// The search term
    #[serde(rename = "searchTerm")]
    pub search_term: String,

    /// The suggestion type
    #[serde(rename = "kind")]
    pub kind: String,
}
