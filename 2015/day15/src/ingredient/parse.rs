use super::Ingredient;

use nom::{alpha, digit, space, IResult};

use std::str;
use std::str::FromStr;

pub fn parse(line: &str) -> Ingredient {
    let result = ingredient(line.as_bytes());

    match result {
        IResult::Done(_, ingredient) => ingredient,
        _                            => panic!("Invalid result {:?}", result)
    }
}

named!(name<String>, map_res!(map_res!(alpha, str::from_utf8), FromStr::from_str));

named!(positive<i32>, map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
named!(negative<i32>, chain!(tag!("-") ~ value: positive, || 0 - value));
named!(number<i32>, alt!(positive | negative));

named!(ingredient<Ingredient>, chain!(
    name:       name     ~
    tag!(": capacity")   ~
    space                ~
    capacity:   number   ~
    tag!(", durability") ~
    space                ~
    durability: number   ~
    tag!(", flavor")     ~
    space                ~
    flavor: number       ~
    tag!(", texture")    ~
    space                ~
    texture: number      ~
    tag!(", calories")   ~
    space                ~
    calories: number, || Ingredient::new(&name, capacity, durability, flavor, texture, calories)
));
