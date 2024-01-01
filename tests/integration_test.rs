use std::{fs, path::PathBuf, time::Duration};

use rust_recipe::NutritionInformation;

#[test]
fn test_default_scraper_bbc_crab_lasagne() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/crab-lasagne.html");
    let html = fs::read_to_string(path).expect("couldn't read test data");
    let res = rust_recipe::scrape_recipe(&html);

    assert!(res.is_ok());
    let recipe = res.unwrap();

    assert_eq!(Some(Duration::from_secs(40 * 60)), recipe.cook_time());
    assert!(recipe.language().is_none());
    assert_eq!(Some(String::from("Crab lasagne")), recipe.name());
    assert_eq!(Some(Duration::from_secs(30 * 60)), recipe.prep_time());
    assert!(recipe.suitable_diets().is_none());
    assert_eq!(Some(Duration::from_secs(70 * 60)), recipe.total_time());
    assert_eq!(
        Some(String::from("Makes 2, each serves 4")),
        recipe.yields()
    );

    let authors = vec![String::from("Sara Buenfeld")];
    assert_eq!(Some(authors), recipe.authors());

    let categories = vec![String::from("Main course")];
    assert_eq!(Some(categories), recipe.categories());

    let cuisines = vec![String::from("Italian")];
    assert_eq!(Some(cuisines), recipe.cuisines());

    let description = String::from("A creamy pasta bake with crab, cod and leeks that can be made ahead and frozen, then baked for an occasion");
    assert_eq!(Some(description), recipe.description());

    let image_url = String::from("https://images.immediate.co.uk/production/volatile/sites/30/2020/08/crab-lasagne-5556e42.jpg?resize=768,574");
    assert_eq!(Some(image_url), recipe.image_url());

    let ingredients = vec![
        String::from("600ml milk"),
        String::from("¼ tsp ground nutmeg"),
        String::from("85g butter , plus extra for greasing"),
        String::from("50g plain flour"),
        String::from("227ml pot double cream"),
        String::from("100g parmesan , grated"),
        String::from("1 egg , beaten"),
        String::from("3 x 100g pots brown and white crabmeat"),
        String::from("2 x 500g packs trimmed  leek , well washed and finely sliced"),
        String::from("12 sheets fresh lasagne"),
        String::from("400g chunky skinless cod fillet, cubed"),
    ];
    assert_eq!(Some(ingredients), recipe.ingredients());

    let instructions = vec![
        String::from("Tip the milk, nutmeg, 50g of the butter and the flour into a non-stick pan. Heat while whisking or stirring until thickened. Remove from the heat and stir in the cream with plenty of seasoning. Measure 450ml of the sauce and beat half the Parmesan and the egg into it. Once cool, stir the crabmeat into the remaining sauce."),
        String::from("Melt the remaining butter in a very large pan or wok, add the leeks and cook for about 10 mins, stirring frequently, until softened and cooked down."),
        String::from("Lightly grease 2 x 20cm square ovenproof dishes – trim the lasagne sheets to fit, if you need to."),
        String::from("Spoon a quarter of the leeks into each dish and spread out. Top each with 2 lasagne sheets, then cover with the crab mixture and scatter over the chunks of fish. Top each with another 2 sheets, then cover with the leeks and the last of the sheets followed by the cheese sauce, then the remaining Parmesan."),
        String::from("To freeze: Wrap the dishes in several layers of cling film, then foil. Use within 2 months. Thaw overnight in the fridge. Heat oven to 180C/160C fan/gas 4. Unwrap the lasagnes and bake for 40 mins. (A freshly made version will take the same time.) Serve with a salad."),
    ];
    assert_eq!(Some(instructions), recipe.instructions());

    let mut ni = NutritionInformation::default();
    ni.calories = Some(612_f32);
    ni.carbohydrate_grams = Some(42_f32);
    ni.fat_grams = Some(34_f32);
    ni.fiber_grams = Some(8_f32);
    ni.protein_grams = Some(34_f32);
    ni.saturated_fat_grams = Some(19_f32);
    ni.sodium_milligrams = Some(1_f32);
    ni.sugar_grams = Some(10_f32);
    assert_eq!(Some(612_f32), ni.calories);
    assert_eq!(Some(42_f32), ni.carbohydrate_grams);
    assert_eq!(Some(34_f32), ni.fat_grams);
    assert_eq!(Some(8_f32), ni.fiber_grams);
    assert_eq!(Some(34_f32), ni.protein_grams);
    assert_eq!(Some(19_f32), ni.saturated_fat_grams);
    assert_eq!(Some(1_f32), ni.sodium_milligrams);
    assert_eq!(Some(10_f32), ni.sugar_grams);
}
