use std::io::{self, Write};


use thirtyfour::prelude::*;

#[tokio::main]
pub async fn scrape() -> WebDriverResult<()> {
    // Set up Chrome capabilities
    let caps = DesiredCapabilities::edge();

    // Connect to the chromedriver instance running on port 50098
    let driver = WebDriver::new("http://localhost:50098", caps).await?;

    // Navigate to example.com
    driver.goto("https://example.com").await?;

    // Grab the page title as a sanity check
    let title = driver.title().await?;
    println!("Page title: {}", title);

    // Grab the page source (the actual rendered HTML)
    let html = driver.source().await?;
    println!("{}", html);

    // Always close the browser session when done
    driver.quit().await?;

    Ok(())
}