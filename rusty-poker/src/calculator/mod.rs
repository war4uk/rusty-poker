use hand;
use table;
use types;
use card;

mod straight_flush;
mod four_of_a_kind;
mod utility;

pub struct Calculator {

}

impl Calculator {
  pub fn get_highest_combination<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> types::Combination {
    if let Some(result) = straight_flush::test(hand, table) {
      return result;
    }

    if let Some(result) = four_of_a_kind::test(hand, table) {
      return result;
    }

    if let Some(result) = Calculator::test_straight(hand, table) {
      return result;
    }

    if let Some(result) = Calculator::test_flush(hand, table) {
      return result;
    }

    types::Combination::HighCard(vec![])
  }

  fn test_flush<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
    let sorted_cards = utility::get_sorted_cards(&hand.cards[..], &table.cards[..]);
    if sorted_cards.len() < 5 {
      return None;
    }

     for suit in vec![types::Suit::Clubs, types::Suit::Diamonds, types::Suit::Hearts, types::Suit::Spades] {
       if let Some(flush) = Calculator::test_flush_for_suit_for_slice(suit, &sorted_cards[..]) {
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

  fn test_straight<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
    if table.cards_count() < 4 {
      return None
    }

    let combined_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]); 
    
    if combined_cards.len() < 5 {
      return None;
    }
   
    if Calculator::test_ace_starting_straight(&combined_cards[..]) {
      return Some(types::Combination::Straight(vec![types::Rank::Five]));
    }

    let sorted_cards = utility::get_sorted_cards(&hand.cards[..], &table.cards[..]);
    
    for i in 0..(combined_cards.len() - 5 + 1) {      
      let slice_to_test = &sorted_cards[i..(i+5)];        
      if Calculator::test_straight_for_slice(slice_to_test) {
        return Some(types::Combination::Straight(vec![slice_to_test[0].rank]))   
      }
    }
    None 
  } 

  fn test_straight_for_slice(sorted_cards: &[card::Card]) -> bool {
    let mut prev_card = sorted_cards[0];
    for card in sorted_cards[1..].iter() {
      if prev_card.rank as i32 != (card.rank as i32 + 1) {
        return false;
      }
      prev_card = *card;
    }

    return true;
  }

  fn test_ace_starting_straight(cards: &[card::Card]) -> bool {
    let ace_starting_straight_ranks = vec![types::Rank::Ace, types::Rank::Two, types::Rank::Three, types::Rank::Four, types::Rank::Five];
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
}