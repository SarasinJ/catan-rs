use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevCard {
    Knight,
    VictoryPoint,
    RoadBuilding,
    YearOfPlenty,
    Monopoly,
}

/// The standard 25-card deck, unshuffled:
/// 14 Knight, 5 VictoryPoint, 2 RoadBuilding, 2 YearOfPlenty, 2 Monopoly.
pub fn standard_deck() -> Vec<DevCard> {
    [
        vec![DevCard::Knight; 14],
        vec![DevCard::VictoryPoint; 5],
        vec![DevCard::RoadBuilding; 2],
        vec![DevCard::YearOfPlenty; 2],
        vec![DevCard::Monopoly; 2],
    ]
    .concat()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deck_has_25_cards() {
        let deck = standard_deck();
        assert_eq!(deck.len(), 25)
    }

    #[test]
    fn deck_has_correct_counts() {
        let deck = standard_deck();
        let mut counts: HashMap<DevCard, usize> = HashMap::new();

        for card in deck {
            *counts.entry(card).or_insert(0) += 1;
        }

        assert_eq!(
            counts,
            HashMap::from([
                (DevCard::Knight, 14),
                (DevCard::VictoryPoint, 5),
                (DevCard::RoadBuilding, 2),
                (DevCard::YearOfPlenty, 2),
                (DevCard::Monopoly, 2)
            ])
        )
    }
}
