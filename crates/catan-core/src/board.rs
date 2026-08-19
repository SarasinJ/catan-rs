use crate::hex::Hex;

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
    use std::collections::HashSet;

    #[test]
    fn nineteen_positions() {
        let board = standard_positions();
        assert_eq!(board.len(), 19)
    }

    #[test]
    fn no_duplicates() {
        let board = standard_positions();
        let unique: HashSet<Hex> = board.clone().into_iter().collect();
        assert_eq!(board.len(), unique.len())
    }

    #[test]
    fn contains_center_and_ring() {
        //includes (0,0); includes (2,0); excludes (2,1)
        let board = standard_positions();
        let include = [Hex::new(0, 0), Hex::new(2, 0)];
        for h in include {
            assert!(board.contains(&h));
        }
        let exclude = Hex::new(2, 1);
        assert!(!board.contains(&exclude))
    }

    #[test]
    fn all_distance_two_from_origin() {
        let board = standard_positions();
        let origin = Hex::origin();

        for h in board {
            assert!(h.distance(origin) <= 2)
        }
    }
}
