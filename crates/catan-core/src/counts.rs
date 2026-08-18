use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::resource::Resource;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceCounts(HashMap<Resource, u8>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CountsError {
    Insufficient {
        resource: Resource,
        wanted: u8,
        had: u8,
    },
}

impl ResourceCounts {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// The bank's starting stock; 19 of each resource.
    pub fn bank() -> Self {
        ResourceCounts(HashMap::from([
            (Resource::Brick, 19),
            (Resource::Lumber, 19),
            (Resource::Wool, 19),
            (Resource::Grain, 19),
            (Resource::Ore, 19),
        ]))
    }

    pub fn get(&self, r: Resource) -> u8 {
        self.0.get(&r).copied().unwrap_or(0)
    }

    pub fn add(&mut self, r: Resource, n: u8) {
        *self.0.entry(r).or_insert(0) += n;
    }
    
    /// Removes n of r, or returns an error and leaves self UNCHANGED.
    pub fn try_remove(&mut self, r: Resource, n: u8) -> Result<(), CountsError> {
        let count = self.get(r);
        if count < n {
            return Err(CountsError::Insufficient {
                resource: r,
                wanted: n,
                had: count,
            });
        }

        let new_count = count - n;
        self.0.insert(r, new_count);

        Ok(())
    }

    pub fn total(&self) -> u8 {
        self.0.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_then_get() {
        let mut counts = ResourceCounts::new();
        counts.add(Resource::Lumber, 4);

        assert_eq!(counts.get(Resource::Lumber), 4)
    }

    #[test]
    fn remove_succeeds_when_enough() {
        let mut counts = ResourceCounts(HashMap::from([
            (Resource::Brick, 3),
            (Resource::Wool, 2),
            (Resource::Ore, 1),
        ]));

        let result = counts.try_remove(Resource::Brick, 3);
        assert_eq!(result, Ok(()))
    }

    #[test]
    fn remove_fails_when_short() {
        let mut counts = ResourceCounts(HashMap::from([
            (Resource::Brick, 3),
            (Resource::Wool, 2),
            (Resource::Ore, 1),
        ]));

        let result = counts.try_remove(Resource::Brick, 4);
        assert_eq!(
            result,
            Err(CountsError::Insufficient {
                resource: Resource::Brick,
                wanted: 4,
                had: 3,
            })
        )
    }
}
