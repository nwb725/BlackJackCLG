mod cards_and_decks;
mod pretty_printer;
use pretty_printer::*;
use cards_and_decks::*;
use core::time;
use std::io::BufRead;

const MAX_HAND: usize = 8;
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
    hand: [Card; MAX_HAND],
    hand_count: usize,
    hand_value: u8,
    is_standing: bool,
    is_bust: bool,
}

impl Player {
    fn new() -> Self {
        // New player with an empty hand.
        Self {
            hand: [Card::new_empty(); MAX_HAND],
            hand_count: 0,
            hand_value: 0,
            is_standing: false,
            is_bust: false,
        }
    }

    fn is_bust(&self) -> bool {
        self.is_bust
    }

    fn has_blackjack(&self) -> bool {
        if self.hand_count == 2 && self.hand_value == BLACKJACK {
            true
        }
        else { false }
    }

    fn set_player_value(&mut self) {
        // Sets the players value to the current value
        let mut sum = 0;
        for i in &self.hand {
            sum += i.get_value()
        }
        self.hand_value = sum;
    }

    fn get_player_cards(&self) -> Vec<Card> {
        let mut r = Vec::new();
        for i in 0..self.hand_count {
            r.push(self.hand[i]);
        }
        r
    }

    fn hit(&mut self, deck: &mut Deck) {
        // This is called wheneve
        // Grabs the top card of the deck, add to the hand, 
        // and checks what 'kind' of hand the player has.
        let card = deck.take_card();
        self.hand[self.hand_count] = card;
        self.hand_count += 1;
        self.set_player_value();

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
        self.set_player_value();
        self.is_standing = true;
    }

    #[warn(dead_code)]
    fn split() {
        // TODO: Make it so the player can split their hand.
        // Need to look at the rules.
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
        Self {
            player: Player::new(),
            dealer: Player::new(),
            deck:   Deck::new(4),
        }
    }

    fn start() -> Self {
        let mut game = Game::new();

        game.player.hit(&mut game.deck);
        game.player.hit(&mut game.deck);

        game.dealer.hit(&mut game.deck);
        game.dealer.hit(&mut game.deck); 
        


        println!("INITAL DEALER CARDS");
        for i in game.dealer.get_player_cards() {
            i.print_card();
        }

        println!("INITIAL PLAYER CARDS");
        for i in game.player.get_player_cards() {
            i.print_card();
        }

        game
    }

    fn dealer_play(&mut self) {
        if self.player.is_bust() || self.player.has_blackjack() || self.dealer.has_blackjack() {
            return;
        }

        loop {
            if self.dealer.hand_value > BLACKJACK {
                self.dealer.is_bust = true;
                break;
            }
    
            if self.dealer.hand_value >= MAX_DEALER_HAND_VALUE {
                self.dealer.is_standing = true;
                break;
            }

            self.dealer.hit(&mut self.deck);
        }
    }

    fn check_winner(&mut self) -> Option<Result> {
        if self.player.is_bust() {
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
        if self.dealer.is_bust() || self.player.hand_value > self.dealer.hand_value {
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
            match handle.read_line(&mut input) {
                Ok(0) => return, // EOF detected, exit the program, otherwise the dealer plays.
                Ok(_) => {
                    // Parse command as u8, for now assume that user is perfekt :)
                    command = input.trim().parse::<u8>().expect("User was WRONG FIX!")
                }
                Err(err) => {
                    eprintln!("Error reading input: {}", err);
                    break;
                }
            }
            match command {
                1 => { 
                    g.player.hit(&mut g.deck); 
                    g.player.hand[g.player.hand_count-1].print_card();
                    if g.player.is_bust() {
                        // Dealers turn.
                        break;
                    }
                },
                2 => { g.player.stand(); break;},
                _ => (),
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
