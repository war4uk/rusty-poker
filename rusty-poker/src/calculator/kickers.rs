use hand;
use table;
use types;

use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> types::Combination {
  let mut high_cards = utility::get_sorted_cards(&hand.cards[..], &table.cards[..]);
  high_cards.truncate(5);
  return types::Combination::HighCard(high_cards.iter().map(|card| card.rank).collect());
}
