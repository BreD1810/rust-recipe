/// Nutritional Information
/// See https://schema.org/NutritionInformation
pub struct NutritionInformation {
    /// The number of calories.
    calories: f32,

    /// The number of grams of carbohydrates.
    carbohydrate_grams: f32,

    /// The number of milligrams of cholesterol.
    cholesterol_milligrams: f32,

    /// The number of grams of fat.
    fat_grams: f32,

    /// The number of grams of fiber.
    fiber_grams: f32,

    /// The number of grams of protein.
    protein_grams: f32,

    /// The number of grams of saturated fat.
    saturated_fat_grams: f32,

    /// The serving size, in terms of the number of volume or mass.
    serving_size: String,

    /// The number of milligrams of sodium.
    sodium_milligrams: f32,

    /// The number of grams of sugar.
    sugar_grams: f32,

    /// The number of grams of trans fat.
    trans_fat_grams: f32,

    /// The number of grams of unsaturated fat.
    unsaturated_fat_grams: f32,
}
