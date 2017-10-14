use card;
use types;
use calculator::utility;

pub fn test(sorted_cards: Vec<card::Card>) -> Option<types::Combination> {
  if sorted_cards.len() < 2 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&sorted_cards[..]);

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
