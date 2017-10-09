mod types;
mod card;
mod hand;
mod table;
mod calculator;

pub use self::card::Card;
pub use self::hand::Hand;
pub use self::table::Table;
pub use calculator::Calculator;

fn main() {    
    let card1 = Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Diamonds,
    };

    let card2 = Card {
        rank: types::Rank::Four,
        suit: types::Suit::Spades,
    };

    let hand = Hand::new(card1, card2);
    let mut table = Table::new();

    table.add_card(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Spades,
    });
    table.add_card(Card {
        rank: types::Rank::Three,
        suit: types::Suit::Spades,
    });    
    table.add_card(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Spades,
    });   
    table.add_card(Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Spades,
    });      

    println!("{:?}", hand);
    println!("{:?}", table);  
    println!(" -> {:?}", Calculator::get_highest_combination(&hand, &table));  
}
