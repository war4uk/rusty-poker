use types;
use hand;
use table;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  if table.cards_count() < 4 {
    return None;
  }

  let mut sorted_cards = utility::get_sorted_cards(&hand.cards[..], &table.cards[..]);
  if sorted_cards.len() < 5 {
    return None;
  }

  let highest_five_cards = &mut sorted_cards[0..4];
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
