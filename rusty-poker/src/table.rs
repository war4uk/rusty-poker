use card;
use std::fmt;

#[derive(Debug)]
pub struct Table {
  cards_on_table: usize,
  pub cards: [Option<card::Card>; 5],
}

impl Table {
  pub fn new() -> Table {
    Table {
      cards_on_table: 0,
      cards: [None, None, None, None, None],
    }
  }

  pub fn add_card(&mut self, card: card::Card) {
    if self.cards_on_table > 4 {
      panic!("all cards are on table");
    }
    self.cards[self.cards_on_table] = Some(card);
    self.cards_on_table = self.cards_on_table + 1;
  }

  pub fn cards_count(&self) -> usize {
    self.cards_on_table
  }
}

impl fmt::Display for Table {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    try!(write!(f, "Table:"));

    for card_option in self.cards.iter() {
      if let Some(card) = *card_option {
        try!(write!(f, " {};", card));
      }
    }

    write!(f, "")
  }
}
