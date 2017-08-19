#[derive(Debug)]
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

#[derive(Debug)]
pub enum Suit {
  Diamonds,
  Clubs,
  Hearts,
  Spades
}

#[derive(Debug)]
pub enum Combination<'a> {
  HighCard(&'a[Rank]),
  Pair(&'a[Rank]),
  TwoPair(&'a[Rank]),
  ThreeOfAKind(&'a[Rank]),
  Straight(&'a[Rank]),
  Flush(&'a[Rank]),
  FullHouse(&'a[Rank]),
  FourOfAKind(&'a[Rank]),
  StraightFlush(&'a[Rank]),
  RoyalFlush(&'a[Rank])
}