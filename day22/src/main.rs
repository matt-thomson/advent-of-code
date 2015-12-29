extern crate rand;

mod boss;
mod spell;
mod player;

use std::cmp::min;
use std::env;

pub use boss::Boss;
pub use player::Player;
pub use spell::Spell;

fn main() {
    let hit_points = env::args().nth(1).expect("Must supply hit points").parse().unwrap();
    let damage = env::args().nth(2).expect("Must supply damage").parse().unwrap();

    println!("{}", solve_part_one(hit_points, damage));
    println!("{}", solve_part_two(hit_points, damage));
}

fn solve_part_one(hit_points: u32, damage: u32) -> u32 {
    solve(hit_points, damage, 0)
}

fn solve_part_two(hit_points: u32, damage: u32) -> u32 {
    solve(hit_points, damage, 1)
}

fn solve(hit_points: u32, damage: u32, damage_per_turn: u32) -> u32 {
    let mut best_so_far = u32::max_value();

    for _ in 0..10000000 {
        match play_game(hit_points, damage, damage_per_turn) {
            Some(mana) => best_so_far = min(best_so_far, mana),
            None       => ()
        }
    }

    best_so_far
}

fn play_game(hit_points: u32, damage: u32, damage_per_turn: u32) -> Option<u32> {
    let mut boss = Boss::new(hit_points, damage);
    let mut player = Player::new();

    let mut mana_spent = 0;

    loop {
        player.damage(damage_per_turn);

        if !player.alive() {
            return None;
        }

        player.tick();
        boss.tick();

        if !boss.alive() {
            return Some(mana_spent);
        }

        if !player.can_cast_spell() {
            return None;
        }

        let spell = player.select_spell();

        player.cast(&spell);
        boss.receive(&spell);
        mana_spent += spell.cost();

        player.tick();
        boss.tick();

        if !boss.alive() {
            return Some(mana_spent);
        }

        boss.attack(&mut player);

        if !player.alive() {
            return None;
        }
    }
}
