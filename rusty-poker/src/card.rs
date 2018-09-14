use std::fmt;
use types;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
  pub rank: types::Rank,
  pub suit: types::Suit,
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?} of {:?}", self.rank, self.suit)
  }
}
