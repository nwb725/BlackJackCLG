use crate::{Game, Player};
use std::io::Write;

fn pp_cards(p: &Player) {
    for _ in 0..p.hand_count {
        print!("+------+");
    }
    print!("\n");
    for c in 0..p.hand_count {
        if p.hand[c].face_down == true {
            print!("|+    +|");
        } else {
            print!("|    {}|", p.hand[c].value.to_name().iter().collect::<String>());
        }
    }
    print!("\n");
    for c in 0..p.hand_count {
        if p.hand[c].face_down == true {
            print!("| +  + |");
        } else {
            print!("|      |");
        }
    }
    print!("\n");
    for c in 0..p.hand_count {
        if p.hand[c].face_down == true {
            print!("|  ++  |");
        } else {
            print!("|  {}   |", p.hand[c].color.to_value());
        }
    }
    print!("\n");
    for c in 0..p.hand_count {
        if p.hand[c].face_down == true {
            print!("| +  + |");
        } else {
            print!("|      |")
        }
    }
    print!("\n");
    for c in 0..p.hand_count {
        if p.hand[c].face_down == true {
            print!("|+    +|");
        } else {
            print!("|      |")
        }
    }
    print!("\n");
    for _ in 0..p.hand_count {
        print!("+------+");
    }
}

pub fn print_game(g: &Game) {
    // Clear and flush stdout.
    print!("\x1B[2J\x1B[H");
    std::io::stdout().flush().unwrap();

    println!("DEALER:");
    pp_cards(&g.dealer);
    
    print!("\n\n");
    //println!("Cards left: {}", g.deck.cards.len());
    //pp_deck();
    
    println!("PLAYER:");
    pp_cards(&g.player);
    print!("\n")
}