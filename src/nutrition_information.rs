/// Nutritional Information
/// See <https://schema.org/NutritionInformation>
pub struct NutritionInformation {
    /// The number of calories.
    pub calories: f32,

    /// The number of grams of carbohydrates.
    pub carbohydrate_grams: f32,

    /// The number of milligrams of cholesterol.
    pub cholesterol_milligrams: f32,

    /// The number of grams of fat.
    pub fat_grams: f32,

    /// The number of grams of fiber.
    pub fiber_grams: f32,

    /// The number of grams of protein.
    pub protein_grams: f32,

    /// The number of grams of saturated fat.
    pub saturated_fat_grams: f32,

    /// The serving size, in terms of the number of volume or mass.
    pub serving_size: String,

    /// The number of milligrams of sodium.
    pub sodium_milligrams: f32,

    /// The number of grams of sugar.
    pub sugar_grams: f32,

    /// The number of grams of trans fat.
    pub trans_fat_grams: f32,

    /// The number of grams of unsaturated fat.
    pub unsaturated_fat_grams: f32,
}
