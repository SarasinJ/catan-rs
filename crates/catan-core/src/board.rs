use crate::hex::Hex;
use crate::terrain::Terrain;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tile {
    pub terrain: Terrain,
    pub token: Option<u8>,
}

pub struct Board {
    tiles: HashMap<Hex, Tile>,
}

impl Board {
    /// `standard_positions()` generates a board by row from left to right,
    /// this matches that by position and maps the default setup from the rulebook.
    pub const STANDARD_TILES: [Tile; 19] = [
        // region Standard tiles
        Tile {
            terrain: Terrain::Forest,
            token: Some(10),
        },
        Tile {
            terrain: Terrain::Fields,
            token: Some(2),
        },
        Tile {
            terrain: Terrain::Mountains,
            token: Some(6),
        },
        Tile {
            terrain: Terrain::Hills,
            token: Some(6),
        },
        Tile {
            terrain: Terrain::Fields,
            token: Some(5),
        },
        Tile {
            terrain: Terrain::Hills,
            token: Some(9),
        },
        Tile {
            terrain: Terrain::Fields,
            token: Some(3),
        },
        Tile {
            terrain: Terrain::Desert,
            token: None,
        },
        Tile {
            terrain: Terrain::Pasture,
            token: Some(3),
        },
        Tile {
            terrain: Terrain::Hills,
            token: Some(12),
        },
        Tile {
            terrain: Terrain::Mountains,
            token: Some(4),
        },
        Tile {
            terrain: Terrain::Pasture,
            token: Some(10),
        },
        Tile {
            terrain: Terrain::Fields,
            token: Some(8),
        },
        Tile {
            terrain: Terrain::Forest,
            token: Some(4),
        },
        Tile {
            terrain: Terrain::Forest,
            token: Some(11),
        },
        Tile {
            terrain: Terrain::Forest,
            token: Some(8),
        },
        Tile {
            terrain: Terrain::Pasture,
            token: Some(11),
        },
        Tile {
            terrain: Terrain::Mountains,
            token: Some(9),
        },
        Tile {
            terrain: Terrain::Pasture,
            token: Some(5),
        },
        // endregion
    ];

    pub fn standard_fixed() -> Self {
        let standard_board = standard_positions();

        Self {
            tiles: standard_board
                .into_iter()
                .zip(Self::STANDARD_TILES)
                .collect(),
        }
    }

    pub fn tile(&self, at: Hex) -> Option<&Tile> {
        self.tiles.get(&at)
    }

    pub fn tiles(&self) -> impl Iterator<Item = (&Hex, &Tile)> {
        self.tiles.iter()
    }
}

/// All axial coords (q, r) where max(|q|, |r|, |q + r|) <= 2.
/// That inequality is exactly "within 2 hex steps of the origin".
pub fn standard_positions() -> Vec<Hex> {
    let mut board = Vec::new();
    let o = Hex::origin();
    for q in -2..=2 {
        for r in -2..=2 {
            let h = Hex::new(q, r);
            if h.distance(o) <= 2 {
                board.push(h)
            }
        }
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokens::STANDARD_TOKENS;
    use std::collections::HashSet;

    #[test]
    fn nineteen_positions() {
        let hexes = standard_positions();
        assert_eq!(hexes.len(), 19)
    }

    #[test]
    fn no_duplicates() {
        let hexes = standard_positions();
        let unique: HashSet<Hex> = hexes.clone().into_iter().collect();
        assert_eq!(hexes.len(), unique.len())
    }

    #[test]
    fn contains_center_and_ring() {
        //includes (0,0); includes (2,0); excludes (2,1)
        let hexes = standard_positions();
        let include = [Hex::new(0, 0), Hex::new(2, 0)];
        for h in include {
            assert!(hexes.contains(&h));
        }
        let exclude = Hex::new(2, 1);
        assert!(!hexes.contains(&exclude))
    }

    #[test]
    fn all_distance_two_from_origin() {
        let hexes = standard_positions();
        let origin = Hex::origin();

        for h in hexes {
            assert!(h.distance(origin) <= 2)
        }
    }

    #[test]
    fn has_19_tiles() {
        let hexes = standard_positions();
        let board = Board::standard_fixed();

        assert_eq!(board.tiles.len(), 19);

        // each standard hex is in board
        for hex in &hexes {
            assert!(board.tiles.contains_key(hex))
        }

        // each board location is in standard hexes
        for (hex, _) in board.tiles() {
            assert!(hexes.contains(hex))
        }
    }

    #[test]
    fn terrain_multiset_correct() {
        let board = Board::standard_fixed();
        let standard_counts = HashMap::from(Terrain::TERRAIN_COUNTS);

        let terrain_counts = board.tiles.values().fold(HashMap::new(), |mut acc, tile| {
            *acc.entry(tile.terrain).or_insert(0) += 1;
            acc
        });

        assert_eq!(terrain_counts, standard_counts)
    }

    #[test]
    fn token_multiset_correct() {
        let board = Board::standard_fixed();
        let standard_tokens = STANDARD_TOKENS.iter().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(*x).or_insert(0) += 1;
            acc
        });

        let token_counts = board.tiles.values().filter_map(|tile| tile.token).fold(
            HashMap::new(),
            |mut acc, token| {
                *acc.entry(token).or_default() += 1;
                acc
            },
        );

        assert_eq!(token_counts, standard_tokens)
    }

    #[test]
    fn desert_and_only_desert_has_no_token() {
        let board = Board::standard_fixed();

        for (_, tile) in board.tiles() {
            match tile.terrain {
                Terrain::Desert => assert!(tile.token.is_none()),
                _ => assert!(tile.token.is_some()),
            }
        }
    }
}
