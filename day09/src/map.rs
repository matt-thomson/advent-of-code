use std::collections::{BTreeSet, HashMap};

use link::Link;

#[derive(Debug)]
pub struct Map {
    places: Vec<String>,
    distances: HashMap<(String, String), u32>
}

impl Map {
    pub fn new<I: Iterator<Item = Link>>(links: I) -> Map {
        let mut places = BTreeSet::new();
        let mut distances = HashMap::new();

        for link in links {
            places.insert(link.from());
            places.insert(link.to());

            distances.insert((link.from(), link.to()), link.distance());
            distances.insert((link.to(), link.from()), link.distance());
        }

        Map { places: places.iter().map(|x| x.to_string()).collect(), distances: distances }
    }

    pub fn places(&self) -> &Vec<String> {
        &self.places
    }

    pub fn distance(&self, from: &str, to: &str) -> u32 {
        *self.distances.get(&(from.to_string(), to.to_string())).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Map;
    use link::Link;

    #[test]
    fn should_build_map() {
        let links = vec![
            Link::parse("London to Dublin = 464"),
            Link::parse("London to Belfast = 518"),
            Link::parse("Dublin to Belfast = 141")
        ];

        let map = Map::new(links.into_iter());

        assert_eq!(map.places, vec!["Belfast", "Dublin", "London"]);
        assert_eq!(map.distance("London", "Belfast"), 518);
        assert_eq!(map.distance("Belfast", "London"), 518);
    }
}
