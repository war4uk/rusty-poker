use std::collections::HashMap;

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

  let hash_map = combined_cards.iter().fold(HashMap::new(), |mut acc, &card| {
      { 
        let stat = acc.entry(card.rank as i32).or_insert(0);
      *stat += 1;
      }
    acc
  });

  for (&rank_value, &count) in &hash_map {
      if count == 4 {
        let mut cards_of_four = combined_cards.iter().filter(|card| { card.rank as i32 == rank_value });
      return Some(types::Combination::FourOfAKind(vec![cards_of_four.next().unwrap().rank]));  
      }  
  }

  return None;
}