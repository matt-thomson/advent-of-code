use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use modinverse::modinverse;
use num::{BigUint, One, ToPrimitive};

use super::step::Step;

#[derive(Debug)]
pub struct Shuffle {
    a: usize,
    b: usize,
    cards: usize,
}

impl Shuffle {
    pub fn read(path: &Path, cards: usize) -> Self {
        let file = File::open(&path).unwrap();
        let reader = BufReader::new(file);

        let steps: Vec<Step> = reader
            .lines()
            .map(|line| Step::parse(&line.unwrap()))
            .collect();

        Self::from(&steps, cards)
    }

    fn from(steps: &[Step], cards: usize) -> Self {
        let mut a = 1;
        let mut b = 0;

        for step in steps {
            match step {
                Step::DealIntoNewStack => {
                    b = (cards + b - a) % cards;
                    a = cards - a;
                }
                Step::CutCards(number) => {
                    let offset = if *number < 0 {
                        (*number + cards as isize) as usize
                    } else {
                        *number as usize
                    };

                    b = (b + a * offset) % cards;
                }
                Step::DealWithIncrement(increment) => {
                    let inverse = modinverse(*increment as isize, cards as isize).unwrap() as usize;
                    a = (a * inverse) % cards;
                }
            }
        }

        Shuffle { a, b, cards }
    }

    pub fn card(&self, position: usize) -> usize {
        (self.a * position + self.b) % self.cards
    }

    pub fn repeat(&self, times: usize) -> Self {
        let a = BigUint::from(self.a);
        let b = BigUint::from(self.b);
        let cards = BigUint::from(self.cards);
        let times = BigUint::from(times);

        let inverse = BigUint::from(
            modinverse((self.cards - self.a + 1) as isize, self.cards as isize).unwrap() as usize,
        );

        let new_a = a.modpow(&times, &cards);
        let new_b = ((b * (&cards + BigUint::one() - &new_a)) * inverse) % &cards;

        Self {
            a: new_a.to_usize().unwrap(),
            b: new_b.to_usize().unwrap(),
            cards: self.cards,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::PathBuf;

    #[test]
    fn test_deal_into_new_stack() {
        let steps = [Step::DealIntoNewStack];

        let shuffle = Shuffle::from(&steps, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut_positive() {
        let steps = [Step::CutCards(3)];

        let shuffle = Shuffle::from(&steps, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_apply_cut_negative() {
        let steps = [Step::CutCards(-4)];

        let shuffle = Shuffle::from(&steps, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_apply_deal_with_increment() {
        let steps = [Step::DealWithIncrement(3)];

        let shuffle = Shuffle::from(&steps, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_read_a() {
        let path = PathBuf::from("fixtures/day22a.txt");

        let shuffle = Shuffle::read(&path, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_read_b() {
        let path = PathBuf::from("fixtures/day22b.txt");

        let shuffle = Shuffle::read(&path, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_read_c() {
        let path = PathBuf::from("fixtures/day22c.txt");

        let shuffle = Shuffle::read(&path, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_read_d() {
        let path = PathBuf::from("fixtures/day22d.txt");

        let shuffle = Shuffle::read(&path, 10);
        let result: Vec<usize> = (0..10).map(|position| shuffle.card(position)).collect();

        assert_eq!(result, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
