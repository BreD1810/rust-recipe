pub use nutrition_information::NutritionInformation;
pub use recipe_scraper::RecipeScraper;
pub use restricted_diet::RestrictedDiet;

use std::error::Error;
mod default_scraper;
pub mod nutrition_information;
pub mod recipe_scraper;
pub mod restricted_diet;

/// scrape_recipe takes some HTML and attempts to parse a respice to a RecipeScraper
pub fn scrape_recipe(html: &str) -> Result<impl RecipeScraper, Box<dyn Error>> {
    // TODO: Figure out custom scrapers here
    let scraper = default_scraper::new_schema_scraper(html)?;
    Ok(scraper)
}

/// scrape_recipe_from_url_blocking uses ureq to scrape a recipe from a URL
#[cfg(feature = "blocking")]
pub fn scrape_recipe_from_url_blocking(url: &str) -> Result<impl RecipeScraper, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;
    scrape_recipe(&res)
}

/// scrape_recipe_from_url uses reqwest to scrape a recipe from a URL
#[cfg(feature = "async")]
pub async fn scrape_recipe_from_url(url: &str) -> Result<impl RecipeScraper, Box<dyn Error>> {
    let res = reqwest::get(url).await?.text().await?;
    scrape_recipe(&res)
}
