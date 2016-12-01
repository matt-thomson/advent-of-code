use ingredient::Ingredient;

use std::cmp::max;

#[derive(Debug)]
pub struct Recipe {
    available: u32,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Recipe {
    pub fn new() -> Recipe {
        Recipe {
            available: 100,
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0
        }
    }

    pub fn available(&self) -> u32 {
        self.available
    }

    pub fn with(&self, amount: u32, ingredient: &Ingredient) -> Recipe {
        Recipe {
            available: self.available - amount,
            capacity: self.capacity + amount as i32 * ingredient.capacity(),
            durability: self.durability + amount as i32 * ingredient.durability(),
            flavor: self.flavor + amount as i32 * ingredient.flavor(),
            texture: self.texture + amount as i32 * ingredient.texture(),
            calories: self.calories + amount as i32 * ingredient.calories()
        }
    }

    pub fn score(&self) -> u32 {
        [self.capacity, self.durability, self.flavor, self.texture].iter()
            .map(positive_or_zero)
            .fold(1, |acc, i| acc * i)
    }

    pub fn calories(&self) -> i32 {
        self.calories
    }
}

fn positive_or_zero(input: &i32) -> u32 {
    max(*input, 0) as u32
}

#[cfg(test)]
mod tests {
    use super::Recipe;
    use ingredient::Ingredient;

    #[test]
    fn test_new_recipe() {
        let recipe = Recipe::new();
        assert_eq!(recipe.available(), 100);
        assert_eq!(recipe.capacity, 0);
        assert_eq!(recipe.durability, 0);
        assert_eq!(recipe.flavor, 0);
        assert_eq!(recipe.texture, 0);
        assert_eq!(recipe.calories(), 0);
    }

    #[test]
    fn test_with() {
        let ingredient = Ingredient::parse("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8");
        let recipe = Recipe::new().with(10, &ingredient);

        assert_eq!(recipe.available, 90);
        assert_eq!(recipe.capacity, -10);
        assert_eq!(recipe.durability, -20);
        assert_eq!(recipe.flavor, 60);
        assert_eq!(recipe.texture, 30);
        assert_eq!(recipe.calories(), 80);
    }

    #[test]
    fn test_score_with_negative() {
        let ingredient = Ingredient::parse("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8");
        let recipe = Recipe::new().with(10, &ingredient);

        assert_eq!(recipe.score(), 0);
    }

    #[test]
    fn test_score_with_all_positive() {
        let ingredient = Ingredient::parse("Butterscotch: capacity 1, durability 2, flavor 6, texture 3, calories 8");
        let recipe = Recipe::new().with(10, &ingredient);

        assert_eq!(recipe.score(), 360000);
    }
}
