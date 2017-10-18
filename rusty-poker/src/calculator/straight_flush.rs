use std::collections::HashMap;

use types;
use card::Card;

use calculator::utility;
use calculator::straight;

pub fn test(mut cards: Vec<Card>) -> Option<types::Combination> {
  if cards.len() < 5 {
    return None;
  }

  utility::sort_cards(&mut cards);

  let hashmap_suit = cards.iter().fold(HashMap::new(), |mut acc, &card| {
    {
      let stat = acc.entry(card.suit).or_insert(0);
      *stat += 1
    };
    acc
  });

  let suit_for_straight_flush = hashmap_suit.iter().filter(|&(_, value)| *value > 4).next();

  if suit_for_straight_flush == None {
    return None;
  }

  if let Some((suit_for_straight_flush, _)) = suit_for_straight_flush {
    cards = cards
      .iter()
      .cloned()
      .filter(|card| card.suit == *suit_for_straight_flush)
      .collect::<Vec<Card>>();

    if let Some(straight_combination) = straight::test(cards.to_vec()) {
      if let types::Combination::Straight(straight_ranks) = straight_combination {
        return Some(types::Combination::StraightFlush(
          *suit_for_straight_flush,
          straight_ranks,
        ));
      }
    }
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
    assert_eq!(
      None,
      test(vec![
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Queen,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Ten,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }

  #[test]
  fn option_for_king_starting_straight_flush() {
    assert_eq!(
      Some(types::Combination::StraightFlush(
        types::Suit::Diamonds,
        [
          types::Rank::King,
          types::Rank::Queen,
          types::Rank::Jack,
          types::Rank::Ten,
          types::Rank::Nine
        ]
      )),
      test(vec![
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Six,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Ten,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Nine,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Queen,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }


  #[test]
  fn option_for_ace_starting_straight_flush() {
    assert_eq!(
      Some(types::Combination::StraightFlush(
        types::Suit::Diamonds,
        [
          types::Rank::Ace,
          types::Rank::King,
          types::Rank::Queen,
          types::Rank::Jack,
          types::Rank::Ten
        ]
      )),
      test(vec![
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Six,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Ten,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Ace,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Nine,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Queen,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }


  #[test]
  fn option_for_five_starting_straight_flush() {
    assert_eq!(
      Some(types::Combination::StraightFlush(
        types::Suit::Diamonds,
        [
          types::Rank::Five,
          types::Rank::Four,
          types::Rank::Three,
          types::Rank::Two,
          types::Rank::Ace
        ]
      )),
      test(vec![
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Five,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Diamonds,
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
          rank: types::Rank::Seven,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Four,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }


  #[test]
  fn option_for_king_starting_straight_flush_with_nine_other_rank() {
    assert_eq!(
      Some(types::Combination::StraightFlush(
        types::Suit::Diamonds,
        [
          types::Rank::King,
          types::Rank::Queen,
          types::Rank::Jack,
          types::Rank::Ten,
          types::Rank::Nine
        ]
      )),
      test(vec![
        Card {
          rank: types::Rank::Three,
          suit: types::Suit::Hearts,
        },
        Card {
          rank: types::Rank::King,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Jack,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Six,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Ten,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Nine,
          suit: types::Suit::Spades,
        },
        Card {
          rank: types::Rank::Nine,
          suit: types::Suit::Diamonds,
        },
        Card {
          rank: types::Rank::Queen,
          suit: types::Suit::Diamonds,
        },
      ])
    );
  }


}
