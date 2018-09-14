use calculator::WinningHand;
use std::cmp;

use calculator::utility;
use card::Card;
use types;

pub fn test(cards: Vec<Card>) -> Option<WinningHand> {
  if cards.len() < 3 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

  let mut largest_set: Option<types::Rank> = None;

  for (&rank_value, &count) in &hash_map {
    if count == 3 {
      // if count == 4 we have a four of a kind, do not consider this.
      if let Some(current_set_rank) = largest_set {
        largest_set = Some(cmp::max(current_set_rank, rank_value));
      } else {
        largest_set = Some(rank_value);
      }
    }
  }

  if let Some(current_set_rank) = largest_set {
    return Some((
      types::Combination::ThreeOfAKind(current_set_rank),
      utility::gather_cards_with_ranks(
        &[current_set_rank, current_set_rank, current_set_rank],
        &cards[..],
      ),
    ));
  }

  None
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn none_for_empty() {
    assert_eq!(None, test(vec![]));
  }

  #[test]
  fn none_for_seven_cards_without_three_of_a_kind() {
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Six,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Hearts,
        },
      ])
    );
  }

  #[test]
  fn none_for_seven_cards_with_four_of_a_kind() {
    // because of impl specifics
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Two,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Six,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Clubs,
        },
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Hearts,
        },
      ])
    );
  }

  #[test]
  fn option_for_seven_cards_with_three_of_a_kind() {
    // because of impl specifics

    let expected_combination = types::Combination::ThreeOfAKind(types::Rank::Four);
    let cards = vec![
      Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Six,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Four,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Four,
        suit: types::Suit::Clubs,
      },
      Card {
        rank: types::Rank::Four,
        suit: types::Suit::Diamonds,
      },
      Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Hearts,
      },
    ];

    let expected_cards = [
      Some(Card {
        rank: types::Rank::Four,
        suit: types::Suit::Hearts,
      }),
      Some(Card {
        rank: types::Rank::Four,
        suit: types::Suit::Clubs,
      }),
      Some(Card {
        rank: types::Rank::Four,
        suit: types::Suit::Diamonds,
      }),
      None,
      None,
    ];

    let expected_winning_hand: WinningHand = (expected_combination, expected_cards);
    assert_eq!(Some(expected_winning_hand), test(cards));
  }

  #[test]
  fn option_for_seven_cards_chooses_highest_with_three_of_a_kind() {
    // because of impl specifics
    let expected_combination = types::Combination::ThreeOfAKind(types::Rank::Jack);
    let cards = vec![
      Card {
        rank: types::Rank::Four,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Four,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Jack,
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
        rank: types::Rank::Jack,
        suit: types::Suit::Diamonds,
      },
      Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Hearts,
      },
    ];
    let expected_cards = [
      Some(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Spades,
      }),
      Some(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Diamonds,
      }),
      Some(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Hearts,
      }),
      None,
      None,
    ];
    let expected_winning_hand: WinningHand = (expected_combination, expected_cards);
    assert_eq!(Some(expected_winning_hand), test(cards));
  }
}
