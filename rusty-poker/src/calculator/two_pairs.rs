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

  let mut highest_two_cards: Option<types::Rank> = None;
  let mut lower_two_cards: Option<types::Rank> = None;

  let mut current_pair_rank: types::Rank;

  for (&rank_value, &count) in &hash_map {
    if count == 2 {
      // if we have 3 or 4, then it should have been handled already
      current_pair_rank = rank_value;

      if let Some(current_highest_pair_rank) = highest_two_cards {
        if current_highest_pair_rank < current_pair_rank {
          highest_two_cards = Some(current_pair_rank);
          current_pair_rank = current_highest_pair_rank;
        }
      } else {
        highest_two_cards = Some(current_pair_rank);
        continue;
      }

      if let Some(current_lower_pair_rank) = lower_two_cards {
        if current_lower_pair_rank < current_pair_rank {
          lower_two_cards = Some(current_pair_rank);
        }
      } else {
        lower_two_cards = Some(current_pair_rank);
        continue;
      }
    }
  }

  if let Some(current_high_pair_rank) = highest_two_cards {
    if let Some(current_low_pair_rank) = lower_two_cards {
      return Some(types::Combination::TwoPair(
        vec![current_high_pair_rank, current_low_pair_rank],
      ));
    }
  }

  return None;
}
