use card;
use num_traits::FromPrimitive;
use num_traits::ToPrimitive;
use types;

pub struct CardsIterator {
  current_rank: types::Rank,
  current_suit: types::Suit,
  is_depleted: bool,
}

impl CardsIterator {
  pub fn new() -> CardsIterator {
    let starting_rank: types::Rank = match FromPrimitive::from_u8(0) {
      Some(rank) => rank,
      None => FromPrimitive::from_u8(0).unwrap(),
    };

    let starting_suit: types::Suit = match FromPrimitive::from_u8(0) {
      Some(suit) => suit,
      None => FromPrimitive::from_u8(0).unwrap(),
    };

    CardsIterator {
      current_rank: starting_rank,
      current_suit: starting_suit,
      is_depleted: false,
    }
  }
}

impl Iterator for CardsIterator {
  type Item = card::Card;

  // next() is the only required method
  fn next(&mut self) -> Option<card::Card> {
    if self.is_depleted {
      return None;
    }
    // increment our count. This is why we started at zero.
    let result = Some(card::Card {
      rank: self.current_rank,
      suit: self.current_suit,
    });

    let rank_uint = ToPrimitive::to_u8(&self.current_rank).unwrap();
    let suit_uint = ToPrimitive::to_u8(&self.current_suit).unwrap();

    let next_rank: types::Rank = match FromPrimitive::from_u8(rank_uint + 1) {
      Some(rank) => rank,
      None => {
        let next_suit: types::Suit = match FromPrimitive::from_u8(suit_uint + 1) {
          Some(suit) => suit,
          None => {
            self.is_depleted = true;
            FromPrimitive::from_u8(0).unwrap()
          }
        };

        self.current_suit = next_suit;
        FromPrimitive::from_u8(0).unwrap()
      }
    };

    self.current_rank = next_rank;

    result
  }
}
