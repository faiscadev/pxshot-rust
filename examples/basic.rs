//! Basic example showing screenshot capture.

use pxshot::{ImageFormat, Pxshot, ScreenshotRequest};
use std::env;

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    // Get API key from environment
    let api_key = env::var("PXSHOT_API_KEY").expect("PXSHOT_API_KEY environment variable required");

    // Create client
    let client = Pxshot::new(&api_key);

    // Capture a screenshot as bytes
    println!("Capturing screenshot...");
    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .format(ImageFormat::Png)
                .width(1280)
                .height(720)
                .build()?,
        )
        .await?;

    // Save to file
    if let Some(bytes) = response.bytes() {
        std::fs::write("screenshot.png", bytes)?;
        println!("Saved screenshot.png ({} bytes)", bytes.len());
    }

    // Now capture with storage
    println!("\nCapturing with storage...");
    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://example.com")
                .store(true)
                .build()?,
        )
        .await?;

    if let Some(stored) = response.stored() {
        println!("Screenshot URL: {}", stored.url);
        println!("Dimensions: {}x{}", stored.width, stored.height);
        println!("Size: {} bytes", stored.size_bytes);
        println!("Expires at: {}", stored.expires_at);
    }

    // Check usage
    println!("\nChecking usage...");
    let usage = client.usage().await?;
    println!("Screenshots this period: {}", usage.screenshots);
    println!("Bytes used: {}", usage.bytes);

    Ok(())
}
