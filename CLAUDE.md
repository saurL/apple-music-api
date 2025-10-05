# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library for interacting with the Apple Music API. It provides a type-safe client for both catalog operations (public Apple Music data) and library operations (user-specific data requiring authentication).

## Build and Test Commands

```bash
# Build the project
cargo build

# Build with release optimizations
cargo build --release

# Run all tests
cargo test

# Run a specific test
cargo test test_name

# Check code without building
cargo check
```

## Authentication Architecture

The library uses a two-token authentication system:

1. **Developer Token** (required): JWT generated from your Apple Developer credentials (Team ID, Key ID, and ES256 private key). Generated via `create_developer_token()` in [src/auth.rs](src/auth.rs). Tokens are valid for 180 days maximum.

2. **User Token** (optional): Required for personalized operations (library management, playlist creation). Must be obtained client-side via MusicKit JS and passed to the backend.

## Core Components

### Client Configuration ([src/config.rs](src/config.rs))
- `ClientConfig`: Main configuration struct built using the builder pattern
- `MediaType` enum: Defines searchable media types (songs, albums, artists, playlists, etc.)
- `SearchOptions`: Configures search parameters (limit, offset, types)

### HTTP Layer ([src/http.rs](src/http.rs))
- `HttpClient`: Wrapper around `reqwest` with rustls for TLS
- `RequestBuilder`: Fluent API for complex requests with query params and headers
- Handles authentication headers automatically:
  - `Authorization: Bearer {developer_token}` (always)
  - `Music-User-Token: {user_token}` (when user token is set)

### Main Client ([src/client.rs](src/client.rs))
The `AppleMusicClient` is organized into three categories:

1. **Catalog API Methods**: Public Apple Music data (search, get songs/albums/artists/playlists by ID)
2. **Library API Methods**: User-specific operations (requires user token):
   - `get_library_*()`: Retrieve user's library content
   - `add_*_to_library()`: Add content to user's library
   - `create_library_playlist()`: Create playlists
   - `add_tracks_to_playlist()`: Populate playlists
3. **Utility Methods**: Configuration accessors and helpers

### Models ([src/models/](src/models/))
- `common.rs`: Shared types (`ApiResponse<T>`, `Artwork`, `Relationship<T>`)
- `catalog.rs`: Catalog resource types (`Song`, `Album`, `Artist`, `Playlist`)
- `library.rs`: Library-specific types and request/response structures
- `search.rs`: Search result types

### Error Handling ([src/error.rs](src/error.rs))
Custom `AppleMusicError` type using `thiserror` with variants for:
- HTTP errors (wrapped `reqwest::Error`)
- API errors (status code + message from Apple)
- Authentication errors
- Configuration errors
- URL parsing errors

## Key Implementation Details

### Storefront Handling
The client uses a configurable storefront (default: "us"). API paths use `{storefront}` placeholder which the HTTP client replaces with the configured value.

### Resource ID Validation
Resource IDs are validated using `utils::validate_resource_id()` before making API calls to prevent invalid requests.

### User Token Check
Library operations call `check_user_token()` internally, returning a clear error if the user token is missing.

### Path Building
The HTTP client's `request()` method creates a `RequestBuilder` that supports:
- Dynamic path placeholders (e.g., `{storefront}`)
- Query parameter encoding
- Custom headers
