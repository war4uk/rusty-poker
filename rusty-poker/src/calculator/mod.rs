use hand;
use table;
use types;
use card;

pub struct Calculator {

}

impl Calculator {
  pub fn get_highest_combination<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> types::Combination {
    if let Some(result) = Calculator::test_royal_flush(hand, table) {
      return result;
    }

    types::Combination::HighCard(vec![])
  }

  fn test_royal_flush<'a, 'b>(hand: &'b hand::Hand, table: &'b table::Table) -> Option<types::Combination> {
    if (table.cards_count() < 4) {
      return None
    }

    let sorted_cards = Calculator::get_sorted_cards(&hand.cards[..], &table.cards[..]);
    if (sorted_cards.len() < 5) {
      return None;
    }

    if (sorted_cards[0].rank as i32 != types::Rank::Ace as i32) {
      return None;
    }

    let suit = sorted_cards[0].suit as i32;
    let mut rank: i32 = sorted_cards[0].rank as i32;

    for card in &sorted_cards {
        if suit == (card.suit as i32) && rank == (card.rank as i32) {
          rank = rank - 1;
        } else {
          return None;
        }
    }

    let ranks: Vec<types::Rank> = sorted_cards.iter().map(|x| x.rank).collect();

    Some(types::Combination::RoyalFlush(ranks))
  }

  fn get_sorted_cards(
    hand_cards: &[card::Card], table_cards: &[Option<card::Card>]
  ) -> Vec<card::Card> {
    let mut result: Vec<card::Card> = vec![];
    result.extend(hand_cards.iter().cloned());

    for card in table_cards {
      if let Some(c) = *card {
        result.push(c);
      }
    }

    result.sort_by(|ref a, ref b| { 
      let a_rank: i32 = a.rank as i32;
      let b_rank: i32 = b.rank as i32;
      return b_rank.cmp(&a_rank);
    });

    result.truncate(5);
    result
  }
}