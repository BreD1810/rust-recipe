use rust_recipe::scrape_recipe_from_url_blocking;

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
