#![doc = include_str!("../README.md")]
#[doc(inline)]
pub use nutrition_information::NutritionInformation;
#[doc(inline)]
pub use recipe_information_provider::RecipeInformationProvider;
#[doc(inline)]
pub use restricted_diet::RestrictedDiet;
use schema_scraper::SchemaScraper;
use std::error::Error;

pub mod nutrition_information;
pub mod recipe_information_provider;
pub mod restricted_diet;
mod schema_scraper;

/// Provides methods to scrape HTML for recipe information.
pub trait RecipeScraper {
    fn scrape_recipe(
        &self,
        html: &str,
    ) -> Result<Box<dyn RecipeInformationProvider>, serde_json::Error>;
}

/// Takes some HTML and attempts to parse a recipe to a RecipeInformationProvider
pub fn scrape_recipe(html: &str) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    let scraper = SchemaScraper {};
    custom_scrape_recipe(html, &scraper)
}

/// Uses a custom scraper to retrieve a recipe
/// # Arguments
/// - `html` - The HTML of the recipe page.
/// - `scraper` - The custom scraper to use.
pub fn custom_scrape_recipe(
    html: &str,
    scraper: &impl RecipeScraper,
) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    Ok(scraper.scrape_recipe(html)?)
}

/// Uses ureq to scrape a recipe from a URL
#[cfg(feature = "blocking")]
pub fn scrape_recipe_from_url_blocking(
    url: &str,
) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;
    scrape_recipe(&res)
}

/// Uses ureq to scrape a recipe from a URL using a custom scraper
#[cfg(feature = "blocking")]
pub fn custom_scrape_recipe_from_url_blocking(
    url: &str,
    scraper: &impl RecipeScraper,
) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    let res = ureq::get(url).call()?.into_string()?;
    custom_scrape_recipe(&res, scraper)
}

/// Uses reqwest to scrape a recipe from a URL
#[cfg(feature = "async")]
pub async fn scrape_recipe_from_url(
    url: &str,
) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    let res = reqwest::get(url).await?.text().await?;
    scrape_recipe(&res)
}

/// Uses reqwest to scrape a recipe from a URL
#[cfg(feature = "async")]
pub async fn custom_scrape_recipe_from_url(
    url: &str,
    scraper: &impl RecipeScraper,
) -> Result<Box<dyn RecipeInformationProvider>, Box<dyn Error>> {
    let res = reqwest::get(url).await?.text().await?;
    custom_scrape_recipe(&res, scraper)
}
