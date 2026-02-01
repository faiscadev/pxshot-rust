//! # Pxshot
//!
//! Official Rust SDK for the [Pxshot](https://pxshot.com) screenshot API.
//!
//! ## Features
//!
//! - **Async-first**: Built on `tokio` and `reqwest` for high-performance async I/O
//! - **Strongly typed**: Full type safety with serde serialization
//! - **Builder pattern**: Ergonomic request construction
//! - **Optional blocking client**: Enable with the `blocking` feature
//!
//! ## Quick Start
//!
//! ```no_run
//! use pxshot::{Pxshot, ScreenshotRequest, ImageFormat};
//!
//! #[tokio::main]
//! async fn main() -> pxshot::Result<()> {
//!     // Create client with your API key
//!     let client = Pxshot::new("px_your_api_key");
//!
//!     // Capture a screenshot as bytes
//!     let response = client
//!         .screenshot(
//!             ScreenshotRequest::builder()
//!                 .url("https://example.com")
//!                 .format(ImageFormat::Png)
//!                 .width(1920)
//!                 .height(1080)
//!                 .build()?,
//!         )
//!         .await?;
//!
//!     // Get the image bytes
//!     let bytes = response.into_bytes().unwrap();
//!     std::fs::write("screenshot.png", bytes)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Storing Screenshots
//!
//! Instead of receiving raw bytes, you can store screenshots and get a URL:
//!
//! ```no_run
//! use pxshot::{Pxshot, ScreenshotRequest};
//!
//! #[tokio::main]
//! async fn main() -> pxshot::Result<()> {
//!     let client = Pxshot::new("px_your_api_key");
//!
//!     let response = client
//!         .screenshot(
//!             ScreenshotRequest::builder()
//!                 .url("https://example.com")
//!                 .store(true)
//!                 .build()?,
//!         )
//!         .await?;
//!
//!     let stored = response.into_stored().unwrap();
//!     println!("Screenshot URL: {}", stored.url);
//!     println!("Expires at: {}", stored.expires_at);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Blocking Client
//!
//! Enable the `blocking` feature in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! pxshot = { version = "0.1", features = ["blocking"] }
//! ```
//!
//! Then use the blocking client:
//!
//! ```ignore
//! use pxshot::blocking::Pxshot;
//! use pxshot::ScreenshotRequest;
//!
//! fn main() -> pxshot::Result<()> {
//!     let client = Pxshot::new("px_your_api_key");
//!
//!     let response = client.screenshot(
//!         ScreenshotRequest::builder()
//!             .url("https://example.com")
//!             .build()?,
//!     )?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Usage Statistics
//!
//! Check your API usage:
//!
//! ```no_run
//! use pxshot::Pxshot;
//!
//! #[tokio::main]
//! async fn main() -> pxshot::Result<()> {
//!     let client = Pxshot::new("px_your_api_key");
//!
//!     let usage = client.usage().await?;
//!     println!("Screenshots this period: {}", usage.screenshots);
//!     println!("Bytes used: {}", usage.bytes);
//!
//!     Ok(())
//! }
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;
mod error;
mod types;

pub use client::Pxshot;
pub use error::{Error, Result};
pub use types::{
    ImageFormat, ScreenshotRequest, ScreenshotRequestBuilder, ScreenshotResponse,
    StoredScreenshot, Usage, WaitUntil,
};

/// Blocking client module (requires `blocking` feature).
#[cfg(feature = "blocking")]
#[cfg_attr(docsrs, doc(cfg(feature = "blocking")))]
pub mod blocking {
    pub use crate::client::BlockingPxshot as Pxshot;
}
