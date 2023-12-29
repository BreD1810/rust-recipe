pub use nutrition_information::NutritionInformation;
pub use recipe_scraper::RecipeScraper;
pub use restricted_diet::RestrictedDiet;
use std::error::Error;
mod default_scraper;
pub mod nutrition_information;
pub mod recipe_scraper;
pub mod restricted_diet;

pub fn scrape_recipe(url: String) -> Result<impl RecipeScraper, Box<dyn Error>> {
    // TODO: Figure out custom scrapers here
    let res = reqwest::blocking::get(url)?.text()?;
    let scraper = default_scraper::new_schema_scraper(res)?;
    Ok(scraper)
}
