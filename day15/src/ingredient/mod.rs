mod parse;

#[derive(Debug)]
pub struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32
}

impl Ingredient {
    fn new(name: &str, capacity: i32, durability: i32, flavor: i32, texture: i32, calories: i32) -> Ingredient {
        Ingredient {
            name: name.to_string(),
            capacity: capacity,
            durability: durability,
            flavor: flavor,
            texture: texture,
            calories: calories
        }
    }

    pub fn parse(input: &str) -> Ingredient {
        parse::parse(input)
    }

    pub fn capacity(&self) -> i32 {
        self.capacity
    }

    pub fn durability(&self) -> i32 {
        self.durability
    }

    pub fn flavor(&self) -> i32 {
        self.flavor
    }

    pub fn texture(&self) -> i32 {
        self.texture
    }

    pub fn calories(&self) -> i32 {
        self.calories
    }
}

#[cfg(test)]
mod tests {
    use super::Ingredient;

    #[test]
    fn test_parse_ingredient() {
        let ingredient = Ingredient::parse("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8");
        assert_eq!(ingredient.capacity(), -1);
        assert_eq!(ingredient.durability(), -2);
        assert_eq!(ingredient.flavor(), 6);
        assert_eq!(ingredient.texture(), 3);
        assert_eq!(ingredient.calories(), 8);
    }
}
