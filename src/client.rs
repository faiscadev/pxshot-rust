//! Pxshot API client.

use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
    Client, StatusCode,
};

use crate::error::{Error, Result};
use crate::types::{ApiError, ScreenshotRequest, ScreenshotResponse, StoredScreenshot, Usage};

const DEFAULT_BASE_URL: &str = "https://api.pxshot.com";

/// Pxshot API client.
///
/// # Example
///
/// ```no_run
/// use pxshot::{Pxshot, ScreenshotRequest};
///
/// #[tokio::main]
/// async fn main() -> pxshot::Result<()> {
///     let client = Pxshot::new("px_your_api_key");
///
///     let screenshot = client
///         .screenshot(
///             ScreenshotRequest::builder()
///                 .url("https://example.com")
///                 .build()?,
///         )
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Pxshot {
    client: Client,
    base_url: String,
}

impl Pxshot {
    /// Create a new Pxshot client with the given API key.
    ///
    /// # Example
    ///
    /// ```
    /// use pxshot::Pxshot;
    ///
    /// let client = Pxshot::new("px_your_api_key");
    /// ```
    pub fn new(api_key: impl AsRef<str>) -> Self {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }

    /// Create a new Pxshot client with a custom base URL.
    ///
    /// This is primarily useful for testing or self-hosted instances.
    pub fn with_base_url(api_key: impl AsRef<str>, base_url: impl Into<String>) -> Self {
        let mut headers = HeaderMap::new();
        let auth_value = format!("Bearer {}", api_key.as_ref());
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value).expect("invalid API key"),
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("failed to build HTTP client");

        Self {
            client,
            base_url: base_url.into().trim_end_matches('/').to_string(),
        }
    }

    /// Capture a screenshot.
    ///
    /// Returns [`ScreenshotResponse::Bytes`] when `store` is false (default),
    /// or [`ScreenshotResponse::Stored`] when `store` is true.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pxshot::{Pxshot, ScreenshotRequest, ImageFormat};
    ///
    /// #[tokio::main]
    /// async fn main() -> pxshot::Result<()> {
    ///     let client = Pxshot::new("px_your_api_key");
    ///
    ///     // Get screenshot as bytes
    ///     let response = client
    ///         .screenshot(
    ///             ScreenshotRequest::builder()
    ///                 .url("https://example.com")
    ///                 .format(ImageFormat::Png)
    ///                 .width(1920)
    ///                 .height(1080)
    ///                 .build()?,
    ///         )
    ///         .await?;
    ///
    ///     if let Some(bytes) = response.bytes() {
    ///         println!("Got {} bytes", bytes.len());
    ///     }
    ///
    ///     // Get screenshot as stored URL
    ///     let response = client
    ///         .screenshot(
    ///             ScreenshotRequest::builder()
    ///                 .url("https://example.com")
    ///                 .store(true)
    ///                 .build()?,
    ///         )
    ///         .await?;
    ///
    ///     if let Some(stored) = response.stored() {
    ///         println!("Screenshot URL: {}", stored.url);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn screenshot(&self, request: ScreenshotRequest) -> Result<ScreenshotResponse> {
        let store = request.store.unwrap_or(false);
        let url = format!("{}/v1/screenshot", self.base_url);

        let response = self
            .client
            .post(&url)
            .header(CONTENT_TYPE, "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.parse_error(status, response).await);
        }

        if store {
            let stored: StoredScreenshot = response.json().await.map_err(|e| {
                Error::Parse(format!("failed to parse stored screenshot response: {}", e))
            })?;
            Ok(ScreenshotResponse::Stored(stored))
        } else {
            let bytes = response.bytes().await?;
            Ok(ScreenshotResponse::Bytes(bytes.to_vec()))
        }
    }

    /// Get API usage statistics.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pxshot::Pxshot;
    ///
    /// #[tokio::main]
    /// async fn main() -> pxshot::Result<()> {
    ///     let client = Pxshot::new("px_your_api_key");
    ///
    ///     let usage = client.usage().await?;
    ///     println!("Screenshots this period: {}", usage.screenshots);
    ///     println!("Bytes used: {}", usage.bytes);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn usage(&self) -> Result<Usage> {
        let url = format!("{}/v1/usage", self.base_url);

        let response = self.client.get(&url).send().await?;

        let status = response.status();

        if !status.is_success() {
            return Err(self.parse_error(status, response).await);
        }

        response
            .json()
            .await
            .map_err(|e| Error::Parse(format!("failed to parse usage response: {}", e)))
    }

    async fn parse_error(&self, status: StatusCode, response: reqwest::Response) -> Error {
        match response.json::<ApiError>().await {
            Ok(api_error) => Error::Api {
                status: status.as_u16(),
                message: api_error.error,
            },
            Err(_) => Error::Api {
                status: status.as_u16(),
                message: status.canonical_reason().unwrap_or("Unknown error").to_string(),
            },
        }
    }
}

#[cfg(feature = "blocking")]
mod blocking {
    use super::*;

    /// Blocking Pxshot API client.
    ///
    /// This is only available with the `blocking` feature enabled.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use pxshot::blocking::Pxshot;
    /// use pxshot::ScreenshotRequest;
    ///
    /// fn main() -> pxshot::Result<()> {
    ///     let client = Pxshot::new("px_your_api_key");
    ///
    ///     let screenshot = client.screenshot(
    ///         ScreenshotRequest::builder()
    ///             .url("https://example.com")
    ///             .build()?,
    ///     )?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[derive(Debug, Clone)]
    pub struct Pxshot {
        client: reqwest::blocking::Client,
        base_url: String,
    }

    impl Pxshot {
        /// Create a new blocking Pxshot client with the given API key.
        pub fn new(api_key: impl AsRef<str>) -> Self {
            Self::with_base_url(api_key, DEFAULT_BASE_URL)
        }

        /// Create a new blocking Pxshot client with a custom base URL.
        pub fn with_base_url(api_key: impl AsRef<str>, base_url: impl Into<String>) -> Self {
            let mut headers = HeaderMap::new();
            let auth_value = format!("Bearer {}", api_key.as_ref());
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&auth_value).expect("invalid API key"),
            );

            let client = reqwest::blocking::Client::builder()
                .default_headers(headers)
                .build()
                .expect("failed to build HTTP client");

            Self {
                client,
                base_url: base_url.into().trim_end_matches('/').to_string(),
            }
        }

        /// Capture a screenshot (blocking).
        pub fn screenshot(&self, request: ScreenshotRequest) -> Result<ScreenshotResponse> {
            let store = request.store.unwrap_or(false);
            let url = format!("{}/v1/screenshot", self.base_url);

            let response = self
                .client
                .post(&url)
                .header(CONTENT_TYPE, "application/json")
                .json(&request)
                .send()?;

            let status = response.status();

            if !status.is_success() {
                return Err(self.parse_error(status, response));
            }

            if store {
                let stored: StoredScreenshot = response.json().map_err(|e| {
                    Error::Parse(format!("failed to parse stored screenshot response: {}", e))
                })?;
                Ok(ScreenshotResponse::Stored(stored))
            } else {
                let bytes = response.bytes()?;
                Ok(ScreenshotResponse::Bytes(bytes.to_vec()))
            }
        }

        /// Get API usage statistics (blocking).
        pub fn usage(&self) -> Result<Usage> {
            let url = format!("{}/v1/usage", self.base_url);

            let response = self.client.get(&url).send()?;

            let status = response.status();

            if !status.is_success() {
                return Err(self.parse_error(status, response));
            }

            response
                .json()
                .map_err(|e| Error::Parse(format!("failed to parse usage response: {}", e)))
        }

        fn parse_error(&self, status: StatusCode, response: reqwest::blocking::Response) -> Error {
            match response.json::<ApiError>() {
                Ok(api_error) => Error::Api {
                    status: status.as_u16(),
                    message: api_error.error,
                },
                Err(_) => Error::Api {
                    status: status.as_u16(),
                    message: status.canonical_reason().unwrap_or("Unknown error").to_string(),
                },
            }
        }
    }
}

#[cfg(feature = "blocking")]
pub use blocking::Pxshot as BlockingPxshot;
