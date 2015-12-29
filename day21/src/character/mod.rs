mod boss;
mod player;

use Item;

use num::integer::Integer;

pub use self::boss::Boss;
pub use self::player::Player;

pub fn player(weapon: &Item, armor: &Item, rings: Vec<&Item>) -> Player {
    Player::new(weapon, armor, rings)
}

pub fn boss(hit_points: u32, damage: u32, armor: u32) -> Boss {
    Boss::new(hit_points, damage, armor)
}

pub trait Character {
    fn hit_points(&self) -> u32;
    fn damage(&self) -> u32;
    fn armor(&self) -> u32;

    fn beats(&self, other: &Character) -> bool {
        let win_after = turns_to_beat(self.damage(), other.hit_points(), other.armor());
        let lose_after = turns_to_beat(other.damage(), self.hit_points(), self.armor());

        win_after <= lose_after
    }
}

fn turns_to_beat(damage: u32, hit_points: u32, armor: u32) -> u32 {
    let effective_damage = if damage > armor { damage - armor } else { 1 };
    let (div, rem) = hit_points.div_rem(&effective_damage);

    if rem == 0 { div } else { div + 1 }
}
