//! Example showing full page screenshot with wait options.

use pxshot::{Pxshot, ScreenshotRequest, WaitUntil};
use std::env;

#[tokio::main]
async fn main() -> pxshot::Result<()> {
    let api_key = env::var("PXSHOT_API_KEY").expect("PXSHOT_API_KEY environment variable required");

    let client = Pxshot::new(&api_key);

    println!("Capturing full page screenshot...");
    let response = client
        .screenshot(
            ScreenshotRequest::builder()
                .url("https://en.wikipedia.org/wiki/Rust_(programming_language)")
                .full_page(true)
                .wait_until(WaitUntil::NetworkIdle)
                .wait_for_timeout(500) // Extra 500ms for any lazy-loaded content
                .device_scale_factor(2.0) // Retina quality
                .build()?,
        )
        .await?;

    if let Some(bytes) = response.bytes() {
        std::fs::write("full_page.png", bytes)?;
        println!("Saved full_page.png ({} bytes)", bytes.len());
    }

    Ok(())
}
