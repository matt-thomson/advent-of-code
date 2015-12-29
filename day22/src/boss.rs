use {Player, Spell};

#[derive(Debug)]
pub struct Boss {
    hit_points: u32,
    damage: u32,
    poison_timer: u32
}

impl Boss {
    pub fn new(hit_points: u32, damage: u32) -> Boss {
        Boss { hit_points: hit_points, damage: damage, poison_timer: 0 }
    }

    pub fn receive(&mut self, spell: &Spell) {
        match *spell {
            Spell::MagicMissile => self.damage(4),
            Spell::Drain => self.damage(2),
            Spell::Shield => (),
            Spell::Poison => self.poison_timer = 6,
            Spell::Recharge => ()
        };
    }

    pub fn tick(&mut self) {
        if self.poison_timer > 0 {
            self.damage(3);
            self.poison_timer -= 1;
        }
    }

    pub fn attack(&self, player: &mut Player) {
        player.damage(self.damage);
    }

    pub fn alive(&self) -> bool {
        self.hit_points > 0
    }

    fn damage(&mut self, amount: u32) {
        self.hit_points = if amount > self.hit_points { 0 } else { self.hit_points - amount };
    }
}
