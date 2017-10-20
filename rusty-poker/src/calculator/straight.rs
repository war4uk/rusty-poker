use types;
use card::Card;

use calculator::utility;

pub fn test(mut cards: Vec<Card>) -> Option<types::Combination> {
    if cards.len() < 5 {
        return None;
    }

    utility::sort_cards(&mut cards);
    let mut ranks = cards
        .iter()
        .map(|card| card.rank)
        .collect::<Vec<types::Rank>>();

    ranks.dedup();

    if test_ace_finishing_straight(&ranks[..]) {
        return Some(types::Combination::Straight(
            [
                types::Rank::Five,
                types::Rank::Four,
                types::Rank::Three,
                types::Rank::Two,
                types::Rank::Ace,
            ],
        ));
    }

    for i in 0..(ranks.len() - 5 + 1) {
        let slice_to_test = &ranks[i..(i + 5)];
        if test_straight_for_slice(slice_to_test) {

            let mut result: [types::Rank; 5] = [types::Rank::Ace; 5];
            result.clone_from_slice(slice_to_test);
            return Some(types::Combination::Straight(result));
        }
    }
    None
}

fn test_straight_for_slice(ranks: &[types::Rank]) -> bool {
    let mut prev_rank = ranks[0];
    for rank in ranks[1..].iter() {
        if prev_rank as i32 != (*rank as i32 + 1) {
            return false;
        }
        prev_rank = *rank;
    }

    return true;
}

fn test_ace_finishing_straight(ranks: &[types::Rank]) -> bool {
    let ace_finishing_straight_ranks = vec![
        types::Rank::Ace,
        types::Rank::Two,
        types::Rank::Three,
        types::Rank::Four,
        types::Rank::Five,
    ];
    for rank_in_straight in ace_finishing_straight_ranks.iter() {
        let mut was_found = false;

        for rank_in_hand in ranks.iter() {
            if rank_in_hand == rank_in_straight {
                was_found = true;
                break;
            }
        }

        if !was_found {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none_for_empty() {
        assert_eq!(None, test(vec![]));
    }

    #[test]
    fn none_for_four_cards() {
        assert_eq!(
            None,
            test(vec![
                Card {
                    rank: types::Rank::Four,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Five,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Six,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Seven,
                    suit: types::Suit::Spades,
                },
            ])
        );
    }

    #[test]
    fn returns_ace_finishing_straight() {
        assert_eq!(
            Some(types::Combination::Straight(
                [
                    types::Rank::Five,
                    types::Rank::Four,
                    types::Rank::Three,
                    types::Rank::Two,
                    types::Rank::Ace,
                ],
            )),
            test(vec![
                Card {
                    rank: types::Rank::Three,
                    suit: types::Suit::Spades,
                },

                Card {
                    rank: types::Rank::Jack,
                    suit: types::Suit::Diamonds,
                },
                Card {
                    rank: types::Rank::Five,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Four,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Ace,
                    suit: types::Suit::Clubs,
                },
                Card {
                    rank: types::Rank::King,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Two,
                    suit: types::Suit::Spades,
                },
            ])
        );
    }

    #[test]
    fn returns_ace_starting_straight() {
        assert_eq!(
            Some(types::Combination::Straight(
                [
                    types::Rank::Ace,
                    types::Rank::King,
                    types::Rank::Queen,
                    types::Rank::Jack,
                    types::Rank::Ten,
                ],
            )),
            test(vec![
                Card {
                    rank: types::Rank::King,
                    suit: types::Suit::Spades,
                },

                Card {
                    rank: types::Rank::Jack,
                    suit: types::Suit::Diamonds,
                },
                Card {
                    rank: types::Rank::Jack,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Queen,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Ace,
                    suit: types::Suit::Clubs,
                },
                Card {
                    rank: types::Rank::King,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Ten,
                    suit: types::Suit::Spades,
                },
            ])
        );
    }

    #[test]
    fn returns_queen_starting_straight() {
        assert_eq!(
            Some(types::Combination::Straight(
                [
                    types::Rank::Queen,
                    types::Rank::Jack,
                    types::Rank::Ten,
                    types::Rank::Nine,
                    types::Rank::Eight,
                ],
            )),
            test(vec![
                Card {
                    rank: types::Rank::Eight,
                    suit: types::Suit::Spades,
                },

                Card {
                    rank: types::Rank::Jack,
                    suit: types::Suit::Diamonds,
                },
                Card {
                    rank: types::Rank::Jack,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Queen,
                    suit: types::Suit::Hearts,
                },
                Card {
                    rank: types::Rank::Ace,
                    suit: types::Suit::Clubs,
                },
                Card {
                    rank: types::Rank::Ten,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Ten,
                    suit: types::Suit::Spades,
                },
                Card {
                    rank: types::Rank::Nine,
                    suit: types::Suit::Spades,
                },
            ])
        );
    }
}