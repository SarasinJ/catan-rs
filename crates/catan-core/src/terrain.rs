use crate::resource::Resource;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Terrain {
    Hills,
    Forest,
    Pasture,
    Fields,
    Mountains,
    Desert,
}

impl Terrain {
    pub const ALL: [Terrain; 6] = [
        Terrain::Hills,
        Terrain::Forest,
        Terrain::Pasture,
        Terrain::Fields,
        Terrain::Mountains,
        Terrain::Desert,
    ];

    pub const TERRAIN_COUNTS: [(Terrain, u8); 6] = [
        (Terrain::Hills, 3),
        (Terrain::Forest, 4),
        (Terrain::Pasture, 4),
        (Terrain::Fields, 4),
        (Terrain::Mountains, 3),
        (Terrain::Desert, 1),
    ];

    pub fn produces(&self) -> Option<Resource> {
        match self {
            Terrain::Hills => Some(Resource::Brick),
            Terrain::Forest => Some(Resource::Lumber),
            Terrain::Pasture => Some(Resource::Wool),
            Terrain::Fields => Some(Resource::Grain),
            Terrain::Mountains => Some(Resource::Ore),
            Terrain::Desert => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_terrain_maps_correctly() {
        assert_eq!(Terrain::Hills.produces().unwrap(), Resource::Brick);
        assert_eq!(Terrain::Forest.produces().unwrap(), Resource::Lumber);
        assert_eq!(Terrain::Pasture.produces().unwrap(), Resource::Wool);
        assert_eq!(Terrain::Fields.produces().unwrap(), Resource::Grain);
        assert_eq!(Terrain::Mountains.produces().unwrap(), Resource::Ore);
        assert_eq!(Terrain::Desert.produces(), None);
    }
    #[test]
    fn only_desert_is_barren() {
        let all = Terrain::ALL;

        for terrain in all {
            if terrain == Terrain::Desert {
                assert!(terrain.produces().is_none())
            } else {
                assert!(terrain.produces().is_some())
            }
        }
    }
}
