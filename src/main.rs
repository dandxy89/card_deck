use rand::prelude::SliceRandom;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Suit {
    Diamonds,
    Hearts,
    Clubs,
    Spades,
}

impl Suit {
    const SUITS: [Suit; 4] = [Self::Diamonds, Self::Hearts, Self::Clubs, Self::Spades];
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Card {
    Ace(Suit),
    Number(usize, Suit),
    Jack(Suit),
    Queen(Suit),
    King(Suit),
    Joker(Suit),
}

impl Card {
    fn new(n: usize, suit: Suit) -> Self {
        match n {
            1 => Self::Ace(suit),
            2..=10 => Self::Number(n, suit),
            11 => Self::Jack(suit),
            12 => Self::Queen(suit),
            13 => Self::King(suit),
            _ => Self::Joker(suit),
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name: String = match self {
            Card::Ace(s) => format!("[Ace of {:?}]", s),
            Card::Number(n, s) => format!("[{} of {:?}]", n, s),
            Card::Jack(s) => format!("[Jack of {:?}]", s),
            Card::Queen(s) => format!("[Queen of {:?}]", s),
            Card::King(s) => format!("[King of {:?}]", s),
            Card::Joker(s) => format!("[Joker of {:?}]", s),
        };
        write!(f, "{}", name)
    }
}

fn create_deck() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(56);
    for n in 1..=14 {
        for suit in Suit::SUITS.iter() {
            cards.push(Card::new(n, *suit));
        }
    }

    cards
}

#[test]
fn test_create_deck() {
    let cards = create_deck();

    assert_eq!(&cards.len(), &56_usize);
    assert!(cards.contains(&Card::Ace(Suit::Clubs)));
    assert!(cards.contains(&Card::Number(4_usize, Suit::Clubs)));
    assert!(cards.contains(&Card::King(Suit::Clubs)));
}

fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut rand::thread_rng());
}

fn main() {
    let mut deck_of_cards: Vec<Card> = create_deck(); // Create a deck of cards
    shuffle_cards(&mut deck_of_cards); // Shuffle the deck...

    // Show me the deck!
    for c in deck_of_cards.iter() {
        println!("{}", c);
    }
}
