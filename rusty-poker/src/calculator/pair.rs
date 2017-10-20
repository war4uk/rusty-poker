use card::Card;
use types;
use calculator::utility;

pub fn test(cards: Vec<Card>) -> Option<types::Combination> {
    if cards.len() < 2 {
        return None;
    }

    let hash_map = utility::get_count_hash_map(&cards[..]);

    let mut current_pair: Option<types::Rank> = None;

    for (&rank_value, &count) in &hash_map {
        if count == 2 {
            // if we have 3 or 4, then it should have been handled already
            if let Some(current_pair_rank) = current_pair {
                if current_pair_rank < rank_value {
                    current_pair = Some(rank_value);
                }
            } else {
                current_pair = Some(rank_value);
            }
        }
    }

    if let Some(current_pair_rank) = current_pair {
        return Some(types::Combination::Pair(current_pair_rank));
    }

    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_for_empty() {
        assert_eq!(None, test(vec![]));
    }

    #[test]
    fn none_for_one_card() {
        assert_eq!(
            None,
            test(vec![
                Card {
                    rank: types::Rank::Two,
                    suit: types::Suit::Hearts,
                },
            ])
        );
    }

    #[test]
    fn none_for_two_different_cards() {
        assert_eq!(
            None,
            test(vec![
                Card {
                    rank: types::Rank::Two,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Three,
                    suit: types::Suit::Hearts,
                },
            ])
        );
    }

    #[test]
    fn chooses_highest_pair() {
        assert_eq!(
            Some(types::Combination::Pair(types::Rank::Three)),
            test(vec![
                Card {
                    rank: types::Rank::Two,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Three,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Two,
                    suit: types::Suit::Diamonds,
                },
                Card {
                    rank: types::Rank::Ace,
                    suit: types::Suit::Diamonds,
                },
                Card {
                    rank: types::Rank::King,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Three,
                    suit: types::Suit::Spades,
                },
            ])
        );
    }
}