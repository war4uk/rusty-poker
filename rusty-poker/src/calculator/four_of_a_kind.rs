use hand;
use table;
use types;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  if table.cards_count() < 2 {
    return None;
  }

  let combined_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]);

  if combined_cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&combined_cards[..]);

  for (&rank_value, &count) in &hash_map {
    if count == 4 {
      let mut cards_of_four = combined_cards.iter().filter(|card| card.rank == rank_value);
      return Some(types::Combination::FourOfAKind(
        vec![cards_of_four.next().unwrap().rank],
      ));
    }
  }

  return None;
}
