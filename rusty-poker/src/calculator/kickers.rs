use card;
use types;
use calculator::utility;

pub fn test(mut cards: Vec<card::Card>) -> types::Combination {
  utility::sort_cards(&mut cards);
  cards.truncate(5);
  return types::Combination::HighCard(cards.iter().map(|card| card.rank).collect());
}
