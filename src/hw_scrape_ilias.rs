use std::io::{self, Write};
use thirtyfour::prelude::*;


#[tokio::main]
pub async fn scrape(weblink: &str, localhost: &str) -> WebDriverResult<()> {
    // Set up Chrome capabilities
    let caps = DesiredCapabilities::edge();

    // Connect to the chromedriver instance running on port 50098
    let driver = WebDriver::new(&format!("http://{}:51228", localhost), caps).await?;

    // Navigate to the provided weblink
    driver.goto(weblink).await?;


    let title = driver.title().await?;
    println!("Page title: {}", title);

    let elements = driver
    .find_all(By::Css(".ilc_link_ExtLink"))
    .await?;
    for element in elements {
        let html = element.inner_html().await?;
        println!("{}", html);
    }

    // Always close the browser session when done
    driver.quit().await?;

    Ok(())
}