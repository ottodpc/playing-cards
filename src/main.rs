#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

#[derive(Debug, Clone)]
struct Card {
    suit: String,
    value: String,
}

fn main() {
    // List of "suits"
    let suits: Vec<&str> = vec!["Hearts", "Diamonds", "Clubs", "Spades"];
    //  List of values
    let values = vec![
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        "Jack", "Queen", "King"
    ];

    let mut cards = vec![];

    for suit in suits.into_iter() {
        for value in values.iter() {
            let card = Card {
                suit: suit.to_string(),
                value: value.to_string(),
            };

            cards.push(card);
            println!("{:?}", cards);
        }
    }
    let deck: Deck = Deck { cards: vec![]};
    println!("Deck: {:?}", deck);
    println!("Deck has {} cards", deck.cards.len());
}

