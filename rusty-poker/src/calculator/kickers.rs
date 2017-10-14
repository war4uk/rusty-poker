use card;
use types;

pub fn test<'a, 'b>(mut sorted_cards: Vec<card::Card>) -> types::Combination {
  sorted_cards.truncate(5);
  return types::Combination::HighCard(sorted_cards.iter().map(|card| card.rank).collect());
}
