use rand::{Rng, RngExt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Roll {
    pub die1: u8,
    pub die2: u8,
}

impl Roll {
    pub fn total(&self) -> u8 {
        self.die1 + self.die2
    }

    pub fn new(rng: &mut impl Rng) -> Self {
        Roll {
            die1: die_roll(rng),
            die2: die_roll(rng),
        }
    }
}

fn die_roll(rng: &mut impl Rng) -> u8 {
    rng.random_range(1..=6)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn dice_in_range() {
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let roll = die_roll(&mut rng);
            assert!((1..=6).contains(&roll))
        }
    }

    #[test]
    fn deterministic_with_seed() {
        let mut seed1 = StdRng::seed_from_u64(42);
        let mut seed2 = StdRng::seed_from_u64(42);

        let mut r1: Vec<Roll> = Vec::new();
        for _ in 0..10 {
            r1.push(Roll::new(&mut seed1));
        }

        let mut r2: Vec<Roll> = Vec::new();
        for _ in 0..10 {
            r2.push(Roll::new(&mut seed2));
        }

        assert_eq!(r1, r2)
    }

    #[test]
    fn total_adds_dice() {
        let dice = Roll { die1: 3, die2: 5 };

        assert_eq!(dice.total(), 8)
    }
}
