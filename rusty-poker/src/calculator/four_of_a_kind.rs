use card::Card;
use types;
use calculator::utility;

pub fn test(cards: Vec<Card>) -> Option<types::Combination> {
  if cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

  for (&rank_value, &count) in &hash_map {
    if count == 4 {
      return Some(types::Combination::FourOfAKind(rank_value));
    }
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
  fn none_for_only_three_same_cards() {
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
      ])
    );
  }

  #[test]
  fn none_for_full_house_five_cards() {
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Hearts,
        },
      ])
    );
  }

  #[test]
  fn option_for_seven_cards_with_four_of_a_kind() {
    assert_eq!(
      Some(types::Combination::FourOfAKind(types::Rank::Two)),
      test(vec![
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }
  #[test]
  fn option_for_seven_cards_with_four_of_a_kind_with_duplicates() {
    assert_eq!(
      Some(types::Combination::FourOfAKind(types::Rank::Two)),
      test(vec![
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }
}
