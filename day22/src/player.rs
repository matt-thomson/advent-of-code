use rand;
use rand::Rng;

use std::cmp::max;

use Spell;

#[derive(Debug)]
pub struct Player {
    hit_points: u32,
    mana: u32,
    shield_timer: u32,
    recharge_timer: u32
}

impl Player {
    pub fn new() -> Player {
        Player {
            hit_points: 50,
            mana: 500,
            shield_timer: 0,
            recharge_timer: 0
        }
    }

    pub fn cast(&mut self, spell: &Spell) {
        match *spell {
            Spell::MagicMissile => (),
            Spell::Drain => self.hit_points += 2,
            Spell::Shield => self.shield_timer = 6,
            Spell::Poison => (),
            Spell::Recharge => self.recharge_timer = 5
        };

        self.mana -= spell.cost();
    }

    pub fn tick(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }

        if self.recharge_timer > 0 {
            self.mana += 101;
            self.recharge_timer -= 1;
        }
    }

    pub fn alive(&self) -> bool {
        self.hit_points > 0
    }

    pub fn damage(&mut self, amount: u32) {
        let actual_damage = self.actual_damage(amount);
        self.hit_points = if actual_damage > self.hit_points { 0 } else { self.hit_points - actual_damage };
    }

    pub fn can_cast_spell(&self) -> bool {
        self.mana >= 53
    }

    pub fn select_spell(&self) -> Spell {
        loop {
            let spell: Spell = rand::thread_rng().gen();

            if spell.cost() <= self.mana {
                return spell;
            }
        }
    }

    fn actual_damage(&self, amount: u32) -> u32 {
        if self.shield_timer > 0 {
            if amount > 7 {
                amount - 7
            } else {
                max(1, amount)
            }
        } else {
            amount
        }
    }
}
