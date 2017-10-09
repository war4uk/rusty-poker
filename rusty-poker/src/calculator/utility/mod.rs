use std::collections::HashMap;

use card; 
use types;

pub fn get_sorted_cards(
  hand_cards: &[card::Card], table_cards: &[Option<card::Card>]
) -> Vec<card::Card> {
  let mut result: Vec<card::Card> = combine_hand_and_table(hand_cards, table_cards);
  
  result.sort_by(|ref a, ref b| { 
    let a_rank: i32 = a.rank as i32;
    let b_rank: i32 = b.rank as i32;
    return b_rank.cmp(&a_rank);
  });

  result
}

pub fn combine_hand_and_table(hand_cards: &[card::Card], table_cards: &[Option<card::Card>]) ->  Vec<card::Card> {
  let mut result: Vec<card::Card> = vec![];
  result.extend(hand_cards.iter().cloned());

  for card in table_cards {
    if let Some(c) = *card {
      result.push(c);
    }
  }

  result
}

pub fn get_count_hash_map(cards: &[card::Card]) -> HashMap<types::Rank, i32> {

  cards.iter().fold(HashMap::new(), |mut acc, &card| {
      { 
        let stat = acc.entry(card.rank).or_insert(0);
        *stat += 1;
      }
    acc
  })
} 
