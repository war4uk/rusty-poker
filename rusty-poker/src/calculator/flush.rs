use hand;
use table;
use types;
use card;
use calculator::utility;

pub fn test<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
  let sorted_cards = utility::get_sorted_cards(&hand.cards[..], &table.cards[..]);
  if sorted_cards.len() < 5 {
    return None;
  }

    for suit in vec![types::Suit::Clubs, types::Suit::Diamonds, types::Suit::Hearts, types::Suit::Spades] {
      if let Some(flush) = test_flush_for_suit_for_slice(suit, &sorted_cards[..]) {
        return Some(types::Combination::Flush(flush));
      }       
    }

    return None;
}

fn test_flush_for_suit_for_slice(suit: types::Suit, sorted_cards: &[card::Card]) -> Option<Vec<types::Rank>> { 
  let mut count = 0;
  let mut flush: Vec<types::Rank> = vec![];
  for card in sorted_cards[..].iter() {
    if card.suit as i32 == suit as i32 {
      flush.push((*card).rank);
      count = count + 1;
    }
  }

  if count > 4 {
    flush.truncate(5);
    return Some(flush);
  } 

  None
}