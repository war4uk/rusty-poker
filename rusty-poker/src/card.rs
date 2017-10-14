use types;

#[derive(Debug, Copy)]
pub struct Card {
  pub rank: types::Rank,
  pub suit: types::Suit,
}

impl Clone for Card {
  fn clone(&self) -> Card {
    *self
  }
}
