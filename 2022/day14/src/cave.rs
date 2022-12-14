use std::collections::HashSet;

use eyre::Result;

use crate::rock::Rock;

#[derive(Debug)]
pub struct Cave {
    occupied: HashSet<(usize, usize)>,
}

impl Cave {
    pub fn new(rocks: &[Rock]) -> Result<Self> {
        let mut occupied = HashSet::new();

        for rock in rocks {
            for point in rock.points()? {
                occupied.insert(point);
            }
        }

        Ok(Self { occupied })
    }
}
