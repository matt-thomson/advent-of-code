#[derive(Debug)]
pub struct Item {
    cost: u32,
    damage: u32,
    armor: u32
}

impl Item {
    pub fn new_weapon(cost: u32, damage: u32) -> Item {
        Item { cost: cost, damage: damage, armor: 0 }
    }

    pub fn new_armor(cost: u32, armor: u32) -> Item {
        Item { cost: cost, damage: 0, armor: armor }
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    pub fn damage(&self) -> u32 {
        self.damage
    }

    pub fn armor(&self) -> u32 {
        self.armor
    }
}
