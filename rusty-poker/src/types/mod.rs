#[derive(Debug)]
#[derive(Copy)]
pub enum Rank {
  Two,
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
  Ace
}
impl Clone for Rank {
    fn clone(&self) -> Rank { *self }
}

#[derive(Debug)]
#[derive(Copy)]
pub enum Suit {
  Diamonds,
  Clubs,
  Hearts,
  Spades
}
impl Clone for Suit {
    fn clone(&self) -> Suit { *self }
}

#[derive(Debug)]
pub enum Combination {
  HighCard(Vec<Rank>),
  Pair(Vec<Rank>),
  TwoPair(Vec<Rank>),
  ThreeOfAKind(Vec<Rank>),
  Straight(Vec<Rank>),
  Flush(Vec<Rank>),
  FullHouse(Vec<Rank>),
  FourOfAKind(Vec<Rank>),
  StraightFlush(Vec<Rank>),
  RoyalFlush(Vec<Rank>)
}
