use rust_recipe::scrape_recipe_from_url;
extern crate tokio;

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
