use crate::recipe_scraper::RecipeScraper;

struct DefaultScraper {}

pub fn new_default_scraper(html: String) -> DefaultScraper {
    DefaultScraper {}
}

impl RecipeScraper for DefaultScraper {
    fn author(self) -> Option<String> {
        todo!()
    }

    fn categories(self) -> Option<Vec<String>> {
        todo!()
    }

    fn cook_time(self) -> Option<std::time::Duration> {
        todo!()
    }

    fn cuisine(self) -> Option<Vec<String>> {
        todo!()
    }

    fn description(self) -> Option<String> {
        todo!()
    }

    fn image_url(self) -> Option<String> {
        todo!()
    }

    fn ingredients(self) -> Option<Vec<String>> {
        todo!()
    }

    fn instructions(self) -> Option<Vec<String>> {
        todo!()
    }

    fn language(self) -> Option<String> {
        todo!()
    }

    fn name(self) -> Option<String> {
        todo!()
    }

    fn nutrition(self) -> Option<crate::nutrition_information::NutritionInformation> {
        todo!()
    }

    fn prep_time(self) -> Option<std::time::Duration> {
        todo!()
    }

    fn suitable_diets(self) -> Option<Vec<crate::restricted_diet::RestrictedDiet>> {
        todo!()
    }

    fn total_time(self) -> Option<std::time::Duration> {
        todo!()
    }

    fn yields(self) -> Option<String> {
        todo!()
    }
}
