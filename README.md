# rust-recipe

rust-recipe is a Rust crate that scrapes recipes from webites.
It is inspired by the Golang library "[go-recipe](https://github.com/kkyr/go-recipe)".

## Adding to your Project
```shell
cargo add rust-recipe
```

Optionally, you can use the `blocking` or `async` features.

## Usage

### Custom
By default, the crate provides the `scrape_recipe` method, which takes in the HTML of the website and attempts to parse it.
The `RecipeScraper` trait provides the methods available to fetch information once scraped.

```rust
use rust_recipe::{scrape_recipe, RecipeScraper};
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

### Async
The `async` feature uses the `reqwest` to make an async call to the URL provided:

```rust
use rust_recipe::{scrape_recipe_from_url, RecipeScraper};

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

```rust
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
