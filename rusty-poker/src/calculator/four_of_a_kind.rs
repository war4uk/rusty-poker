use card;
use types;
use calculator::utility;

pub fn test(sorted_cards: Vec<card::Card>) -> Option<types::Combination> {
  if sorted_cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&sorted_cards[..]);

  for (&rank_value, &count) in &hash_map {
    if count == 4 {
      return Some(types::Combination::FourOfAKind(rank_value));
    }
  }

  return None;
}
