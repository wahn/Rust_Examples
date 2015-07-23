extern crate rand;

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
    let chars = vec![74, 226, 153, 163];
    let clubs_jack: String = String::from_utf8(chars).unwrap();
    // (see http://www.termsys.demon.co.uk/vtansi.htm#colors)
    let esc_char = vec![27];
    let esc = String::from_utf8(esc_char).unwrap();
    let bright: u8 = 1;
    let black: u8 = 30;
    let red: u8 = 31;
    println!("{}[{};{}m{}{}[0m", esc, bright, black, clubs_jack, esc);
    println!("{}[{};{}m{}{}[0m", esc, bright, red, clubs_jack, esc);
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
