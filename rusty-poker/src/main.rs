#[macro_use]
extern crate num_derive;
extern crate num_traits;

mod calculator;
mod card;
mod cards_iterator;
mod hand;
mod table;
mod types;

pub use self::card::Card;
pub use self::hand::Hand;
pub use self::table::Table;
pub use calculator::Calculator;

fn main() {
    let card1 = Card {
        rank: types::Rank::Three,
        suit: types::Suit::Diamonds,
    };

    let card2 = Card {
        rank: types::Rank::King,
        suit: types::Suit::Hearts,
    };

    let hand = Hand::new(card1, card2);
    let mut table = Table::new();

    table.add_card(Card {
        rank: types::Rank::Queen,
        suit: types::Suit::Hearts,
    });
    table.add_card(Card {
        rank: types::Rank::Jack,
        suit: types::Suit::Hearts,
    });
    table.add_card(Card {
        rank: types::Rank::Ten,
        suit: types::Suit::Diamonds,
    });
    table.add_card(Card {
        rank: types::Rank::Ace,
        suit: types::Suit::Hearts,
    });

    let cloned_table = table.clone();

    println!("hand: {}", hand);
    println!("table: {}", table);

    println!(
        "combination -> {:?}",
        Calculator::get_winning_hand(&hand, &table)
    );

    /*let cards_iterator = cards_iterator::CardsIterator::new();
    let mut counter = 0;
    for card in cards_iterator {
        let mut current_table = cloned_table.clone();
        if !hand.cards.contains(&card) && !table.cards.contains(&Some(card)) {
            current_table.add_card(card);
            println!("{} -> hand: {}", counter + 1, hand);
            println!("{} -> table: {}", counter + 1, current_table);
            println!(
                "{} -> {:?}",
                counter + 1,
                Calculator::get_highest_combination(&hand, &current_table)
            );
        } else {
            println!("{} -> already in game: {}", counter + 1, card);
        }

        counter = counter + 1;
    }

    println!("total iterations: {}", counter);*/
}
