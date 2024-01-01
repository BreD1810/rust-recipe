# rust-recipe

rust-recipe is a Rust crate that scrapes recipes from webites.
It is inspired by the Golang library "[go-recipe](https://github.com/kkyr/go-recipe)".

## Adding to your Project
```shell
cargo add rust-recipe
```

Optionally, you can use the `blocking` or `async` features.

## Usage

### Custom Scraping
By default, the crate provides the `scrape_recipe` method, which takes in HTML you have scraped from the website and attempts to parse it.
The `RecipeInformationProvider` trait provides the methods available to fetch information once scraped.

```{.rust .ignore}
use rust_recipe::scrape_recipe;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.bbcgoodfood.com/recipes/crab-lasagne";
    let html = ureq::get(url).call()?.into_string()?;
    let recipe = scrape_recipe(&html).unwrap();

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
```

Custom scrapers can also be used by implementing the `RecipeScraper` trait.

```{.rust .ignore}
use rust_recipe::{custom_scrape_recipe, RecipeInformationProvider, RecipeScraper};
use std::{collections::HashMap, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://www.bbcgoodfood.com/recipes/crab-lasagne";
    let html = ureq::get(url).call()?.into_string()?;
    let scraper = CustomScraper {};
    let recipe = custom_scrape_recipe(&html, scraper).unwrap();

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

pub struct CustomScraper {...}

pub struct CustomRecipeInfoProvider {
    vals: HashMap<String, String>,
}

impl RecipeScraper for CustomScraper {
    fn scrape_recipe(
        self,
        html: &str,
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
        ...
        Ok(Box::new(CustomRecipeInfoProvider { vals: m }))
    }
}

impl RecipeInformationProvider for CustomRecipeInfoProvider {
    ...
    fn description(&self) -> Option<String> {
        self.vals.get("description").cloned()
    }

    fn ingredients(&self) -> Option<Vec<String>> {
        self.vals
            .get("ingredients")
            .cloned()
            .map(|s| s.split(", ").map(String::from).collect())
    }
    ...
}
```

### Async 
The `async` feature uses the `reqwest` to make an async call to the URL provided:

```{.rust .ignore}
``use rust_recipe::scrape_recipe_from_url;

#[tokio::main]
async fn main() {
    let url = "https://www.bbcgoodfood.com/recipes/crab-lasagne";
    println!("Fetching {:?}...\n", url);
    let recipe = scrape_recipe_from_url(url).await.unwrap();

    let desc = recipe.description().unwrap();
    println!("Description: {}", desc);
    println!();
    println!("Ingredients:");
    for i in recipe.ingredients().unwrap().iter() {
        println!("- {}", i);
    }
}
```

### Blocking
The `blocking` feature uses the `ureq` crate to make a blocking call to the URL provided.

```{.rust .ignore}
use rust_recipe::{scrape_recipe_from_url_blocking, RecipeScraper};

fn main() {
    let url = "https://www.bbcgoodfood.com/recipes/crab-lasagne";
    println!("Fetching {:?}...\n", url);
    let recipe = scrape_recipe_from_url_blocking(url).unwrap();

    let desc = recipe.description().unwrap();
    println!("Description: {}", desc);
    println!();
    println!("Ingredients:");
    for i in recipe.ingredients().unwrap().iter() {
        println!("- {}", i);
    }
}
```
