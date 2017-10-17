use std::cmp;

use card::Card;
use types;
use calculator::utility;

pub fn test(cards: Vec<Card>) -> Option<types::Combination> {
  if cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

  let mut three_cards: Option<types::Rank> = None;
  let mut two_cards: Option<types::Rank> = None;

  for (&rank_value, &count) in &hash_map {
    let mut cur_rnk = rank_value;
    if count > 2 {
      if let Some(current_three) = three_cards {
        let max_rank = cmp::max(current_three, cur_rnk);
        three_cards = Some(max_rank);
        cur_rnk = cmp::min(current_three, cur_rnk)
      // the old three can become new two
      } else {
        three_cards = Some(cur_rnk);
        continue; // no chance to change 2 cards here
      }
    }

    if count > 1 {
      if let Some(current_two) = two_cards {
        if current_two < cur_rnk {
          two_cards = Some(cur_rnk);
        }
      } else {
        two_cards = Some(cur_rnk);
      }
    }
  }

  if let Some(current_three) = three_cards {
    if let Some(current_two) = two_cards {
      return Some(types::Combination::FullHouse(current_three, current_two));
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
  fn none_for_four_cards() {
    assert_eq!(
      None,
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
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

  #[test]
  fn none_for_five_duplcate_cards() {
    assert_eq!(
      None,
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
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
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
  fn option_for_three_and_two_duplcate_cards() {
    assert_eq!(
      Some(types::Combination::FullHouse(
        types::Rank::Two,
        types::Rank::Three
      )),
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
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

  #[test]
  fn option_for_fullhouse_with_three_twos_and_two_jacks() {
    assert_eq!(
      Some(types::Combination::FullHouse(
        types::Rank::Two,
        types::Rank::Jack
      )),
      test(vec![
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
      ])
    );
  }

  #[test]
  fn option_for_fullhouse_with_three_jacks_and_two_twos() {
    assert_eq!(
      Some(types::Combination::FullHouse(
        types::Rank::Jack,
        types::Rank::Two
      )),
      test(vec![
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Spades,
        },
      ])
    );
  }

}
