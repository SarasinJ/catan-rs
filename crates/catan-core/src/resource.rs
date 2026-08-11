use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    Brick,
    Lumber,
    Wool,
    Grain,
    Ore,
}

impl Resource {
    pub const ALL: [Resource; 5] = [
        Resource::Brick,
        Resource::Lumber,
        Resource::Wool,
        Resource::Grain,
        Resource::Ore,
    ];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_has_five_distinct() {
        let all = Resource::ALL;
        assert_eq!(5, all.len());
        for i in 0..all.len() {
            for j in (i + 1)..all.len() {
                assert_ne!(all[i], all[j]);
            }
        }
    }
}
