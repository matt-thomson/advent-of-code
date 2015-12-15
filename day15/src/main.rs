#[macro_use]
extern crate nom;

mod ingredient;
mod recipe;

use ingredient::Ingredient;
use recipe::Recipe;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");

    println!("{}", solve_part_one(&filename));
    println!("{}", solve_part_two(&filename));
}

fn solve_part_one(filename: &str) -> u32 {
    solve(filename, &|_| true)
}

fn solve_part_two(filename: &str) -> u32 {
    solve(filename, &|recipe| recipe.calories() == 500)
}

fn solve(filename: &str, predicate: &Fn(&Recipe) -> bool) -> u32 {
    let file = BufReader::new(File::open(filename).unwrap());
    let ingredients = file.lines().map(|line| Ingredient::parse(&line.unwrap())).collect::<Vec<_>>();
    let recipe = Recipe::new();

    find_best_score(&ingredients, &recipe, predicate)
}

fn find_best_score(ingredients: &[Ingredient],
                   recipe: &Recipe,
                   predicate: &Fn(&Recipe) -> bool) -> u32 {
    let ingredient = &ingredients[0];
    let other_ingredients = &ingredients[1..];

    if ingredients.len() == 1 {
        let final_recipe = recipe.with(recipe.available(), ingredient);

        if predicate(&final_recipe) {
            return final_recipe.score();
        } else {
            return 0;
        }
    }

    (0..(recipe.available() + 1))
        .map(|i| find_best_score(&other_ingredients, &recipe.with(i, ingredient), predicate))
        .max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    #[test]
    fn test_part_one() {
        assert_eq!(solve_part_one("data/example.txt"), 62842880);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(solve_part_two("data/example.txt"), 57600000);
    }
}
