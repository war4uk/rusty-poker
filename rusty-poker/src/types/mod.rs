#[derive(Debug, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum Rank {
  Two = 0,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
  Ace,
}
impl Clone for Rank {
  fn clone(&self) -> Rank {
    *self
  }
}

#[derive(Debug, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum Suit {
  Diamonds = 0,
  Clubs,
  Hearts,
  Spades,
}
impl Clone for Suit {
  fn clone(&self) -> Suit {
    *self
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Combination {
  HighCard(Vec<Rank>), // kickers from biggest to lowest. Can have variable cards
  Pair(Rank),
  TwoPair(Rank, Rank),
  ThreeOfAKind(Rank),
  Straight([Rank; 5]), // there can be a A;2;3;4;5 straight which is not trivial in calculating
  Flush(Suit, [Rank; 5]),
  FullHouse(Rank, Rank),
  FourOfAKind(Rank),
  StraightFlush(Suit, [Rank; 5]), // there can be a A;2;3;4;5 straight flush which is not trivial in calculating
}
