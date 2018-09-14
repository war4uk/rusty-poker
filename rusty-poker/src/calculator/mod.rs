use card;
use hand;
use table;
use types;

mod flush;
mod four_of_a_kind;
mod full_house;
mod kickers;
mod pair;
mod straight;
mod straight_flush;
mod three_of_a_kind;
mod two_pairs;

mod utility;

pub struct Calculator {}

type WinningHand = (types::Combination, [Option<card::Card>; 5]);

impl Calculator {
  pub fn get_winning_hand<'a>(hand: &'a hand::Hand, table: &'a table::Table) -> WinningHand {
    let sorted_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]);

    utility::check_cards_sanity(&sorted_cards);

    if let Some(result) = straight_flush::test(sorted_cards.clone()) {
      return result;
    }
    /*
    if let Some(result) = four_of_a_kind::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = full_house::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = flush::test(sorted_cards.clone()) {
      return result;
    }
    */
    if let Some(result) = straight::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = three_of_a_kind::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = two_pairs::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = pair::test(sorted_cards.clone()) {
      return result;
    }

    return kickers::test(sorted_cards);
  }
}
