//! HTTP client implementation with rusttls for Apple Music API

use crate::config::ClientConfig;
use crate::error::{AppleMusicError, Result};
use reqwest::{Client, ClientBuilder, Response};

/// HTTP client wrapper for Apple Music API requests
pub struct HttpClient {
    client: Client,
    base_url: String,
    developer_token: String,
    user_token: Option<String>,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration
    pub fn new(config: &ClientConfig) -> Result<Self> {
        // Configure rustls client
        let client = ClientBuilder::new()
            .use_rustls_tls()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .build()
            .map_err(AppleMusicError::Http)?;

        Ok(Self {
            client,
            base_url: config.base_url.clone(),
            developer_token: config.developer_token.clone(),
            user_token: config.user_token.clone(),
        })
    }

    /// Execute a GET request
    pub async fn get(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path)?;
        let mut request = self.client.get(&url);

        // Add authentication headers
        request = self.add_auth_headers(request);

        let response = request.send().await.map_err(AppleMusicError::Http)?;
        self.handle_response(response).await
    }

    /// Execute a POST request with JSON body
    pub async fn post<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<Response> {
        let url = self.build_url(path)?;
        let mut request = self.client.post(&url).json(body);

        // Add authentication headers
        request = self.add_auth_headers(request);

        let response = request.send().await.map_err(AppleMusicError::Http)?;
        self.handle_response(response).await
    }

    /// Execute a PUT request with JSON body
    pub async fn put<T: serde::Serialize>(&self, path: &str, body: &T) -> Result<Response> {
        let url = self.build_url(path)?;
        let mut request = self.client.put(&url).json(body);

        // Add authentication headers
        request = self.add_auth_headers(request);

        let response = request.send().await.map_err(AppleMusicError::Http)?;
        self.handle_response(response).await
    }

    /// Execute a DELETE request
    pub async fn delete(&self, path: &str) -> Result<Response> {
        let url = self.build_url(path)?;
        let mut request = self.client.delete(&url);

        // Add authentication headers
        request = self.add_auth_headers(request);

        let response = request.send().await.map_err(AppleMusicError::Http)?;
        self.handle_response(response).await
    }

    /// Build a full URL from a path
    fn build_url(&self, path: &str) -> Result<String> {
        let path = path.trim_start_matches('/');
        format!("{}/{}", self.base_url.trim_end_matches('/'), path)
            .parse::<url::Url>()
            .map_err(AppleMusicError::Url)?
            .to_string();
        Ok(format!("{}/{}", self.base_url.trim_end_matches('/'), path))
    }

    /// Add authentication headers to a request
    fn add_auth_headers(&self, request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        let mut request = request
            .header("Authorization", format!("Bearer {}", self.developer_token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        // Add user token if available (for personalized requests)
        if let Some(user_token) = &self.user_token {
            request = request.header("Music-User-Token", user_token);
        }

        request
    }

    /// Handle API response and check for errors
    async fn handle_response(&self, response: Response) -> Result<Response> {
        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else {
            // Try to parse error response
            let error_text = response.text().await.unwrap_or_default();

            if let Ok(error_response) =
                serde_json::from_str::<crate::error::ApiErrorResponse>(&error_text)
            {
                if let Some(error) = error_response.errors.first() {
                    return Err(AppleMusicError::Api {
                        status: status.as_u16(),
                        message: error.detail.clone(),
                    });
                }
            }

            // Fallback to generic API error
            Err(AppleMusicError::Api {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }

    /// Get the response as JSON
    pub async fn get_json<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let response = self.get(path).await?;
        response.json().await.map_err(AppleMusicError::Http)
    }

    /// Post JSON and get JSON response
    pub async fn post_json<T: serde::Serialize, U: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U> {
        let response = self.post(path, body).await?;
        response.json().await.map_err(AppleMusicError::Http)
    }

    /// Put JSON and get JSON response
    pub async fn put_json<T: serde::Serialize, U: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &T,
    ) -> Result<U> {
        let response = self.put(path, body).await?;
        response.json().await.map_err(AppleMusicError::Http)
    }

    /// Update the user token
    pub fn set_user_token(&mut self, user_token: Option<String>) {
        self.user_token = user_token;
    }

    /// Get the current user token
    pub fn user_token(&self) -> Option<&str> {
        self.user_token.as_deref()
    }

    /// Check if user token is set
    pub fn has_user_token(&self) -> bool {
        self.user_token.is_some()
    }
}

/// Request builder for complex requests
pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    path: String,
    query_params: Vec<(String, String)>,
    headers: Vec<(String, String)>,
}

impl<'a> RequestBuilder<'a> {
    /// Create a new request builder
    pub fn new(client: &'a HttpClient, path: impl Into<String>) -> Self {
        Self {
            client,
            path: path.into(),
            query_params: Vec::new(),
            headers: Vec::new(),
        }
    }

    /// Add a query parameter
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// Add multiple query parameters
    pub fn query_params(mut self, params: Vec<(String, String)>) -> Self {
        self.query_params.extend(params);
        self
    }

    /// Add a custom header
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    /// Build the URL with query parameters
    fn build_url(&self) -> Result<String> {
        let mut url = self.client.build_url(&self.path)?;

        if !self.query_params.is_empty() {
            url.push('?');
            let query_string = self
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
                .collect::<Vec<_>>()
                .join("&");
            url.push_str(&query_string);
        }

        Ok(url)
    }

    /// Execute GET request
    pub async fn get(self) -> Result<Response> {
        let url = self.build_url()?;
        let mut request = self.client.client.get(&url);

        // Add authentication headers
        request = self.client.add_auth_headers(request);

        // Add custom headers
        for (key, value) in self.headers {
            request = request.header(key, value);
        }

        let response = request.send().await.map_err(AppleMusicError::Http)?;
        self.client.handle_response(response).await
    }

    /// Execute GET request and parse JSON response
    pub async fn get_json<T: serde::de::DeserializeOwned>(self) -> Result<T> {
        let response = self.get().await?;
        response.json().await.map_err(AppleMusicError::Http)
    }
}

impl HttpClient {
    /// Create a request builder
    pub fn request(&self, path: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new(self, path)
    }
}
