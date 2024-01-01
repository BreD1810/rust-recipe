use crate::{nutrition_information::NutritionInformation, restricted_diet::RestrictedDiet};
use std::time::Duration;

/// Provides methods to retrieve information from a recipe after it has been scraped.
/// See <https://schema.org/Recipe>
pub trait RecipeInformationProvider {
    /// The author of the recipe.
    fn authors(&self) -> Option<Vec<String>>;

    /// The categories of the recipe, e.g. appetizer, entrée, etc.
    fn categories(&self) -> Option<Vec<String>>;

    /// The time it takes to actually cook the dish.
    fn cook_time(&self) -> Option<Duration>;

    /// The cuisines of the recipe, e.g. mexican-inspired, french, etc.
    fn cuisines(&self) -> Option<Vec<String>>;

    /// The description of the recipe.
    fn description(&self) -> Option<String>;

    /// A URL to an image of the dish.
    fn image_url(&self) -> Option<String>;

    /// All the ingredients used in the recipe.
    fn ingredients(&self) -> Option<Vec<String>>;

    /// All the steps in making the recipe.
    fn instructions(&self) -> Option<Vec<String>>;

    /// The language used in the recipe expressed in IETF BCP 47 standard.
    fn language(&self) -> Option<String>;

    /// The name of the recipe.
    fn name(&self) -> Option<String>;

    /// Nutritional information about the dish.
    fn nutrition(&self) -> Option<NutritionInformation>;

    /// The length of time it takes to prepare the items to be used in the instructions.
    fn prep_time(&self) -> Option<Duration>;

    /// Indicates dietary restrictions or guidelines for which the recipe is suitable.
    fn suitable_diets(&self) -> Option<Vec<RestrictedDiet>>;

    /// The total time required to perform the instructions (including the prep time).
    fn total_time(&self) -> Option<Duration>;

    /// The quantity that results from the recipe.
    fn yields(&self) -> Option<String>;
}
