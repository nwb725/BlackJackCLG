use rand::seq::SliceRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use rand::thread_rng;

#[derive(EnumIter, Clone, Copy, PartialEq)]
pub enum CardColor {
    Hearts,
    Clubs,
    Clover,
    Diamond,
    Empty,
}

impl CardColor {
    pub fn to_value(&self) -> &'static str {
        match self {
            CardColor::Clover  => "\u{2663}",
            CardColor::Clubs   => "\u{2660}",
            CardColor::Diamond => "\u{2666}",
            CardColor::Hearts  => "\u{2665}",
            CardColor::Empty   => "",
        }
    }
}

#[derive(EnumIter, Clone, Copy, PartialEq)]
pub enum CardType {
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
    Ace,
    Empty,
}

impl CardType {
    pub fn to_value(&self, is_ace_high: bool) -> u8 {
        match self {
            CardType::Ace => if is_ace_high { 11 } else { 1 }, // if true then 11 else 1
            CardType::Two => 2,
            CardType::Three => 3,
            CardType::Four => 4,
            CardType::Five => 5,
            CardType::Six => 6,
            CardType::Seven => 7,
            CardType::Eight => 8,
            CardType::Nine => 9,
            CardType::Ten => 10,
            CardType::Jack => 10,
            CardType::Queen => 10,
            CardType::King => 10,
            CardType::Empty => 0,
        }
    }

    pub fn to_name(&self) -> [char; 2] {
        match self {
            CardType::Ace => ['A', ' '],
            CardType::Two => ['2', ' '],
            CardType::Three => ['3', ' '],
            CardType::Four => ['4', ' '],
            CardType::Five => ['5', ' '],
            CardType::Six => ['6', ' '],
            CardType::Seven => ['7', ' '],
            CardType::Eight => ['8', ' '],
            CardType::Nine => ['9', ' '],
            CardType::Ten => ['1', '0'],
            CardType::Jack => ['J', ' '],
            CardType::Queen => ['Q', ' '],
            CardType::King => ['K', ' '],
            CardType::Empty => [' ', ' '],
        }
    }
}

// Card has the number and the color.
#[derive(Clone, Copy)]
pub struct Card {
    pub color: CardColor,
    pub value: CardType, 
    pub face_down: bool
}

impl Card {
    pub fn new(c: CardColor, val: CardType) -> Self {
        Self {
            color: c,
            value: val,
            face_down: false
        }
    }

    pub fn new_empty() -> Self {
        Self {
            color: CardColor::Empty,
            value: CardType::Empty,
            face_down: false,
        }
    }

    // Helper function.
    pub fn overwrite_card(&mut self, color: CardColor, value: CardType) {
        // When called on a card, it will overwrite the current card
        // With another card (Should be from the deck). 
        // Is also use to populate the deck.
        self.color = color;
        self.value = value;
    }

    pub fn get_value(&self) -> u8 {
        // Gets the value of the card
        self.value.to_value(true)

    }

    pub fn print_card(&self) {
        println!(
            "({}, {})", 
            self.color.to_value(), 
            self.value.to_name().iter().collect::<String>())
    }

}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new(num_decks: u8) -> Self {
        // In BlackJack you usually play with 4 decks of cards.
        // For extendability, we can specify how many decks we want: num_decks*52.
        // Then we want to randomly shuffle it.
        let mut deck: Vec<Card> = Vec::new();
        let mut current_card = Card::new(CardColor::Clubs, CardType::Ace);
        for value in CardType::iter() {
            for color in CardColor::iter() {
                if value != CardType::Empty && color != CardColor::Empty {
                    for _ in 0..num_decks {
                        current_card.overwrite_card(color, value);
                        deck.push(current_card);
                    }
                }
            }
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        Self {
            cards: deck,
        }
    }

    pub fn take_card(&mut self) -> Card {
        let c = self.cards.pop();
        match c {
            Some(card) => card,
            None       => Card::new_empty()
        }
    }

    pub fn print_deck(&self) {
        for card in &self.cards {
            card.print_card();
        }
    }

}