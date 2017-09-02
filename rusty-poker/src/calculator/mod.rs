use hand;
use table;
use types;
use card;

pub struct Calculator {

}

impl Calculator {
  pub fn get_highest_combination<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> types::Combination {
    if let Some(result) = Calculator::test_straight_flush(hand, table) {
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
    let sorted_cards = Calculator::get_sorted_cards(&hand.cards[..], &table.cards[..]);
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

    let combined_cards = Calculator::combine_hand_and_table(&hand.cards[..], &table.cards[..]); 
    
    if combined_cards.len() < 5 {
      return None;
    }
   
    if Calculator::test_ace_starting_straight(&combined_cards[..]) {
      return Some(types::Combination::Straight(vec![types::Rank::Five]));
    }

    let sorted_cards = Calculator::get_sorted_cards(&hand.cards[..], &table.cards[..]);
    
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

  fn test_straight_flush<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
    if table.cards_count() < 4 {
      return None
    }
    
    let mut sorted_cards = Calculator::get_sorted_cards(&hand.cards[..], &table.cards[..]);
    if sorted_cards.len() < 5 {
      return None;
    }

    let mut highest_five_cards = &mut sorted_cards[0..4];
    highest_five_cards.reverse();

    let suit = highest_five_cards[0].suit as i32;
    let mut rank: i32 = highest_five_cards[0].rank as i32;

    for card in highest_five_cards.iter() {
        if suit == (card.suit as i32) && rank == (card.rank as i32) {
          rank = rank - 1;
        } else {
          return None;
        }
    }

    let ranks: Vec<types::Rank> = highest_five_cards.iter().map(|x| x.rank).collect();

    Some(types::Combination::StraightFlush(ranks))
  }

  fn combine_hand_and_table(hand_cards: &[card::Card], table_cards: &[Option<card::Card>]) ->  Vec<card::Card> {
    let mut result: Vec<card::Card> = vec![];
    result.extend(hand_cards.iter().cloned());

    for card in table_cards {
      if let Some(c) = *card {
        result.push(c);
      }
    }

    result
  }

  fn get_sorted_cards(
    hand_cards: &[card::Card], table_cards: &[Option<card::Card>]
  ) -> Vec<card::Card> {
    let mut result: Vec<card::Card> = Calculator::combine_hand_and_table(hand_cards, table_cards);
   
    result.sort_by(|ref a, ref b| { 
      let a_rank: i32 = a.rank as i32;
      let b_rank: i32 = b.rank as i32;
      return b_rank.cmp(&a_rank);
    });

    result
  }
}