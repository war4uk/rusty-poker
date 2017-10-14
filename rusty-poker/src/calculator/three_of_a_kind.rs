use std::cmp;

use hand;
use table;
use types;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  let combined_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]);

  if combined_cards.len() < 3 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&combined_cards[..]);

  let mut largest_set: Option<types::Rank> = None;

  for (&rank_value, &count) in &hash_map {
    if count == 3 {
      // if count == 4 we have a four of a kind, do not consider this.
      if let Some(current_set_rank) = largest_set {
        largest_set = Some(cmp::max(current_set_rank, rank_value));
      } else {
        largest_set = Some(rank_value);
      }
    }
  }

  if let Some(current_set_rank) = largest_set {
    return Some(types::Combination::ThreeOfAKind(current_set_rank));
  }

  None
}
