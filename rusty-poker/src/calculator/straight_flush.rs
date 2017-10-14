use types;
use card;

use calculator::utility;

pub fn test(mut cards: Vec<card::Card>) -> Option<types::Combination> {
  if cards.len() < 5 {
    return None;
  }

  utility::sort_cards(&mut cards);

  let highest_five_cards = &mut cards[0..4];
  highest_five_cards.reverse();

  let suit = highest_five_cards[0].suit;
  let mut rank: i32 = highest_five_cards[0].rank as i32;

  for card in highest_five_cards.iter() {
    if suit == card.suit && rank == (card.rank as i32) {
      rank = rank - 1;
    } else {
      return None;
    }
  }

  let ranks: Vec<types::Rank> = highest_five_cards.iter().map(|x| x.rank).collect();

  let mut result: [types::Rank; 5] = [types::Rank::Ace; 5];
  result.clone_from_slice(&ranks);

  Some(types::Combination::StraightFlush(suit, result))
}
