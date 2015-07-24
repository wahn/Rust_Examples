extern crate ansi_term;
extern crate rand;

use ansi_term::Colour::*;
use rand::Rng;
use std::io;

enum Card {
    ClubsAce,
    ClubsTen,
    ClubsKing,
    ClubsQueen,
    ClubsJack,
    ClubsNine,
    ClubsEight,
    ClubsSeven,
    SpadesAce,
    SpadesTen,
    SpadesKing,
    SpadesQueen,
    SpadesJack,
    SpadesNine,
    SpadesEight,
    SpadesSeven,
    HeartsAce,
    HeartsTen,
    HeartsKing,
    HeartsQueen,
    HeartsJack,
    HeartsNine,
    HeartsEight,
    HeartsSeven,
    DiamondsAce,
    DiamondsTen,
    DiamondsKing,
    DiamondsQueen,
    DiamondsJack,
    DiamondsNine,
    DiamondsEight,
    DiamondsSeven,
}

enum Game {
    Suit,
    Grand,
    Null,
}

enum Player {
    A,
    B,
    C,
}

fn bid(dealer: u8) -> (u8, u8) {
    let winner: u8 = dealer;
    let highest: u8 = 18;
    // TODO
    (winner, highest)
}

fn deal(dealer: u8) {
    // TODO
}

fn announce(player: u8) -> Game {
    Game::Suit
}

fn main() {
    // just for fun
    let clubs_jack  = "J♣"; // Emacs 'M-x insert-char' followed by BLACK CLUB SUIT
    let hearts_jack = "J♥"; // Emacs 'M-x insert-char' followed by BLACK HEART SUIT
    println!("{}", Black.bold().paint(clubs_jack));
    println!("{}", Red.bold().paint(hearts_jack));
    // randomly select player
    let player_number: u8 = rand::thread_rng().gen_range(0, 3);
    match player_number {
        0 => println!("player A:"),
        1 => println!("player B:"),
        2 => println!("player C:"),
        _ => panic!("Uknown player {}", player_number),
    }
    loop {
        // player player_number is dealing
        deal(player_number);
        // bid
        let (player, bid) = bid(player_number);
        // announce
        let game = announce(player);
        // play 10 tricks in a row
        for trick in 0..10 {
            println!("trick #{}:", trick);
            // ask for input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            // is input valid?
        };
        println!("New game?");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!("your input: {}", input);
        if input != 1 {
            break;
        }
    }
}
