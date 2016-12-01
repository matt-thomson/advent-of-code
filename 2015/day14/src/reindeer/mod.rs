mod parse;

use std::cmp::min;

#[derive(Debug)]
pub struct Reindeer {
    name: String,
    speed: u32,
    fly_time: u32,
    rest_time: u32
}

impl Reindeer {
    fn new(name: &str, speed: u32, fly_time: u32, rest_time: u32) -> Reindeer {
        Reindeer { name: name.to_string(), speed: speed, fly_time: fly_time, rest_time: rest_time }
    }

    pub fn parse(input: &str) -> Reindeer {
        parse::parse(input)
    }

    pub fn distance(&self, time: u32) -> u32 {
        let mut time_left = time;
        let mut distance = 0;

        loop {
            distance += self.speed * min(self.fly_time, time_left);

            if time_left <= self.fly_time + self.rest_time {
                break;
            }

            time_left -= self.fly_time + self.rest_time;
        }

        distance
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Reindeer;

    #[test]
    fn test_parse_reindeer() {
        let reindeer = Reindeer::parse("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");
        assert_eq!(reindeer.speed, 16);
        assert_eq!(reindeer.fly_time, 11);
        assert_eq!(reindeer.rest_time, 162);
    }

    #[test]
    fn test_calculates_distance() {
        let reindeer = Reindeer::parse("Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.");
        assert_eq!(reindeer.distance(10), 160);
        assert_eq!(reindeer.distance(11), 176);
        assert_eq!(reindeer.distance(12), 176);
        assert_eq!(reindeer.distance(173), 176);
        assert_eq!(reindeer.distance(174), 192);
    }
}
