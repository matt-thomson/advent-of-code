use rand::{Rand, Rng};

#[derive(Debug)]
pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge
}

impl Spell {
    pub fn cost(&self) -> u32 {
        match *self {
            Spell::MagicMissile => 53,
            Spell::Drain        => 73,
            Spell::Shield       => 113,
            Spell::Poison       => 173,
            Spell::Recharge     => 229
        }
    }
}

impl Rand for Spell {
    fn rand<R: Rng>(rng: &mut R) -> Spell {
        match rng.gen_range(0, 5) {
            0 => Spell::MagicMissile,
            1 => Spell::Drain,
            2 => Spell::Shield,
            3 => Spell::Poison,
            4 => Spell::Recharge,
            _ => unreachable!()
        }
    }
}
