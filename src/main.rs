mod cards_and_decks;
mod pretty_printer;
use pretty_printer::*;
use cards_and_decks::*;
use core::time;
use std::io::BufRead;

const MAX_HAND: usize = 10;
const MAX_DEALER_HAND_VALUE: u8 = 17;
const BLACKJACK: u8 = 21;

enum Result {
    PlayerWin,
    DealerWin,
    Even,
    Blackjack,
}

#[allow(dead_code)]
struct Player {
    // I believe the theoritical limit of hand size is 8
    // Might need to double check that later... :)
    hand: [Card; MAX_HAND*2],
    hand_count: usize,
    hand_value: u8,
    is_standing: bool,
    is_bust: bool,
    is_split: bool,
}

impl Player {
    fn new() -> Self {
        // New player with an empty hand.
        Self {
            hand: [Card::new_empty(); MAX_HAND*2],
            hand_count: 0,
            hand_value: 0,
            is_standing: false,
            is_bust: false,
            is_split: false,
        }
    }

    fn has_blackjack(&self) -> bool {
        self.hand_count == 2 && self.hand_value == BLACKJACK 
    }

    fn hit(&mut self, deck: &mut Deck, is_dealer: bool) {
        // This is called wheneve
        // Grabs the top card of the deck, add to the hand, 
        // and checks what 'kind' of hand the player has.
        let card = deck.take_card();
        self.hand[self.hand_count] = card;
        self.hand_count += 1;


        match card.value {
            CardType::Ace => {
                if is_dealer {
                    if 17 <= (self.hand_value + 11) && (self.hand_value + 11) < 21 {
                        self.hand_value += card.get_value(true)
                    } else {
                        self.hand_value += card.get_value(false)
                    }
                } else {
                    if (self.hand_value + 11) <= BLACKJACK {
                        self.hand_value += card.get_value(true)
                    } else {
                        self.hand_value += card.get_value(false)
                    }
                }
            }
            _=> { self.hand_value += card.get_value(false) }
        }

        // Should never be above max hand.
        if self.hand_count == MAX_HAND {
            self.is_standing = true;
            return;
        }

        if self.hand_value > 21 {
            self.is_bust = true;
            return;
        }
    }

    fn stand(&mut self) {
        // Something when very wrong if this assert fails.
        assert_ne!(self.is_standing, true);

        // Should imply that the player is done,
        // And the dealer should finish.
        self.is_standing = true;
    }

    fn can_split(&self) -> bool {
        self.hand_count == 2 && self.hand[0] == self.hand[1] 
    }

    #[allow(dead_code)]
    fn split(&mut self) {
        // TODO: Make it so the player can split their hand.
        // Only possible if first 2 cards a identical.

        // should only ever be allowed at the beginning.
        // If this fails, then look at can_split.
        assert_eq!(self.hand_count, 2);

        self.is_split = true;
        self.hand[MAX_HAND] = self.hand[1];
        self.hand[1] = Card::new_empty();
        todo!()
    }
}

struct Game {
    player: Player,
    dealer: Player,
    deck: Deck,
}

impl Game {
    fn new() -> Self {
        let d = Deck::new(4);
        /* 
        To test split
        d.cards[0] = Card::new(CardColor::Diamond, CardType::King);
        d.cards[2] = Card::new(CardColor::Clover, CardType::King); 
        */
        Self {
            player: Player::new(),
            dealer: Player::new(),
            // 4 decks, can change
            deck:   d,
        }
    }

    fn start() -> Self {
        let mut game = Game::new();


        // Mixed starting order as in the real game.
        game.player.hit(&mut game.deck, false);
        game.dealer.hit(&mut game.deck, true);

        game.player.hit(&mut game.deck, false);
        game.dealer.hit(&mut game.deck, true);

        game.dealer.hand[1].face_down = true;

        game
    }

    fn dealer_play(&mut self) {
        if self.player.is_bust || self.player.has_blackjack() || self.dealer.has_blackjack() {
            return;
        }
        self.dealer.hand[1].face_down = false;

        loop {
            if self.dealer.hand_value > BLACKJACK {
                self.dealer.is_bust = true;
                break;
            }
    
            if self.dealer.hand_value >= MAX_DEALER_HAND_VALUE {
                self.dealer.is_standing = true;
                break;
            }

            self.dealer.hit(&mut self.deck, true);
        }
    }

    fn check_winner(&mut self) -> Option<Result> {
        if self.player.is_bust {
            return Some(Result::DealerWin)
        }
        if self.player.has_blackjack() && !self.dealer.has_blackjack() {
            return Some(Result::Blackjack)
        }
        if self.player.has_blackjack() && self.dealer.has_blackjack() {
            return Some(Result::Even);
        }
        if self.dealer.has_blackjack() {
            return Some(Result::DealerWin);
        }
        if self.dealer.is_bust || self.player.hand_value > self.dealer.hand_value {
            return Some(Result::PlayerWin)
        }
        if self.player.hand_value == self.dealer.hand_value {
            return Some(Result::Even)
        }
        if self.player.hand_value < self.dealer.hand_value {
            return Some(Result::DealerWin)
        }
        None
    }
}

fn main() {
    let mut command;
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    loop {
        let mut g = Game::start();
        loop {
            print_game(&g);
            if g.player.has_blackjack() {
                break;
            }
            let mut input = String::new();
            println!("\nEnter your move!");
            println!("(1) Hit");
            println!("(2) Stand");
            if g.player.can_split() { println!("(3) Split") }
            println!("(4) Exit");
            match handle.read_line(&mut input) {
                Ok(0) => return, // EOF detected, exit the program, otherwise the dealer plays.
                Ok(_) => {
                    // Parse command as u8, for now assume that user is perfekt :)
                    command = input.trim().parse::<u8>().unwrap_or(100)
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
            match command {
                1 => { 
                    g.player.hit(&mut g.deck, false); 
                    g.player.hand[g.player.hand_count-1].print_card();
                    if g.player.is_bust {
                        // Dealers turn.
                        break;
                    }
                },
                2 => { g.player.stand(); break;},
                3 => { if g.player.can_split() { 
                    // Do the split stuff
                    g.player.split();
                }},  
                4 => return,
                100 => (), // Unkown command, nothing happens, Might need to give message later.
                _ => panic!("Panic: Something went very wrong parsing the command.")
            }
        }
        g.dealer_play();
        let game_result = g.check_winner().unwrap();
        print_game(&g);
        print!("\nGame result: ");
        match game_result {
            Result::PlayerWin => println!("Player won normally"),
            Result::DealerWin => println!("Dealer won normally"),
            Result::Even      => println!("Game ended even"),
            Result::Blackjack => println!("Black jack baby!"),
        }
        let dur = time::Duration::from_millis(2000);
        std::thread::sleep(dur);
        
    }
}
