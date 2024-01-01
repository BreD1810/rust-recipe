use core::fmt;
use std::str::FromStr;

/// The type of dietary restrictions that a recipe caters for.
/// See <https://schema.org/RestrictedDiet>
#[derive(Debug, PartialEq)]
pub enum RestrictedDiet {
    UnknownDiet,
    DiabeticDiet,
    GlutenFreeDiet,
    HalalDiet,
    HinduDiet,
    KosherDiet,
    LowCalorieDiet,
    LowFatDiet,
    LowLactoseDiet,
    LowSaltDiet,
    VeganDiet,
    VegetarianDiet,
}

#[derive(Debug)]
pub struct RestrictedDietError;

impl FromStr for RestrictedDiet {
    type Err = RestrictedDietError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UnknownDiet" => Ok(RestrictedDiet::UnknownDiet),
            "DiabeticDiet" => Ok(RestrictedDiet::DiabeticDiet),
            "GlutenFreeDiet" => Ok(RestrictedDiet::GlutenFreeDiet),
            "HalalDiet" => Ok(RestrictedDiet::HalalDiet),
            "HinduDiet" => Ok(RestrictedDiet::HinduDiet),
            "KosherDiet" => Ok(RestrictedDiet::KosherDiet),
            "LowCalorieDiet" => Ok(RestrictedDiet::LowCalorieDiet),
            "LowFatDiet" => Ok(RestrictedDiet::LowFatDiet),
            "LowLactoseDiet" => Ok(RestrictedDiet::LowLactoseDiet),
            "LowSaltDiet" => Ok(RestrictedDiet::LowSaltDiet),
            "VeganDiet" => Ok(RestrictedDiet::VeganDiet),
            "VegetarianDiet" => Ok(RestrictedDiet::VegetarianDiet),
            _ => Err(RestrictedDietError {}),
        }
    }
}

impl fmt::Display for RestrictedDiet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RestrictedDiet::UnknownDiet => write!(f, "Unknown"),
            RestrictedDiet::DiabeticDiet => write!(f, " Diabetic"),
            RestrictedDiet::GlutenFreeDiet => write!(f, "Gluten Free"),
            RestrictedDiet::HalalDiet => write!(f, "Halal"),
            RestrictedDiet::HinduDiet => write!(f, "Hindu"),
            RestrictedDiet::KosherDiet => write!(f, "Kosher"),
            RestrictedDiet::LowCalorieDiet => write!(f, "Low Calorie"),
            RestrictedDiet::LowFatDiet => write!(f, "Low Fat"),
            RestrictedDiet::LowLactoseDiet => write!(f, "Low Lactose"),
            RestrictedDiet::LowSaltDiet => write!(f, "Low Salt"),
            RestrictedDiet::VeganDiet => write!(f, "Vegan"),
            RestrictedDiet::VegetarianDiet => write!(f, "Vegetarian"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_success() -> Result<(), RestrictedDietError> {
        assert_eq!(RestrictedDiet::UnknownDiet, "UnknownDiet".parse()?);
        assert_eq!(RestrictedDiet::DiabeticDiet, "DiabeticDiet".parse()?);
        assert_eq!(RestrictedDiet::GlutenFreeDiet, "GlutenFreeDiet".parse()?);
        assert_eq!(RestrictedDiet::HalalDiet, "HalalDiet".parse()?);
        assert_eq!(RestrictedDiet::HinduDiet, "HinduDiet".parse()?);
        assert_eq!(RestrictedDiet::KosherDiet, "KosherDiet".parse()?);
        assert_eq!(RestrictedDiet::LowCalorieDiet, "LowCalorieDiet".parse()?);
        assert_eq!(RestrictedDiet::LowFatDiet, "LowFatDiet".parse()?);
        assert_eq!(RestrictedDiet::LowLactoseDiet, "LowLactoseDiet".parse()?);
        assert_eq!(RestrictedDiet::LowSaltDiet, "LowSaltDiet".parse()?);
        assert_eq!(RestrictedDiet::VeganDiet, "VeganDiet".parse()?);
        assert_eq!(RestrictedDiet::VegetarianDiet, "VegetarianDiet".parse()?);
        Ok(())
    }

    #[test]
    fn test_from_str_failure() {
        let res = "Invalid".parse::<RestrictedDiet>();
        assert!(res.is_err())
    }
}
