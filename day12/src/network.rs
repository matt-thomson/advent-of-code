use std::collections::{BTreeSet, HashMap, VecDeque};
use std::convert::Infallible;
use std::str::FromStr;

use crate::route::Route;

#[derive(Debug)]
pub struct Network {
    connections: HashMap<String, BTreeSet<String>>,
}

impl FromStr for Network {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut connections = HashMap::new();

        let mut insert = |first: &str, second: &str| {
            if first != "end" && second != "start" {
                connections
                    .entry(first.to_string())
                    .or_insert_with(BTreeSet::new)
                    .insert(second.to_string());
            }
        };

        for line in input.lines() {
            let (first, second) = line.split_once('-').unwrap();

            insert(first, second);
            insert(second, first);
        }

        Ok(Self { connections })
    }
}

impl Network {
    pub fn find_routes(&self, can_repeat: bool) -> usize {
        let mut result = 0;

        let mut queue = VecDeque::new();
        queue.push_back(Route::new(can_repeat));

        while let Some(route) = queue.pop_front() {
            if route.position() == "end" {
                result += 1;
            } else {
                for next in self
                    .connections
                    .get(route.position())
                    .unwrap()
                    .iter()
                    .flat_map(|next| route.step(next))
                {
                    queue.push_back(next);
                }
            }
        }

        result
    }
}
