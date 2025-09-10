//! Common data types used across Apple Music API responses

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for Apple Music resources
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId {
    /// The resource identifier
    pub id: String,
}

impl ResourceId {
    /// Create a new resource ID
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self { id: id.into() }
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<S: Into<String>> From<S> for ResourceId {
    fn from(s: S) -> Self {
        Self::new(s)
    }
}

/// Attributes common to all Apple Music resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonAttributes {
    /// The name of the resource
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// The artwork for the resource
    #[serde(rename = "artwork")]
    pub artwork: Option<Artwork>,

    /// The URL for the resource
    #[serde(rename = "url")]
    pub url: Option<String>,

    /// The date the resource was added to the catalog
    #[serde(rename = "dateAdded")]
    pub date_added: Option<DateTime<Utc>>,

    /// The date the resource was last modified
    #[serde(rename = "lastModifiedDate")]
    pub last_modified_date: Option<DateTime<Utc>>,
}

/// Artwork information for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artwork {
    /// The average background color of the artwork
    #[serde(rename = "bgColor")]
    pub bg_color: Option<String>,

    /// The height of the artwork
    #[serde(rename = "height")]
    pub height: Option<u32>,

    /// The width of the artwork
    #[serde(rename = "width")]
    pub width: Option<u32>,

    /// The URL template for the artwork
    #[serde(rename = "url")]
    pub url: String,

    /// The text color for the artwork
    #[serde(rename = "textColor1")]
    pub text_color1: Option<String>,

    /// The secondary text color for the artwork
    #[serde(rename = "textColor2")]
    pub text_color2: Option<String>,

    /// The tertiary text color for the artwork
    #[serde(rename = "textColor3")]
    pub text_color3: Option<String>,

    /// The quaternary text color for the artwork
    #[serde(rename = "textColor4")]
    pub text_color4: Option<String>,
}

impl Artwork {
    /// Get the artwork URL with specified dimensions
    pub fn url_with_dimensions(&self, width: u32, height: u32) -> String {
        self.url
            .replace("{w}", &width.to_string())
            .replace("{h}", &height.to_string())
    }

    /// Get the artwork URL with square dimensions
    pub fn url_square(&self, size: u32) -> String {
        self.url_with_dimensions(size, size)
    }
}

/// Editorial notes for resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorialNotes {
    /// Short editorial note
    #[serde(rename = "short")]
    pub short: Option<String>,

    /// Standard editorial note
    #[serde(rename = "standard")]
    pub standard: Option<String>,

    /// Name of the editorial note
    #[serde(rename = "name")]
    pub name: Option<String>,

    /// Tagline for the editorial note
    #[serde(rename = "tagline")]
    pub tagline: Option<String>,
}

/// Play parameters for media resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayParameters {
    /// The ID of the resource to play
    #[serde(rename = "id")]
    pub id: String,

    /// The kind of the resource to play
    #[serde(rename = "kind")]
    pub kind: String,

    /// Whether the resource is playable
    #[serde(rename = "isLibrary")]
    pub is_library: Option<bool>,

    /// The catalog ID of the resource
    #[serde(rename = "catalogId")]
    pub catalog_id: Option<String>,

    /// Additional parameters
    #[serde(rename = "reporting")]
    pub reporting: Option<bool>,

    /// Preview parameters
    #[serde(rename = "preview")]
    pub preview: Option<PreviewParameters>,
}

/// Preview parameters for media resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewParameters {
    /// The kind of preview
    #[serde(rename = "kind")]
    pub kind: String,

    /// The URL of the preview
    #[serde(rename = "url")]
    pub url: String,
}

/// Relationship data between resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship<T> {
    /// The data for the relationship
    #[serde(rename = "data")]
    pub data: Vec<T>,

    /// The URL for the relationship
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,
}

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationMeta {
    /// Total number of items
    #[serde(rename = "total")]
    pub total: Option<u32>,
}

/// Generic resource wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource<T> {
    /// The resource ID
    #[serde(rename = "id")]
    pub id: String,

    /// The resource type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The resource href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The resource attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<T>,

    /// The resource relationships
    #[serde(rename = "relationships")]
    pub relationships: Option<std::collections::HashMap<String, Relationship<serde_json::Value>>>,
}

/// Response wrapper for API responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// The response data
    #[serde(rename = "data")]
    pub data: Vec<T>,

    /// The response results (for search responses)
    #[serde(rename = "results")]
    pub results: Option<std::collections::HashMap<String, Vec<T>>>,

    /// The response meta information
    #[serde(rename = "meta")]
    pub meta: Option<PaginationMeta>,

    /// The next URL for pagination
    #[serde(rename = "next")]
    pub next: Option<String>,
}

/// Storefront information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storefront {
    /// The storefront ID
    #[serde(rename = "id")]
    pub id: String,

    /// The storefront type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The storefront href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The storefront attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<StorefrontAttributes>,
}

/// Storefront attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorefrontAttributes {
    /// The default language tag
    #[serde(rename = "defaultLanguageTag")]
    pub default_language_tag: String,

    /// The explicit content policy
    #[serde(rename = "explicitContentPolicy")]
    pub explicit_content_policy: String,

    /// The name of the storefront
    #[serde(rename = "name")]
    pub name: String,

    /// Supported language tags
    #[serde(rename = "supportedLanguageTags")]
    pub supported_language_tags: Vec<String>,
}

/// Content rating information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentRating {
    /// The content rating name
    #[serde(rename = "name")]
    pub name: String,

    /// The content rating value
    #[serde(rename = "value")]
    pub value: i32,
}

/// Genre information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    /// The genre ID
    #[serde(rename = "id")]
    pub id: String,

    /// The genre type
    #[serde(rename = "type")]
    pub resource_type: String,

    /// The genre href
    #[serde(rename = "href")]
    pub href: Option<String>,

    /// The genre attributes
    #[serde(rename = "attributes")]
    pub attributes: Option<GenreAttributes>,
}

/// Genre attributes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreAttributes {
    /// The genre name
    #[serde(rename = "name")]
    pub name: String,

    /// The parent ID
    #[serde(rename = "parentId")]
    pub parent_id: Option<String>,

    /// The parent name
    #[serde(rename = "parentName")]
    pub parent_name: Option<String>,
}
