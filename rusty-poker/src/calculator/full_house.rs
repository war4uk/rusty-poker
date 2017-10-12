use std::cmp;

use hand;
use table;
use types;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  if table.cards_count() < 2 {
    return None
  }

  let combined_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]); 

  if combined_cards.len() < 4 {
    return None;
  }

  let hash_map = utility::get_count_hash_map(&combined_cards[..]);
  
  let mut three_cards: Option<types::Rank> = None;
  let mut two_cards: Option<types::Rank> = None;

  for (&rank_value, &count) in &hash_map {
    let mut cur_rnk = rank_value;
    if count > 2 {
      if let Some(current_three) = three_cards {
        let max_rank = cmp::max(current_three, cur_rnk);
        three_cards = Some(max_rank);
        cur_rnk = cmp::min(current_three, cur_rnk)
        // the old three can become new two
      } else {
        three_cards = Some(cur_rnk);
        continue; // no chance to change 2 cards here
      }

    }

    if count > 1 {
      if let Some(current_two) = two_cards {
        if current_two < cur_rnk {
          two_cards = Some(cur_rnk);
        }
      } else {
         two_cards = Some(cur_rnk);
      }
    }
  }

  if let Some(current_three) = three_cards {
    if let Some(current_two) = two_cards {
     return Some(types::Combination::FullHouse(vec![current_three, current_two]))
    }
  }

  return None;
}