use std::io::{self, Write};
use thirtyfour::prelude::*;


#[tokio::main]
pub async fn scrape(weblink: &str, localhost: &str) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::edge();
    let driver = WebDriver::new(&format!("http://{}:51228", localhost), caps).await?;
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

    driver.quit().await?;

    Ok(())
}
