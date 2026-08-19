// Axial (q, r) based hex coordinates: https://www.redblobgames.com/grids/hexagons/

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hex {
    pub q: i8,
    pub r: i8,
}

impl std::ops::Add for Hex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl std::ops::Sub for Hex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl Hex {
    pub fn new(q: i8, r: i8) -> Self {
        Hex { q, r }
    }

    pub fn origin() -> Self {
        Hex { q: 0, r: 0 }
    }

    /// The 6 adjacent hex coordinates, using these axial direction offsets
    /// (pointy-top orientation), in this order:
    ///   (+1, -1), (+1, 0), (0, +1), (-1, +1), (-1, 0), (0, -1)
    pub fn neighbors(self) -> [Hex; 6] {
        let directions = [
            Hex { q: 1, r: -1 },
            Hex { q: 1, r: 0 },
            Hex { q: 0, r: 1 },
            Hex { q: -1, r: 1 },
            Hex { q: -1, r: 0 },
            Hex { q: 0, r: -1 },
        ];

        directions.map(|x| x + self)
    }

    /// Manhattan (grid) distance between two hexes
    pub fn distance(self, rhs: Self) -> u8 {
        let dq = self.q - rhs.q;
        let dr = self.r - rhs.r;
        let ds = dq + dr;

        ((dq.abs() + dr.abs() + ds.abs()) / 2) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn six_distinct_neighbors() {
        let h = Hex { q: 1, r: 0 };
        let neighbors = h.neighbors();
        // has 6
        assert_eq!(neighbors.len(), 6);
        // is unique
        let unique: HashSet<Hex> = neighbors.into_iter().collect();
        assert_eq!(unique.len(), neighbors.len())
    }
    #[test]
    fn neighbors_are_at_distance_one() {
        let h = Hex { q: -1, r: 1 };
        for n in h.neighbors() {
            let d = h.distance(n);
            assert_eq!(d, 1);
        }
    }

    #[test]
    fn neighborhood_is_symmetric() {
        let h = Hex { q: -2, r: 3 };
        for n in h.neighbors() {
            assert!(n.neighbors().contains(&h))
        }
    }

    #[test]
    fn distance_examples() {
        let h = Hex { q: 0, r: 0 };
        //self = 0
        assert_eq!(h.distance(h), 0);
        //(2, -1) = 2
        assert_eq!(h.distance(Hex::new(2, -1)), 2)
    }
}
