use std::collections::HashSet;

use eyre::Result;

use crate::rock::Rock;

#[derive(Debug)]
pub struct Cave {
    occupied: HashSet<(usize, usize)>,
    bottom: usize,
}

impl Cave {
    pub fn new(rocks: &[Rock]) -> Result<Self> {
        let mut occupied = HashSet::new();

        for rock in rocks {
            for point in rock.points()? {
                occupied.insert(point);
            }
        }

        let bottom = *occupied.iter().map(|(_, y)| y).max().unwrap();

        Ok(Self { occupied, bottom })
    }

    pub fn drop(&mut self) -> bool {
        let mut x = 500;

        for y in 1..=self.bottom {
            let candidates = [x, x - 1, x + 1];
            if let Some(next) = candidates
                .into_iter()
                .find(|candidate| !self.occupied.contains(&(*candidate, y)))
            {
                x = next;
            } else {
                self.occupied.insert((x, y - 1));
                return true;
            }
        }

        false
    }
}
