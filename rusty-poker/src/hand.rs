use card;
use std::fmt;

#[derive(Debug)]
pub struct Hand {
  pub cards: [card::Card; 2],
}

impl Hand {
  pub fn new(card1: card::Card, card2: card::Card) -> Hand {
    Hand {
      cards: [card1, card2],
    }
  }
}

impl fmt::Display for Hand {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Hand: {}; {};", self.cards[0], self.cards[1])
  }
}
