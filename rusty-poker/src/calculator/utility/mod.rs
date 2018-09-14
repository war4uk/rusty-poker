use std::collections::HashMap;

use card;
use types;

pub fn sort_cards(cards: &mut Vec<card::Card>) -> () {
  cards.sort_by(|ref a, ref b| {
    let a_rank = a.rank;
    let b_rank = b.rank;
    return b_rank.cmp(&a_rank);
  });
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

pub fn combine_hand_and_table(
  hand_cards: &[card::Card],
  table_cards: &[Option<card::Card>],
) -> Vec<card::Card> {
  let mut result: Vec<card::Card> = vec![];
  result.extend(hand_cards.iter().cloned());

  for card in table_cards {
    if let Some(c) = *card {
      result.push(c);
    }
  }

  result
}

pub fn check_cards_sanity(cards: &[card::Card]) -> () {
  if cards.len() == 0 {
    panic!("no cards provided");
  }

  if cards.len() > 7 {
    panic!("too many cards: {}", cards.len());
  }

  let hash_map = get_count_hash_map(cards);

  for (&rank, &count) in &hash_map {
    if count > 4 {
      panic!("too many cards of {:?} rank: {} ", rank, count);
    }
  }

  let mut index = 1;
  for card_to_find in cards.iter() {
    for card_to_be_tested in &cards[index..] {
      if *card_to_find == *card_to_be_tested {
        panic!("two same cards found: {}", card_to_find);
      }
    }
    index = index + 1;
  }
}

pub fn gather_winning_hand(cards: &[card::Card]) -> [Option<card::Card>; 5] {
  let mut result: [Option<card::Card>; 5] = [None; 5];
  let mut index = 0;
  for card_to_find in cards.iter() {
    result[index] = Some(*card_to_find);
    index = index + 1;
  }

  return result;
}

pub fn gather_cards_with_ranks(
  ranks: &[types::Rank],
  cards: &[card::Card],
) -> [Option<card::Card>; 5] {
  let mut winning_cards: Vec<card::Card> = vec![];

  for rank in ranks.to_vec().iter() {
    let candidate = cards
      .iter()
      .find(|card| card.rank == *rank && !winning_cards.contains(card))
      .unwrap();

    winning_cards.push(*candidate);
  }

  gather_winning_hand(&winning_cards[..])
}
