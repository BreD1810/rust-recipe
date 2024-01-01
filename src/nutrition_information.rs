use once_cell::sync::Lazy;
use regex::Regex;

/// Nutritional Information from a recipe.
/// See <https://schema.org/NutritionInformation>
#[derive(Default, Debug, PartialEq)]
pub struct NutritionInformation {
    /// The number of calories.
    pub calories: Option<f32>,

    /// The number of grams of carbohydrates.
    pub carbohydrate_grams: Option<f32>,

    /// The number of milligrams of cholesterol.
    pub cholesterol_milligrams: Option<f32>,

    /// The number of grams of fat.
    pub fat_grams: Option<f32>,

    /// The number of grams of fiber.
    pub fiber_grams: Option<f32>,

    /// The number of grams of protein.
    pub protein_grams: Option<f32>,

    /// The number of grams of saturated fat.
    pub saturated_fat_grams: Option<f32>,

    /// The serving size, in terms of the number of volume or mass.
    pub serving_size: Option<String>,

    /// The number of milligrams of sodium.
    pub sodium_milligrams: Option<f32>,

    /// The number of grams of sugar.
    pub sugar_grams: Option<f32>,

    /// The number of grams of trans fat.
    pub trans_fat_grams: Option<f32>,

    /// The number of grams of unsaturated fat.
    pub unsaturated_fat_grams: Option<f32>,
}

impl NutritionInformation {
    pub fn add_info(&mut self, field_name: &str, value: &str) {
        let numeric_value = get_first_number_from_str(value);
        match field_name {
            "calories" => self.calories = numeric_value,
            "carbohydrateContent" => self.carbohydrate_grams = numeric_value,
            "cholesterolContent" => self.cholesterol_milligrams = numeric_value,
            "fatContent" => self.fat_grams = numeric_value,
            "fiberContent" => self.fiber_grams = numeric_value,
            "proteinContent" => self.protein_grams = numeric_value,
            "saturatedFatContent" => self.saturated_fat_grams = numeric_value,
            "servingSize" => self.serving_size = Some(value.to_string()),
            "sodiumContent" => self.sodium_milligrams = numeric_value,
            "sugarContent" => self.sugar_grams = numeric_value,
            "transFatContent" => self.trans_fat_grams = numeric_value,
            "unsaturatedFatContent" => self.unsaturated_fat_grams = numeric_value,
            _ => (),
        }
    }
}

fn get_first_number_from_str(s: &str) -> Option<f32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(\d*[.])?\d+"#).unwrap());
    let captures = RE.captures(s);
    match captures {
        Some(caps) => caps[0].parse().ok(),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_info_success() {
        let mut ni = NutritionInformation::default();
        ni.add_info("calories", "200 calories");
        ni.add_info("carbohydrateContent", "1 carbs");
        ni.add_info("cholesterolContent", "2 cholesterol");
        ni.add_info("fatContent", "3 fat");
        ni.add_info("fiberContent", "4 fiber");
        ni.add_info("proteinContent", "5 protein");
        ni.add_info("saturatedFatContent", "6 fat");
        ni.add_info("servingSize", "7 servings");
        ni.add_info("sodiumContent", "8 sodium");
        ni.add_info("sugarContent", "9 sugar");
        ni.add_info("transFatContent", "10 fat");
        ni.add_info("unsaturatedFatContent", "11 fat");

        assert_eq!(Some(200_f32), ni.calories);
        assert_eq!(Some(1_f32), ni.carbohydrate_grams);
        assert_eq!(Some(2_f32), ni.cholesterol_milligrams);
        assert_eq!(Some(3_f32), ni.fat_grams);
        assert_eq!(Some(4_f32), ni.fiber_grams);
        assert_eq!(Some(5_f32), ni.protein_grams);
        assert_eq!(Some(6_f32), ni.saturated_fat_grams);
        assert_eq!(Some(String::from("7 servings")), ni.serving_size);
        assert_eq!(Some(8_f32), ni.sodium_milligrams);
        assert_eq!(Some(9_f32), ni.sugar_grams);
        assert_eq!(Some(10_f32), ni.trans_fat_grams);
        assert_eq!(Some(11_f32), ni.unsaturated_fat_grams);
    }

    #[test]
    fn test_add_info_failure() {
        let mut ni = NutritionInformation::default();
        ni.add_info("not valid", "100 of something");
        assert_eq!(NutritionInformation::default(), ni);
    }

    #[test]
    fn test_get_first_number_from_str_success_int() {
        let res = get_first_number_from_str("About 200 calories");
        assert_eq!(Some(200_f32), res)
    }

    #[test]
    fn test_get_first_number_from_str_success_float() {
        let res = get_first_number_from_str("200.1 calories");
        assert_eq!(Some(200.1), res)
    }

    #[test]
    fn test_get_first_number_from_str_failure() {
        let res = get_first_number_from_str("some calories");
        assert_eq!(None, res)
    }
}
