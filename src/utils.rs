//! Utility functions for Apple Music API client

use crate::config::MediaType;
use crate::error::{AppleMusicError, Result};

/// Build a URL path with query parameters
pub fn build_path_with_params(path: &str, params: &[(&str, String)]) -> String {
    if params.is_empty() {
        return path.to_string();
    }

    let query_string = params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&");

    format!("{}?{}", path, query_string)
}

/// Convert media types to comma-separated string for API requests
pub fn media_types_to_string(types: &[MediaType]) -> String {
    types
        .iter()
        .map(|mt| mt.as_str())
        .collect::<Vec<_>>()
        .join(",")
}

/// Parse a storefront identifier from a string
pub fn parse_storefront(storefront: &str) -> Result<String> {
    let storefront = storefront.trim().to_lowercase();

    // Basic validation - storefront should be 2-3 characters
    if storefront.len() < 2 || storefront.len() > 3 {
        return Err(AppleMusicError::invalid_request(
            "Storefront identifier must be 2-3 characters long",
        ));
    }

    // Check if it contains only letters
    if !storefront.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(AppleMusicError::invalid_request(
            "Storefront identifier must contain only letters",
        ));
    }

    Ok(storefront)
}

/// Validate an Apple Music resource ID
pub fn validate_resource_id(id: &str) -> Result<()> {
    if id.is_empty() {
        return Err(AppleMusicError::invalid_request(
            "Resource ID cannot be empty",
        ));
    }

    if id.len() > 100 {
        return Err(AppleMusicError::invalid_request("Resource ID is too long"));
    }

    // Check for invalid characters
    if id
        .chars()
        .any(|c| !c.is_alphanumeric() && c != '-' && c != '_' && c != '.')
    {
        return Err(AppleMusicError::invalid_request(
            "Resource ID contains invalid characters. Only alphanumeric characters, hyphens, underscores, and periods are allowed"
        ));
    }

    Ok(())
}

/// Extract IDs from a list of resource identifiers
pub fn extract_ids<T: HasId>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.id().to_string()).collect()
}

/// Trait for types that have an ID
pub trait HasId {
    fn id(&self) -> &str;
}

/// Pagination helper for handling next URLs
#[derive(Debug, Clone)]
pub struct PaginationHelper {
    next_url: Option<String>,
}

impl PaginationHelper {
    /// Create a new pagination helper
    pub fn new() -> Self {
        Self { next_url: None }
    }

    /// Create from a next URL
    pub fn from_next_url(next_url: Option<String>) -> Self {
        Self { next_url }
    }

    /// Check if there are more pages
    pub fn has_next(&self) -> bool {
        self.next_url.is_some()
    }

    /// Get the next URL
    pub fn next_url(&self) -> Option<&str> {
        self.next_url.as_deref()
    }

    /// Update the next URL
    pub fn set_next_url(&mut self, next_url: Option<String>) {
        self.next_url = next_url;
    }

    /// Extract path and query from next URL
    pub fn extract_path_and_query(&self) -> Option<(&str, &str)> {
        self.next_url.as_ref()?.split_once('?')
    }
}

impl Default for PaginationHelper {
    fn default() -> Self {
        Self::new()
    }
}

/// Rate limiting helper
#[derive(Debug, Clone)]
pub struct RateLimiter {
    requests_per_second: u32,
    last_request_time: std::time::Instant,
    request_count: u32,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(requests_per_second: u32) -> Self {
        Self {
            requests_per_second,
            last_request_time: std::time::Instant::now(),
            request_count: 0,
        }
    }

    /// Wait if necessary to respect rate limits
    pub async fn wait_if_needed(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_request_time);

        // Reset counter if more than a second has passed
        if elapsed.as_secs() >= 1 {
            self.request_count = 0;
            self.last_request_time = now;
        }

        // Check if we need to wait
        if self.request_count >= self.requests_per_second {
            let wait_time = std::time::Duration::from_secs(1) - elapsed;
            tokio::time::sleep(wait_time).await;
            self.request_count = 0;
            self.last_request_time = std::time::Instant::now();
        }

        self.request_count += 1;
    }
}

/// Helper for building search parameters
#[derive(Debug, Clone, Default)]
pub struct SearchParamsBuilder {
    term: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    types: Vec<String>,
    storefront: Option<String>,
}

impl SearchParamsBuilder {
    /// Create a new search parameters builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the search term
    pub fn term<S: Into<String>>(mut self, term: S) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Set the result limit
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Add a media type
    pub fn add_type(mut self, media_type: MediaType) -> Self {
        self.types.push(media_type.as_str().to_string());
        self
    }

    /// Add multiple media types
    pub fn types(mut self, types: Vec<MediaType>) -> Self {
        self.types = types.into_iter().map(|t| t.as_str().to_string()).collect();
        self
    }

    /// Set the storefront
    pub fn storefront<S: Into<String>>(mut self, storefront: S) -> Self {
        self.storefront = Some(storefront.into());
        self
    }

    /// Build the query parameters
    pub fn build(self) -> Vec<(String, String)> {
        let mut params = Vec::new();

        if let Some(term) = self.term {
            params.push(("term".to_string(), term));
        }

        if let Some(limit) = self.limit {
            params.push(("limit".to_string(), limit.to_string()));
        }

        if let Some(offset) = self.offset {
            params.push(("offset".to_string(), offset.to_string()));
        }

        if !self.types.is_empty() {
            params.push(("types".to_string(), self.types.join(",")));
        }

        if let Some(storefront) = self.storefront {
            params.push(("storefront".to_string(), storefront));
        }

        params
    }

    /// Build the query parameters as (&str, String) tuples
    pub fn build_refs(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();

        if let Some(ref term) = self.term {
            params.push(("term", term.clone()));
        }

        if let Some(limit) = self.limit {
            params.push(("limit", limit.to_string()));
        }

        if let Some(offset) = self.offset {
            params.push(("offset", offset.to_string()));
        }

        if !self.types.is_empty() {
            params.push(("types", self.types.join(",")));
        }

        if let Some(ref storefront) = self.storefront {
            params.push(("storefront", storefront.clone()));
        }

        params
    }
}

/// Helper for parsing API response meta information
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ResponseMeta {
    /// Total number of items
    pub total: Option<u32>,

    /// Results information
    pub results: Option<serde_json::Value>,
}

impl ResponseMeta {
    /// Get the total count
    pub fn total(&self) -> Option<u32> {
        self.total
    }

    /// Check if there are more results
    pub fn has_more(&self, current_count: usize) -> bool {
        self.total
            .map_or(false, |total| current_count < total as usize)
    }
}
