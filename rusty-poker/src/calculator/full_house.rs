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
    println!("{:?}-{:?}", rank_value, count);
     // todo -
  }

  println!("---{:?}-{:?}", three_cards, two_cards);
  if let Some(currentThree) = three_cards {
    if let Some(currentTwo) = two_cards {
      println!("heeeeey");
    }
  }

  return None;
}