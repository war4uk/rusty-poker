use types;
use card;

use calculator::utility;

pub fn test(mut cards: Vec<card::Card>) -> Option<types::Combination> {
  if cards.len() < 5 {
    return None;
  }

  utility::sort_cards(&mut cards);

  if test_ace_starting_straight(&cards[..]) {
    return Some(types::Combination::Straight([
      types::Rank::Five,
      types::Rank::Four,
      types::Rank::Three,
      types::Rank::Two,
      types::Rank::Ace,
    ]));
  }

  for i in 0..(cards.len() - 5 + 1) {
    let slice_to_test = &cards[i..(i + 5)];
    if test_straight_for_slice(slice_to_test) {
      let rank_vector = &slice_to_test
        .into_iter()
        .map(|&card| card.clone().rank)
        .collect::<Vec<types::Rank>>();

      let mut result: [types::Rank; 5] = [types::Rank::Ace; 5];
      result.clone_from_slice(rank_vector);
      return Some(types::Combination::Straight(result));
    }
  }
  None
}

fn test_straight_for_slice(cards: &[card::Card]) -> bool {
  let mut prev_card = cards[0];
  for card in cards[1..].iter() {
    if prev_card.rank as i32 != (card.rank as i32 + 1) {
      return false;
    }
    prev_card = *card;
  }

  return true;
}

fn test_ace_starting_straight(cards: &[card::Card]) -> bool {
  let ace_starting_straight_ranks = vec![
    types::Rank::Ace,
    types::Rank::Two,
    types::Rank::Three,
    types::Rank::Four,
    types::Rank::Five,
  ];
  for rank_in_straight in ace_starting_straight_ranks.iter() {
    let mut was_found = false;

    for card_in_hand in cards.iter() {
      if card_in_hand.rank as i32 == *rank_in_straight as i32 {
        was_found = true;
        break;
      }
    }

    if !was_found {
      return false;
    }
  }

  return true;
}
