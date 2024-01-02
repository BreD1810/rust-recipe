use rust_recipe::{custom_scrape_recipe, RecipeInformationProvider, RecipeScraper};
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.bbcgoodfood.com/recipes/crab-lasagne";
    let html = ureq::get(url).call()?.into_string()?;
    let scraper = CustomScraper {};
    let recipe = custom_scrape_recipe(&html, &scraper).unwrap();

    println!("{:?}", scraper);

    println!("Fetching {:?}...\n", url);
    let desc = recipe.description().unwrap();
    println!("Description: {}", desc);
    println!();
    println!("Ingredients:");
    for i in recipe.ingredients().unwrap().iter() {
        println!("- {}", i);
    }

    Ok(())
}

#[derive(Debug)]
pub struct CustomScraper {}

pub struct CustomRecipeInfoProvider {
    vals: HashMap<String, String>,
}

impl RecipeScraper for CustomScraper {
    fn scrape_recipe(
        &self,
        _html: &str,
    ) -> Result<Box<dyn rust_recipe::RecipeInformationProvider>, serde_json::Error> {
        let mut m = HashMap::new();
        m.insert(
            String::from("description"),
            String::from("My favourite recipe"),
        );
        m.insert(
            String::from("ingredients"),
            String::from("carrots, potatoes"),
        );
        Ok(Box::new(CustomRecipeInfoProvider { vals: m }))
    }
}

impl RecipeInformationProvider for CustomRecipeInfoProvider {
    fn authors(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn categories(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn cook_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn cuisines(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn description(&self) -> Option<String> {
        self.vals.get("description").cloned()
    }

    fn image_url(&self) -> Option<String> {
        todo!()
    }

    fn ingredients(&self) -> Option<Vec<String>> {
        self.vals
            .get("ingredients")
            .cloned()
            .map(|s| s.split(", ").map(String::from).collect())
    }

    fn instructions(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn language(&self) -> Option<String> {
        todo!()
    }

    fn name(&self) -> Option<String> {
        todo!()
    }

    fn nutrition(&self) -> Option<rust_recipe::NutritionInformation> {
        todo!()
    }

    fn prep_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn suitable_diets(&self) -> Option<Vec<rust_recipe::RestrictedDiet>> {
        todo!()
    }

    fn total_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn yields(&self) -> Option<String> {
        todo!()
    }
}
