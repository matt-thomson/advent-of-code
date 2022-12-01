use std::collections::{BTreeSet, HashMap};
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
    pub fn routes(&self, can_repeat: bool) -> usize {
        self.routes_from(&Route::new(can_repeat))
    }

    fn routes_from(&self, route: &Route) -> usize {
        if route.position() == "end" {
            1
        } else {
            self.connections[route.position()]
                .iter()
                .flat_map(|next| route.step(next))
                .map(|route| self.routes_from(&route))
                .sum()
        }
    }
}
