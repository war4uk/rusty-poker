use std::cmp;

use card;
use types;
use calculator::utility;

pub fn test(cards: Vec<card::Card>) -> Option<types::Combination> {
  if cards.len() < 3 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&cards[..]);

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
