use card;

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