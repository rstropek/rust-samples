#[derive(Debug, PartialEq, Eq)]
pub enum RecipeType { Traditional, Vegetarian, Vegan, }

pub mod caesar_salad {
    use super::*;

    // Get ingredients for a Caesar salad
    pub fn get_ingredients(recipe_type: RecipeType) -> Vec<&'static str> {
        let mut ingredients = vec![
            "Romaine lettuce", "Croutons", "Lemon juice",
            "Olive oil", "Salt", "Black pepper",
        ];

        ingredients.push(match recipe_type {
            RecipeType::Traditional | RecipeType::Vegetarian => "Parmesan cheese",
            _ => "Vegan Parmesan cheese substitute",
        });

        ingredients.extend_from_slice(match recipe_type {
            RecipeType::Traditional => &["Anchovies", "Egg yolks"],
            RecipeType::Vegetarian => &["Capers", "Egg yolks"],
            _ => &["Capers", "Vegan mayonnaise"],
        });

        ingredients
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_anchovies_in_vegetarian_or_vegan() {
        let ingredients = caesar_salad::get_ingredients(RecipeType::Vegetarian);

        // Make sure there are no anchovies in a vegetarian salad
        assert!(!ingredients.contains(&"Anchovies"));
    }

    // Add additional tests here
}
