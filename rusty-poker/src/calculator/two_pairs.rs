use card::Card;
use types;

use calculator::utility;

pub fn test(cards: Vec<Card>) -> Option<types::Combination> {
  if cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

  let mut highest_two_cards: Option<types::Rank> = None;
  let mut lower_two_cards: Option<types::Rank> = None;

  let mut current_pair_rank: types::Rank;

  for (&rank_value, &count) in &hash_map {
    if count == 2 {
      // if we have 3 or 4, then it should have been handled already
      current_pair_rank = rank_value;

      if let Some(current_highest_pair_rank) = highest_two_cards {
        if current_highest_pair_rank < current_pair_rank {
          highest_two_cards = Some(current_pair_rank);
          current_pair_rank = current_highest_pair_rank;
        }
      } else {
        highest_two_cards = Some(current_pair_rank);
        continue;
      }

      if let Some(current_lower_pair_rank) = lower_two_cards {
        if current_lower_pair_rank < current_pair_rank {
          lower_two_cards = Some(current_pair_rank);
        }
      } else {
        lower_two_cards = Some(current_pair_rank);
        continue;
      }
    }
  }

  if let Some(current_high_pair_rank) = highest_two_cards {
    if let Some(current_low_pair_rank) = lower_two_cards {
      return Some(types::Combination::TwoPair(
        current_high_pair_rank,
        current_low_pair_rank,
      ));
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
  fn none_for_three_cards() {
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Seven,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

  #[test]
  fn option_for_two_pairs() {
    assert_eq!(
      Some(types::Combination::TwoPair(
        types::Rank::Seven,
        types::Rank::Five
      )),
      test(vec![
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Seven,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Seven,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

  #[test]
  fn chooses_highest_two_pairs() {
    assert_eq!(
      Some(types::Combination::TwoPair(
        types::Rank::Ace,
        types::Rank::Seven
      )),
      test(vec![
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Seven,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Seven,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

}
