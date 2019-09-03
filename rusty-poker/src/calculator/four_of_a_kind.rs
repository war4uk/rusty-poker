use calculator::utility;
use calculator::WinningHand;
use card::Card;
use types;

pub fn test(cards: Vec<Card>) -> Option<WinningHand> {
  if cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

  let mut four_of_a_kind_cards = [None, None, None, None, None];
  let mut four_of_a_kind_rank = None;

  for (&rank_value, &count) in &hash_map {
    if count == 4 {
      four_of_a_kind_rank = Some(rank_value);
      four_of_a_kind_cards = [
        None,
        Some(Card {
          rank: rank_value,
          suit: types::Suit::Diamonds,
        }),
        Some(Card {
          rank: rank_value,
          suit: types::Suit::Clubs,
        }),
        Some(Card {
          rank: rank_value,
          suit: types::Suit::Hearts,
        }),
        Some(Card {
          rank: rank_value,
          suit: types::Suit::Spades,
        }),
      ];
    }
  }

  if let Some(rank) = four_of_a_kind_rank {
    let mut kicker: Option<Card> = None;

    for (_, elem) in cards.iter().enumerate() {
      if elem.rank == rank {
        continue;
      }

      if let Some(current_kicker) = kicker {
        if current_kicker.rank < elem.rank {
          kicker = Some(elem.clone());
        }
      } else {
        kicker = Some(elem.clone());
      }
    }

    four_of_a_kind_cards[0] = kicker;

    return Some((types::Combination::FourOfAKind(rank), four_of_a_kind_cards));
  } else {
    return None;
  }
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
    let expected_combination = types::Combination::FourOfAKind(types::Rank::Two);

    let cards = vec![
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
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Clubs,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Diamonds,
      },
    ];

    let expected_cards = [
      Some(Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      }),
      Some(Card {
        rank: types::Rank::Two,
        suit: types::Suit::Diamonds,
      }),
      Some(Card {
        rank: types::Rank::Two,
        suit: types::Suit::Clubs,
      }),
      Some(Card {
        rank: types::Rank::Two,
        suit: types::Suit::Hearts,
      }),
      Some(Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      }),
    ];

    let expected_winning_hand: WinningHand = (expected_combination, expected_cards);

    assert_eq!(
      Some(expected_winning_hand),
      test(cards)
    );
  }
}
