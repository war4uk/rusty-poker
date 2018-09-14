use calculator::utility;
use calculator::WinningHand;
use card::Card;
use types;

pub fn test(mut cards: Vec<Card>) -> WinningHand {
  if cards.len() == 0 {
    panic!("no cards provided")
  }

  utility::sort_cards(&mut cards);
  cards.truncate(5);

  return (
    types::Combination::HighCard(cards.iter().map(|card| card.rank).collect()),
    utility::gather_winning_hand(&cards[..]),
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "no cards provided")]
  fn panic_for_empty() {
    test(vec![]);
  }

  #[test]
  fn returns_five_highest_cards() {
    let expected_combination = types::Combination::HighCard(vec![
      types::Rank::Ace,
      types::Rank::Ace,
      types::Rank::Ten,
      types::Rank::Ten,
      types::Rank::Seven,
    ]);

    let cards = vec![
      Card {
        rank: types::Rank::Seven,
        suit: types::Suit::Diamonds,
      },
      Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Spades,
      },
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Hearts,
      },
      Card {
        rank: types::Rank::Three,
        suit: types::Suit::Clubs,
      },
      Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Diamonds,
      },
    ];

    let expected_cards = [
      Some(Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
      }),
      Some(Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Diamonds,
      }),
      Some(Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Spades,
      }),
      Some(Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Hearts,
      }),
      Some(Card {
        rank: types::Rank::Seven,
        suit: types::Suit::Diamonds,
      }),
    ];

    let expected_winning_hand: WinningHand = (expected_combination, expected_cards);
    assert_eq!(expected_winning_hand, test(cards));
  }
}
