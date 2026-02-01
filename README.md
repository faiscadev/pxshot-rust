# pxshot

[![Crates.io](https://img.shields.io/crates/v/pxshot.svg)](https://crates.io/crates/pxshot)
[![Documentation](https://docs.rs/pxshot/badge.svg)](https://docs.rs/pxshot)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Official Rust SDK for the [Pxshot](https://pxshot.com) screenshot API.

## Features

- **Async-first**: Built on `tokio` and `reqwest` for high-performance async I/O
- **Strongly typed**: Full type safety with serde serialization
- **Builder pattern**: Ergonomic request construction
- **Optional blocking client**: Enable with the `blocking` feature

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pxshot = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

## Quick Start

```rust
use pxshot::{Pxshot, ScreenshotRequest, ImageFormat};

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    // Create client with your API key
    let client = Pxshot::new("px_your_api_key");

    // Capture a screenshot as bytes
    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .format(ImageFormat::Png)
                .width(1920)
                .height(1080)
                .build()?,
        )
        .await?;

    // Get the image bytes
    let bytes = response.into_bytes().unwrap();
    std::fs::write("screenshot.png", bytes)?;

    Ok(())
}
```

## Storing Screenshots

Instead of receiving raw bytes, you can store screenshots and get a URL:

```rust
use pxshot::{Pxshot, ScreenshotRequest};

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    let client = Pxshot::new("px_your_api_key");

    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .store(true)
                .build()?,
        )
        .await?;

    let stored = response.into_stored().unwrap();
    println!("Screenshot URL: {}", stored.url);
    println!("Expires at: {}", stored.expires_at);
    println!("Dimensions: {}x{}", stored.width, stored.height);

    Ok(())
}
```

## Full Page Screenshots

Capture the entire scrollable page:

```rust
use pxshot::{Pxshot, ScreenshotRequest};

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    let client = Pxshot::new("px_your_api_key");

    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .full_page(true)
                .build()?,
        )
        .await?;

    Ok(())
}
```

## Wait for Content

Wait for specific elements or additional load time:

```rust
use pxshot::{Pxshot, ScreenshotRequest, WaitUntil};

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    let client = Pxshot::new("px_your_api_key");

    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .wait_until(WaitUntil::NetworkIdle)
                .wait_for_selector("#main-content")
                .wait_for_timeout(1000) // Additional 1s wait
                .build()?,
        )
        .await?;

    Ok(())
}
```

## Usage Statistics

Check your API usage:

```rust
use pxshot::Pxshot;

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    let client = Pxshot::new("px_your_api_key");

    let usage = client.usage().await?;
    println!("Screenshots this period: {}", usage.screenshots);
    println!("Bytes used: {}", usage.bytes);
    println!("Period: {} to {}", usage.period_start, usage.period_end);

    Ok(())
}
```

## Blocking Client

For non-async contexts, enable the `blocking` feature:

```toml
[dependencies]
pxshot = { version = "0.1", features = ["blocking"] }
```

```rust
use pxshot::blocking::Pxshot;
use pxshot::ScreenshotRequest;

fn main() -> pxshot::Result<()> {
    let client = Pxshot::new("px_your_api_key");

    let response = client.screenshot(
        ScreenshotRequest::builder()
            .url("https://example.com")
            .build()?,
    )?;

    if let Some(bytes) = response.bytes() {
        std::fs::write("screenshot.png", bytes)?;
    }

    Ok(())
}
```

## API Reference

### ScreenshotRequest Options

| Option | Type | Description |
|--------|------|-------------|
| `url` | `String` | **Required.** URL to capture |
| `format` | `ImageFormat` | `Png`, `Jpeg`, or `Webp` (default: `Png`) |
| `quality` | `u8` | Image quality 1-100 (JPEG/WebP only) |
| `width` | `u32` | Viewport width in pixels |
| `height` | `u32` | Viewport height in pixels |
| `full_page` | `bool` | Capture full scrollable page |
| `wait_until` | `WaitUntil` | `Load`, `DomContentLoaded`, or `NetworkIdle` |
| `wait_for_selector` | `String` | Wait for CSS selector |
| `wait_for_timeout` | `u32` | Additional wait time in ms |
| `device_scale_factor` | `f32` | Device pixel ratio (1-3) |
| `store` | `bool` | Return URL instead of bytes |
| `block_ads` | `bool` | Block ads and trackers |

## Error Handling

The SDK uses a custom `Error` type with detailed error variants:

```rust
use pxshot::{Pxshot, ScreenshotRequest, Error};

#[tokio::main]
async fn main() {
    let client = Pxshot::new("px_your_api_key");

    let result = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .build()
                .unwrap(),
        )
        .await;

    match result {
        Ok(response) => println!("Success!"),
        Err(Error::Api { status, message }) => {
            eprintln!("API error {}: {}", status, message);
        }
        Err(Error::Request(e)) => {
            eprintln!("Network error: {}", e);
        }
        Err(e) => eprintln!("Other error: {}", e),
    }
}
```

## License

MIT License - see [LICENSE](LICENSE) for details.
