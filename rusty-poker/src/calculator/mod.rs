use hand;
use table;
use types;

mod straight_flush;
mod four_of_a_kind;
mod full_house;
mod flush;
mod straight;
mod three_of_a_kind;
mod two_pairs;
mod pair;
mod kickers;

mod utility;

pub struct Calculator {}

impl Calculator {
  pub fn get_highest_combination<'a, 'b>(
    hand: &'b hand::Hand,
    table: &'b table::Table,
  ) -> types::Combination {
    let sorted_cards = utility::combine_hand_and_table(&hand.cards[..], &table.cards[..]);

    if let Some(result) = straight_flush::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = four_of_a_kind::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = full_house::test(sorted_cards.clone()) {
      return result;
    }

    if let Some(result) = flush::test(sorted_cards.clone()) {
      return result;
    }

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
