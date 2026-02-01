//! Request and response types for the Pxshot API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

/// Image format for screenshots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    /// PNG format (default).
    #[default]
    Png,
    /// JPEG format.
    Jpeg,
    /// WebP format.
    Webp,
}

/// When to consider the page loaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum WaitUntil {
    /// Wait until the load event fires.
    Load,
    /// Wait until the DOMContentLoaded event fires.
    DomContentLoaded,
    /// Wait until there are no more than 0 network connections for 500ms.
    #[default]
    NetworkIdle,
}

/// Request to capture a screenshot.
#[derive(Debug, Clone, Serialize)]
pub struct ScreenshotRequest {
    /// URL to capture.
    pub url: String,

    /// Image format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,

    /// Image quality (1-100, only for JPEG/WebP).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<u8>,

    /// Viewport width in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    /// Viewport height in pixels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    /// Capture the full scrollable page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_page: Option<bool>,

    /// When to consider navigation succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_until: Option<WaitUntil>,

    /// Wait for a specific CSS selector before capturing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_selector: Option<String>,

    /// Additional wait time in milliseconds after page load.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for_timeout: Option<u32>,

    /// Device scale factor (1-3).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_scale_factor: Option<f32>,

    /// Store the screenshot and return a URL instead of bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,

    /// Block ads and trackers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_ads: Option<bool>,
}

impl ScreenshotRequest {
    /// Create a new builder for a screenshot request.
    pub fn builder() -> ScreenshotRequestBuilder {
        ScreenshotRequestBuilder::default()
    }
}

/// Builder for [`ScreenshotRequest`].
#[derive(Debug, Default)]
pub struct ScreenshotRequestBuilder {
    url: Option<String>,
    format: Option<ImageFormat>,
    quality: Option<u8>,
    width: Option<u32>,
    height: Option<u32>,
    full_page: Option<bool>,
    wait_until: Option<WaitUntil>,
    wait_for_selector: Option<String>,
    wait_for_timeout: Option<u32>,
    device_scale_factor: Option<f32>,
    store: Option<bool>,
    block_ads: Option<bool>,
}

impl ScreenshotRequestBuilder {
    /// Set the URL to capture.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the image format.
    pub fn format(mut self, format: ImageFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// Set the image quality (1-100, only for JPEG/WebP).
    pub fn quality(mut self, quality: u8) -> Self {
        self.quality = Some(quality);
        self
    }

    /// Set the viewport width in pixels.
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the viewport height in pixels.
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Capture the full scrollable page.
    pub fn full_page(mut self, full_page: bool) -> Self {
        self.full_page = Some(full_page);
        self
    }

    /// Set when to consider navigation succeeded.
    pub fn wait_until(mut self, wait_until: WaitUntil) -> Self {
        self.wait_until = Some(wait_until);
        self
    }

    /// Wait for a specific CSS selector before capturing.
    pub fn wait_for_selector(mut self, selector: impl Into<String>) -> Self {
        self.wait_for_selector = Some(selector.into());
        self
    }

    /// Additional wait time in milliseconds after page load.
    pub fn wait_for_timeout(mut self, timeout: u32) -> Self {
        self.wait_for_timeout = Some(timeout);
        self
    }

    /// Set device scale factor (1-3).
    pub fn device_scale_factor(mut self, factor: f32) -> Self {
        self.device_scale_factor = Some(factor);
        self
    }

    /// Store the screenshot and return a URL instead of bytes.
    pub fn store(mut self, store: bool) -> Self {
        self.store = Some(store);
        self
    }

    /// Block ads and trackers.
    pub fn block_ads(mut self, block_ads: bool) -> Self {
        self.block_ads = Some(block_ads);
        self
    }

    /// Build the screenshot request.
    pub fn build(self) -> Result<ScreenshotRequest> {
        let url = self.url.ok_or(Error::MissingField("url"))?;

        Ok(ScreenshotRequest {
            url,
            format: self.format,
            quality: self.quality,
            width: self.width,
            height: self.height,
            full_page: self.full_page,
            wait_until: self.wait_until,
            wait_for_selector: self.wait_for_selector,
            wait_for_timeout: self.wait_for_timeout,
            device_scale_factor: self.device_scale_factor,
            store: self.store,
            block_ads: self.block_ads,
        })
    }
}

/// Response when storing a screenshot (store=true).
#[derive(Debug, Clone, Deserialize)]
pub struct StoredScreenshot {
    /// URL where the screenshot is stored.
    pub url: String,

    /// When the stored screenshot expires.
    pub expires_at: DateTime<Utc>,

    /// Width of the screenshot in pixels.
    pub width: u32,

    /// Height of the screenshot in pixels.
    pub height: u32,

    /// Size of the screenshot in bytes.
    pub size_bytes: u64,
}

/// Result of a screenshot request.
#[derive(Debug)]
pub enum ScreenshotResponse {
    /// Raw image bytes (when store=false).
    Bytes(Vec<u8>),

    /// Stored screenshot info (when store=true).
    Stored(StoredScreenshot),
}

impl ScreenshotResponse {
    /// Get the image bytes if this is a bytes response.
    pub fn bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            Self::Stored(_) => None,
        }
    }

    /// Get the stored screenshot info if this is a stored response.
    pub fn stored(&self) -> Option<&StoredScreenshot> {
        match self {
            Self::Bytes(_) => None,
            Self::Stored(info) => Some(info),
        }
    }

    /// Convert into bytes, returning None if stored.
    pub fn into_bytes(self) -> Option<Vec<u8>> {
        match self {
            Self::Bytes(bytes) => Some(bytes),
            Self::Stored(_) => None,
        }
    }

    /// Convert into stored info, returning None if bytes.
    pub fn into_stored(self) -> Option<StoredScreenshot> {
        match self {
            Self::Bytes(_) => None,
            Self::Stored(info) => Some(info),
        }
    }
}

/// API usage statistics.
#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    /// Number of screenshots taken this billing period.
    pub screenshots: u64,

    /// Total bytes of screenshots taken this billing period.
    pub bytes: u64,

    /// Current billing period start.
    pub period_start: DateTime<Utc>,

    /// Current billing period end.
    pub period_end: DateTime<Utc>,
}

/// API error response.
#[derive(Debug, Deserialize)]
pub(crate) struct ApiError {
    pub error: String,
}
