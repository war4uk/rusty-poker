use card::Card;
use types;
use calculator::utility;

pub fn test(mut cards: Vec<Card>) -> types::Combination {
  if cards.len() == 0 {
    panic!("no cards provided")
  }

  utility::sort_cards(&mut cards);
  cards.truncate(5);
  return types::Combination::HighCard(cards.iter().map(|card| card.rank).collect());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic(expected = "no cards provided")]
  fn panic_for_empty() {
    assert_eq!(types::Combination::HighCard(vec![]), test(vec![]));
  }

  #[test]
  fn returns_five_highest_cards() {
    assert_eq!(
      types::Combination::HighCard(vec![
        types::Rank::Ace, 
        types::Rank::Ace, 
        types::Rank::Ten,
        types::Rank::Ten, 
        types::Rank::Seven
      ]),
      test(vec![
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
      ])
    );
  }
}
