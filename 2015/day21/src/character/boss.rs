use Character;

#[derive(Debug)]
pub struct Boss {
    hit_points: u32,
    damage: u32,
    armor: u32
}

impl Boss {
    pub fn new(hit_points: u32, damage: u32, armor: u32) -> Boss {
        Boss { hit_points: hit_points, damage: damage, armor: armor }
    }
}

impl Character for Boss {
    fn hit_points(&self) -> u32 {
        self.hit_points
    }

    fn damage(&self) -> u32 {
        self.damage
    }

    fn armor(&self) -> u32 {
        self.armor
    }
}
