use hand;
use table;
use types;

mod straight_flush;
mod four_of_a_kind;
mod full_house;
mod flush;
mod straight;
mod three_of_a_kind;

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

    if let Some(result) = full_house::test(hand, table) {
      return result;
    }

    if let Some(result) = flush::test(hand, table) {
      return result;
    }

    if let Some(result) = straight::test(hand, table) {
      return result;
    }

    if let Some(result) = three_of_a_kind::test(hand, table) {
      return result;
    }    

    types::Combination::HighCard(vec![])
  } 
}