use crate::{nutrition_information::NutritionInformation, restricted_diet::RestrictedDiet};
use std::time::Duration;

/// RecipeScraper provides method to retrieve information from a recipe.
/// See https://schema.org/Recipe
pub trait RecipeScraper {
    /// author is the author of the recipe.
    fn author(&self) -> Option<String>;

    /// categories are the categories of the recipe, e.g. appetizer, entrée, etc.
    fn categories(&self) -> Option<Vec<String>>;

    /// cook_time is the time it takes to actually cook the dish.
    fn cook_time(&self) -> Option<Duration>;

    /// cuisine is the cuisine of the recipe, e.g. mexican-inspired, french, etc.
    fn cuisine(&self) -> Option<Vec<String>>;

    /// description is the description of the recipe.
    fn description(&self) -> Option<String>;

    /// image_url is a URL to an image of the dish.
    fn image_url(&self) -> Option<String>;

    /// ingredients are all the ingredients used in the recipe.
    fn ingredients(&self) -> Option<Vec<String>>;

    /// instructions are all the steps in making the recipe.
    fn instructions(&self) -> Option<Vec<String>>;

    /// language is the language used in the recipe expressed in IETF BCP 47 standard.
    fn language(&self) -> Option<String>;

    /// name is the name of the recipe.
    fn name(&self) -> Option<String>;

    /// nutrition is nutritional information about the dish.
    fn nutrition(&self) -> Option<NutritionInformation>;

    /// prep_time is the length of time it takes to prepare the items to be used in the instructions.
    fn prep_time(&self) -> Option<Duration>;

    /// suitable_diets indicates dietary restrictions or guidelines for which the recipe is suitable.
    fn suitable_diets(&self) -> Option<Vec<RestrictedDiet>>;

    /// total_time is the total time required to perform the instructions (including the prep time).
    fn total_time(&self) -> Option<Duration>;

    /// yields is the quantity that results from the recipe.
    fn yields(&self) -> Option<String>;
}
