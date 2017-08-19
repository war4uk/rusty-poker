mod types;
mod card;
mod hand;
mod table;

pub use self::card::Card;
pub use self::hand::Hand;
pub use self::table::Table;

fn main() {    
    let card1 = Card {
        rank: types::Rank::Five,
        suit: types::Suit::Diamonds,
    };

    let card2 = Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Spades,
    };

    println!("{:?}", Hand::new(card1, card2));
    println!("{:?}", Table::new());    
}
