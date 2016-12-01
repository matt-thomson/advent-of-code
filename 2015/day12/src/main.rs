extern crate rustc_serialize;

use rustc_serialize::json::Json;

use std::collections::BTreeMap;
use std::env;
use std::fs::File;

fn main() {
    let filename = env::args().nth(1).expect("Must supply a filename");
    let mut file = File::open(filename).unwrap();
    let json = Json::from_reader(&mut file).unwrap();

    println!("{}", solve_part_one(&json));
    println!("{}", solve_part_two(&json));
}

fn solve_part_one(json: &Json) -> i64 {
    let sum_object = |items: &BTreeMap<String, Json>| {
        items.values().map(|v| solve_part_one(&v)).fold(0, |i, acc| i + acc)
    };

    sum(json, &sum_object)
}

fn solve_part_two(json: &Json) -> i64 {
    let sum_object = |items: &BTreeMap<String, Json>| {
        if items.values().any(|value| *value == Json::String("red".to_string())) {
            0
        } else {
            items.values().map(|v| solve_part_two(&v)).fold(0, |i, acc| i + acc)
        }
    };

    sum(json, &sum_object)
}

fn sum(json: &Json, sum_object: &Fn(&BTreeMap<String, Json>) -> i64) -> i64 {
    match json {
        &Json::I64(value)        => value,
        &Json::U64(value)        => value as i64,
        &Json::F64(value)        => value as i64,
        &Json::String(_)         => 0,
        &Json::Boolean(_)        => 0,
        &Json::Array(ref items)  => items.iter().map(|v| sum(&v, sum_object)).fold(0, |i, acc| i + acc),
        &Json::Object(ref items) => sum_object(&items),
        &Json::Null              => 0
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};
    use rustc_serialize::json::Json;

    #[test]
    fn test_part_one_example_1() {
        assert_eq!(solve_part_one(&Json::from_str("[1, 2, 3]").unwrap()), 6);
    }

    #[test]
    fn test_part_one_example_2() {
        assert_eq!(solve_part_one(&Json::from_str("{\"a\":2,\"b\":4}").unwrap()), 6);
    }

    #[test]
    fn test_part_one_example_3() {
        assert_eq!(solve_part_one(&Json::from_str("[[[3]]]").unwrap()), 3);
    }

    #[test]
    fn test_part_one_example_4() {
        assert_eq!(solve_part_one(&Json::from_str("{\"a\":{\"b\":4},\"c\":-1}").unwrap()), 3);
    }

    #[test]
    fn test_part_one_example_5() {
        assert_eq!(solve_part_one(&Json::from_str("{\"a\":[-1,1]}").unwrap()), 0);
    }

    #[test]
    fn test_part_one_example_6() {
        assert_eq!(solve_part_one(&Json::from_str("[-1,{\"a\":1}]").unwrap()), 0);
    }

    #[test]
    fn test_part_one_example_7() {
        assert_eq!(solve_part_one(&Json::from_str("[]").unwrap()), 0);
    }

    #[test]
    fn test_part_one_example_8() {
        assert_eq!(solve_part_one(&Json::from_str("{}").unwrap()), 0);
    }

    #[test]
    fn test_part_two_example_1() {
        assert_eq!(solve_part_two(&Json::from_str("[1, 2, 3]").unwrap()), 6);
    }

    #[test]
    fn test_part_two_example_2() {
        assert_eq!(solve_part_two(&Json::from_str("[1,{\"c\":\"red\",\"b\":2},3]").unwrap()), 4);
    }

    #[test]
    fn test_part_two_example_3() {
        assert_eq!(solve_part_two(&Json::from_str("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}").unwrap()), 0);
    }

    #[test]
    fn test_part_two_example_4() {
        assert_eq!(solve_part_two(&Json::from_str("[1,\"red\",5]").unwrap()), 6);
    }
}
