use scraper::{Html, Selector};
use serde_json::Value;

use crate::{NutritionInformation, RecipeInformationProvider, RecipeScraper, RestrictedDiet};

pub struct SchemaScraper {}

pub struct SchemaScraperInfoProvider {
    val: Value,
}

impl RecipeScraper for SchemaScraper {
    fn scrape_recipe(
        &self,
        html: &str,
    ) -> Result<Box<dyn RecipeInformationProvider>, serde_json::Error> {
        let sel = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();
        let document = Html::parse_document(html);

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
            Some(val) => Ok(Box::new(SchemaScraperInfoProvider { val })),
            None => panic!("couldn't parse the value"),
        }
    }
}

impl RecipeInformationProvider for SchemaScraperInfoProvider {
    fn authors(&self) -> Option<Vec<String>> {
        if let Value::Array(authors) = &self.val["author"] {
            let authors: Vec<String> = authors
                .iter()
                .filter_map(|author| {
                    if let Value::Object(o) = author {
                        match o.get("name") {
                            Some(Value::String(name)) => return Some(name.to_string()),
                            _ => return None,
                        }
                    }
                    None
                })
                .collect();
            return Some(authors);
        }
        None
    }

    fn categories(&self) -> Option<Vec<String>> {
        if let Value::String(s) = &self.val["recipeCategory"] {
            return Some(s.split(", ").map(String::from).collect());
        }
        None
    }

    fn cook_time(&self) -> Option<std::time::Duration> {
        if let Value::String(s) = &self.val["cookTime"] {
            match s.parse::<iso8601_duration::Duration>() {
                Ok(v) => return v.to_std(),
                _ => return None,
            };
        }
        None
    }

    fn cuisines(&self) -> Option<Vec<String>> {
        if let Value::String(s) = &self.val["recipeCuisine"] {
            return Some(s.split(", ").map(String::from).collect());
        }
        None
    }

    fn description(&self) -> Option<String> {
        if let Value::String(s) = &self.val["description"] {
            return Some(s.to_string());
        }
        None
    }

    fn image_url(&self) -> Option<String> {
        if let Value::Object(o) = &self.val["image"] {
            match o.get("url") {
                Some(Value::String(url)) => return Some(url.to_string()),
                _ => return None,
            }
        }
        None
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
        if let Value::Array(ins) = &self.val["recipeInstructions"] {
            let instructions: Vec<String> = ins
                .iter()
                .filter_map(|cur_in| match cur_in {
                    Value::String(s) => Some(s.to_string()),
                    Value::Object(obj) => {
                        if let Some(Value::String(s)) = obj.get("@type") {
                            if s == "HowToStep" {
                                return match obj.get("text") {
                                    Some(Value::String(text)) => return Some(text.to_string()),
                                    _ => None,
                                };
                            }
                            None
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect();
            return Some(instructions);
        }
        None
    }

    fn language(&self) -> Option<String> {
        match &self.val["inLanguage"] {
            Value::String(s) => Some(s.to_string()),
            Value::Object(obj) => {
                if let Some(Value::String(t)) = obj.get("@type") {
                    if t == "Language" {
                        return match obj.get("text") {
                            Some(Value::String(text)) => return Some(text.to_string()),
                            _ => None,
                        };
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn name(&self) -> Option<String> {
        if let Value::String(s) = &self.val["name"] {
            return Some(s.to_string());
        }
        None
    }

    fn nutrition(&self) -> Option<NutritionInformation> {
        if let Value::Object(o) = &self.val["nutrition"] {
            let mut nutrition_info = NutritionInformation::default();
            for (field_name, val) in o {
                if let Value::String(amount) = val {
                    nutrition_info.add_info(field_name, amount);
                }
            }
            return Some(nutrition_info);
        }
        None
    }

    fn prep_time(&self) -> Option<std::time::Duration> {
        if let Value::String(s) = &self.val["prepTime"] {
            match s.parse::<iso8601_duration::Duration>() {
                Ok(v) => return v.to_std(),
                _ => return None,
            };
        }
        None
    }

    fn suitable_diets(&self) -> Option<Vec<RestrictedDiet>> {
        if let Value::String(s) = &self.val["suitableForDiet"] {
            return s
                .split(", ")
                .map(|diet| diet.parse::<RestrictedDiet>().ok())
                .collect();
        }
        None
    }

    fn total_time(&self) -> Option<std::time::Duration> {
        if let Value::String(s) = &self.val["totalTime"] {
            match s.parse::<iso8601_duration::Duration>() {
                Ok(v) => return v.to_std(),
                _ => return None,
            };
        }
        None
    }

    fn yields(&self) -> Option<String> {
        if let Value::String(s) = &self.val["recipeYield"] {
            return Some(s.to_string());
        }
        None
    }
}
