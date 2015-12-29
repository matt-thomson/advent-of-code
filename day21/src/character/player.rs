use {Character, Item};

#[derive(Debug)]
pub struct Player {
    cost: u32,
    damage: u32,
    armor: u32
}

impl Player {
    pub fn new(weapon: &Item, armor: &Item, rings: Vec<&Item>) -> Player {
        let mut player = Player { cost: 0, damage: 0, armor: 0 };

        player.add_item(weapon);
        player.add_item(armor);

        for ring in rings {
            player.add_item(ring);
        }

        player
    }

    pub fn cost(&self) -> u32 {
        self.cost
    }

    fn add_item(&mut self, item: &Item) {
        self.cost += item.cost();
        self.damage += item.damage();
        self.armor += item.armor();
    }
}

impl Character for Player {
    fn hit_points(&self) -> u32 {
        100
    }

    fn damage(&self) -> u32 {
        self.damage
    }

    fn armor(&self) -> u32 {
        self.armor
    }
}
