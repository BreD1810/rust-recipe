use scraper::{Html, Selector};
use serde_json::Value;

use crate::recipe_scraper::RecipeScraper;

struct SchemaScraper {
    val: Value,
}

pub fn new_schema_scraper(html: String) -> Result<impl RecipeScraper, serde_json::Error> {
    let sel = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
    let document = Html::parse_document(&html);

    // TODO: Move to function?
    let mut val: Option<Value> = None;
    for e in document.select(&sel) {
        let s = e.text().collect::<String>();
        let v = serde_json::from_str(&s)?;

        if let Value::Object(ref o) = v {
            if o["@type"] == Value::String(String::from("Recipe")) {
                val = Some(v);
                break;
            }
        } else {
            continue;
        }
    }

    match val {
        Some(val) => Ok(SchemaScraper { val }),
        None => panic!("couldn't parse the value"),
    }
}

impl RecipeScraper for SchemaScraper {
    fn author(&self) -> Option<String> {
        todo!()
    }

    fn categories(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn cook_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn cuisine(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn description(&self) -> Option<String> {
        if let Value::String(s) = &self.val["description"] {
            return Some(s.to_string());
        }
        None
    }

    fn image_url(&self) -> Option<String> {
        todo!()
    }

    fn ingredients(&self) -> Option<Vec<String>> {
        if let Value::Array(vals) = &self.val["recipeIngredient"] {
            let ingredients: Vec<String> = vals
                .iter()
                .filter_map(|v| {
                    if let Value::String(s) = v {
                        return Some(s.to_string());
                    }
                    None
                })
                .collect();
            return Some(ingredients);
        }
        None
    }

    fn instructions(&self) -> Option<Vec<String>> {
        todo!()
    }

    fn language(&self) -> Option<String> {
        todo!()
    }

    fn name(&self) -> Option<String> {
        if let Value::String(s) = &self.val["name"] {
            return Some(s.to_string());
        }
        None
    }

    fn nutrition(&self) -> Option<crate::nutrition_information::NutritionInformation> {
        todo!()
    }

    fn prep_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn suitable_diets(&self) -> Option<Vec<crate::restricted_diet::RestrictedDiet>> {
        todo!()
    }

    fn total_time(&self) -> Option<std::time::Duration> {
        todo!()
    }

    fn yields(&self) -> Option<String> {
        todo!()
    }
}
