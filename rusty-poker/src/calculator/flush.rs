use types;
use card::Card;
use calculator::utility;

pub fn test(cards: Vec<Card>) -> Option<types::Combination> {
  if cards.len() < 5 {
    return None;
  }

  for suit in vec![
    types::Suit::Clubs,
    types::Suit::Diamonds,
    types::Suit::Hearts,
    types::Suit::Spades,
  ] {
    if let Some(flush) = test_flush_for_suit_for_slice(suit, &cards[..]) {
      let mut result: [types::Rank; 5] = [types::Rank::Ace; 5];
      result.clone_from_slice(&flush);

      return Some(types::Combination::Flush(suit, result));
    }
  }

  return None;
}

fn test_flush_for_suit_for_slice(suit: types::Suit, cards: &[Card]) -> Option<Vec<types::Rank>> {
  let mut count = 0;
  let mut flush: Vec<Card> = vec![];
  for card in cards[..].iter() {
    if card.suit as i32 == suit as i32 {
      flush.push(*card);
      count = count + 1;
    }
  }

  utility::sort_cards(&mut flush);

  if count > 4 {
    flush.truncate(5);
    return Some(
      flush
        .iter()
        .map(|card| card.rank)
        .collect::<Vec<types::Rank>>(),
    );
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
  fn none_for_four_cards() {
    let cards = [
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Three,
        suit: types::Suit::Spades,
      },
    ];

    assert_eq!(None, test(cards.to_vec()));
  }

  #[test]
  fn option_for_five_cards() {
    let cards = [
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Three,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Spades,
      },
    ];

    assert_eq!(
      Some(types::Combination::Flush(
        types::Suit::Spades,
        [
          types::Rank::Ace,
          types::Rank::King,
          types::Rank::Queen,
          types::Rank::Three,
          types::Rank::Two
        ]
      )),
      test(cards.to_vec())
    );
  }

  #[test]
  fn option_for_seven_unsorted_cards_all_spades() {
    let cards = [
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Three,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Five,
        suit: types::Suit::Spades,
      },
    ];

    assert_eq!(
      Some(types::Combination::Flush(
        types::Suit::Spades,
        [
          types::Rank::Ace,
          types::Rank::King,
          types::Rank::Queen,
          types::Rank::Ten,
          types::Rank::Five
        ]
      )),
      test(cards.to_vec())
    );
  }

  #[test]
  fn option_for_seven_unsorted_cards_with_only_five_spades() {
    let cards = [
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Queen,
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
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Five,
        suit: types::Suit::Spades,
      },
    ];

    assert_eq!(
      Some(types::Combination::Flush(
        types::Suit::Spades,
        [
          types::Rank::King,
          types::Rank::Ten,
          types::Rank::Five,
          types::Rank::Three,
          types::Rank::Two
        ]
      )),
      test(cards.to_vec())
    );
  }

  #[test]
  fn none_for_seven_unsorted_cards_with_only_four_spades() {
    let cards = [
      Card {
        rank: types::Rank::Two,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Diamonds,
      },
      Card {
        rank: types::Rank::King,
        suit: types::Suit::Clubs,
      },
      Card {
        rank: types::Rank::Three,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Five,
        suit: types::Suit::Spades,
      },
    ];

    assert_eq!(None, test(cards.to_vec()));
  }
}
