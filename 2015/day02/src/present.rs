pub struct Present {
    length: u32,
    width: u32,
    height: u32
}

impl Present {
    pub fn new(dimensions: &str) -> Present {
        let parsed = dimensions.split("x").map(|x| x.parse().unwrap()).collect::<Vec<_>>();
        Present { length: parsed[0], width: parsed[1], height: parsed[2] }
    }

    pub fn wrapping_paper(&self) -> u32 {
        self.surface_area() + self.slack()
    }

    pub fn ribbon(&self) -> u32 {
        self.volume() + self.wrap()
    }

    fn surface_area(&self) -> u32 {
        2 * (
            (self.length * self.width) +
            (self.width * self.height) +
            (self.height * self.length)
        )
    }

    fn volume(&self) -> u32 {
        self.length * self.width * self.height
    }

    fn slack(&self) -> u32 {
        let mut dimensions = [self.length, self.width, self.height];
        dimensions.sort();

        dimensions[0] * dimensions[1]
    }

    fn wrap(&self) -> u32 {
        let mut dimensions = [self.length, self.width, self.height];
        dimensions.sort();

        2 * (dimensions[0] + dimensions[1])
    }
}

#[cfg(test)]
mod tests {
    use super::Present;

    #[test]
    fn test_surface_area() {
        let present = Present::new("2x3x4");
        assert_eq!(present.surface_area(), 52);
    }

    #[test]
    fn test_volume() {
        let present = Present::new("2x3x4");
        assert_eq!(present.volume(), 24);
    }

    #[test]
    fn test_slack() {
        let present = Present::new("2x3x4");
        assert_eq!(present.slack(), 6);
    }

    #[test]
    fn test_wrap() {
        let present = Present::new("2x3x4");
        assert_eq!(present.wrap(), 10);
    }

    #[test]
    fn test_wrapping_paper() {
        let present = Present::new("2x3x4");
        assert_eq!(present.wrapping_paper(), 58);
    }

    #[test]
    fn test_ribbon() {
        let present = Present::new("2x3x4");
        assert_eq!(present.ribbon(), 34);
    }
}
