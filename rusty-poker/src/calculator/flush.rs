use types;
use card;

pub fn test(sorted_cards: Vec<card::Card>) -> Option<types::Combination> {
  if sorted_cards.len() < 5 {
    return None;
  }

  for suit in vec![
    types::Suit::Clubs,
    types::Suit::Diamonds,
    types::Suit::Hearts,
    types::Suit::Spades,
  ] {
    if let Some(flush) = test_flush_for_suit_for_slice(suit, &sorted_cards[..]) {
      let mut result: [types::Rank; 5] = [types::Rank::Ace; 5];
      result.clone_from_slice(&flush);

      return Some(types::Combination::Flush(suit, result));
    }
  }

  return None;
}

fn test_flush_for_suit_for_slice(
  suit: types::Suit,
  sorted_cards: &[card::Card],
) -> Option<Vec<types::Rank>> {
  let mut count = 0;
  let mut flush: Vec<types::Rank> = vec![];
  for card in sorted_cards[..].iter() {
    if card.suit as i32 == suit as i32 {
      flush.push((*card).rank);
      count = count + 1;
    }
  }

  if count > 4 {
    flush.truncate(5);
    return Some(flush);
  }

  None
}
