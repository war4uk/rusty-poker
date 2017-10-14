use hand;
use table;
use types;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  let combined_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]);

  if combined_cards.len() < 2 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&combined_cards[..]);

  let mut current_pair: Option<types::Rank> = None;

  for (&rank_value, &count) in &hash_map {
    if count == 2 {
      // if we have 3 or 4, then it should have been handled already
      if let Some(current_pair_rank) = current_pair {
        if current_pair_rank < rank_value {
          current_pair = Some(rank_value);
        }
      } else {
        current_pair = Some(rank_value);
      }
    }
  }

  if let Some(current_pair_rank) = current_pair {
    return Some(types::Combination::Pair(current_pair_rank));
  }

  return None;
}
