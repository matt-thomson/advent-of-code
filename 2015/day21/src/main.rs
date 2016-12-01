extern crate num;

mod character;
mod item;

use std::cmp::{max, min};
use std::env;

use character::Player;
pub use character::Character;
pub use item::Item;

fn main() {
    let hit_points = env::args().nth(1).expect("Must supply hit points").parse().unwrap();
    let damage = env::args().nth(2).expect("Must supply damage").parse().unwrap();
    let armor = env::args().nth(3).expect("Must supply armor").parse().unwrap();

    println!("{}", solve_part_one(hit_points, damage, armor));
    println!("{}", solve_part_two(hit_points, damage, armor));
}

fn solve_part_one(hit_points: u32, damage: u32, armor: u32) -> u32 {
    let boss = character::boss(hit_points, damage, armor);
    let mut best_so_far = u32::max_value();

    let op = |player: &Player, best_so_far: &mut u32| {
        if player.beats(&boss) {
            *best_so_far = min(*best_so_far, player.cost());
        };
    };

    solve(&op, &mut best_so_far)
}

fn solve_part_two(hit_points: u32, damage: u32, armor: u32) -> u32 {
    let boss = character::boss(hit_points, damage, armor);
    let mut best_so_far = 0;

    let op = Box::new(|player: &Player, best_so_far: &mut u32| {
        if !player.beats(&boss) {
            *best_so_far = max(*best_so_far, player.cost());
        };
    });

    solve(&*op, &mut best_so_far)
}

fn solve(op: &Fn(&Player, &mut u32) -> (), best_so_far: &mut u32) -> u32 {
    let weapons = weapons();
    let armors = armors();
    let rings = rings();

    for weapon in weapons.iter() {
        for armor in armors.iter() {
            try_player(weapon, armor, vec![], best_so_far, op);

            for ring1 in rings.iter() {
                try_player(weapon, armor, vec![ring1], best_so_far, op);

                for ring2 in rings.iter() {
                    if ring1.cost() != ring2.cost() {
                        try_player(weapon, armor, vec![ring1, ring2], best_so_far, op);
                    }
                }
            }
        }
    }

    *best_so_far
}

fn try_player(weapon: &Item,
              armor: &Item,
              rings: Vec<&Item>,
              best_so_far: &mut u32,
              op: &Fn(&Player, &mut u32) -> ()) {
    let player = character::player(weapon, armor, rings);
    op(&player, best_so_far);
}

fn weapons() -> Vec<Item> {
    vec![
        Item::new_weapon(8, 4),
        Item::new_weapon(10, 5),
        Item::new_weapon(25, 6),
        Item::new_weapon(40, 7),
        Item::new_weapon(74, 8)
    ]
}

fn armors() -> Vec<Item> {
    vec![
        Item::new_armor(0, 0),
        Item::new_armor(13, 1),
        Item::new_armor(31, 2),
        Item::new_armor(53, 3),
        Item::new_armor(75, 4),
        Item::new_armor(102, 5)
    ]
}

fn rings() -> Vec<Item> {
    vec![
        Item::new_armor(0, 0),
        Item::new_armor(0, 0),
        Item::new_weapon(25, 1),
        Item::new_weapon(50, 2),
        Item::new_weapon(100, 3),
        Item::new_armor(20, 1),
        Item::new_armor(40, 2),
        Item::new_armor(80, 3)
    ]
}
