//! Data models for Apple Music API responses

pub mod catalog;
pub mod common;
pub mod library;
pub mod search;

// Re-export common types
pub use common::*;

// Re-export library types for public API
pub use library::{
    CreatePlaylistRelationships, CreatePlaylistTracksRelationship,
    CreatePlaylistParentRelationship, ParentFolderReference, TrackReference,
};
