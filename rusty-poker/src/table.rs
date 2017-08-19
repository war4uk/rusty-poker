use card;

#[derive(Debug)]
pub struct Table {
  cards: [Option<card::Card>; 5],
}

impl Table {
    pub fn new() -> Table {
      Table {
        cards: [None, None, None, None, None],
      }
    }
} 