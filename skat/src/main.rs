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

fn announce(player: u8) -> Game {
    Game::Suit
}

fn bid(dealer: u8) -> (u8, u8) {
    let winner: u8 = dealer;
    let highest: u8 = 18;
    // TODO
    (winner, highest)
}

fn deal(dealer: u8) {
    let mut cards: Vec<u8> = Vec::new();
    for n in 0..32 {
        cards.push(n);
    }
    // print clubs
    let club    = "♣";
    let spade   = "♠";
    let heart   = "♥";
    let diamond = "♦";
    for n in 0..8 {
        match n {
            0 => {
                let mut ace = "A".to_string();
                ace.push_str(club);
                print!("{} ", Black.bold().paint(&ace));
            },
            1 => {
                let mut ten = "10".to_string();
                ten.push_str(club);
                print!("{} ", Black.bold().paint(&ten));
            },
            2 => {
                let mut king = "K".to_string();
                king.push_str(club);
                print!("{} ", Black.bold().paint(&king));
            },
            3 => {
                let mut queen = "Q".to_string();
                queen.push_str(club);
                print!("{} ", Black.bold().paint(&queen));
            },
            4 => {
                let mut jack = "J".to_string();
                jack.push_str(club);
                print!("{} ", Black.bold().paint(&jack));
            },
            5 => {
                let mut nine = "9".to_string();
                nine.push_str(club);
                print!("{} ", Black.bold().paint(&nine));
            },
            6 => {
                let mut eight = "8".to_string();
                eight.push_str(club);
                print!("{} ", Black.bold().paint(&eight));
            },
            7 => {
                let mut seven = "7".to_string();
                seven.push_str(club);
                print!("{} ", Black.bold().paint(&seven));
            },
            _ => panic!("Unknown card"),
        }
    };
    println!("");
    for n in 8..16 {
        match n {
            8 => {
                let mut ace = "A".to_string();
                ace.push_str(spade);
                print!("{} ", Green.bold().paint(&ace));
            },
            9 => {
                let mut ten = "10".to_string();
                ten.push_str(spade);
                print!("{} ", Green.bold().paint(&ten));
            },
            10 => {
                let mut king = "K".to_string();
                king.push_str(spade);
                print!("{} ", Green.bold().paint(&king));
            },
            11 => {
                let mut queen = "Q".to_string();
                queen.push_str(spade);
                print!("{} ", Green.bold().paint(&queen));
            },
            12 => {
                let mut jack = "J".to_string();
                jack.push_str(spade);
                print!("{} ", Green.bold().paint(&jack));
            },
            13 => {
                let mut nine = "9".to_string();
                nine.push_str(spade);
                print!("{} ", Green.bold().paint(&nine));
            },
            14 => {
                let mut eight = "8".to_string();
                eight.push_str(spade);
                print!("{} ", Green.bold().paint(&eight));
            },
            15 => {
                let mut seven = "7".to_string();
                seven.push_str(spade);
                print!("{} ", Green.bold().paint(&seven));
            },
            _ => panic!("Unknown card"),
        }
    };
    println!("");
    for n in 16..24 {
        match n {
            16 => {
                let mut ace = "A".to_string();
                ace.push_str(heart);
                print!("{} ", Red.bold().paint(&ace));
            },
            17 => {
                let mut ten = "10".to_string();
                ten.push_str(heart);
                print!("{} ", Red.bold().paint(&ten));
            },
            18 => {
                let mut king = "K".to_string();
                king.push_str(heart);
                print!("{} ", Red.bold().paint(&king));
            },
            19 => {
                let mut queen = "Q".to_string();
                queen.push_str(heart);
                print!("{} ", Red.bold().paint(&queen));
            },
            20 => {
                let mut jack = "J".to_string();
                jack.push_str(heart);
                print!("{} ", Red.bold().paint(&jack));
            },
            21 => {
                let mut nine = "9".to_string();
                nine.push_str(heart);
                print!("{} ", Red.bold().paint(&nine));
            },
            22 => {
                let mut eight = "8".to_string();
                eight.push_str(heart);
                print!("{} ", Red.bold().paint(&eight));
            },
            23 => {
                let mut seven = "7".to_string();
                seven.push_str(heart);
                print!("{} ", Red.bold().paint(&seven));
            },
            _ => panic!("Unknown card"),
        }
    };
    println!("");
    for n in 24..32 {
        match n {
            24 => {
                let mut ace = "A".to_string();
                ace.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&ace));
            },
            25 => {
                let mut ten = "10".to_string();
                ten.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&ten));
            },
            26 => {
                let mut king = "K".to_string();
                king.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&king));
            },
            27 => {
                let mut queen = "Q".to_string();
                queen.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&queen));
            },
            28 => {
                let mut jack = "J".to_string();
                jack.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&jack));
            },
            29 => {
                let mut nine = "9".to_string();
                nine.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&nine));
            },
            30 => {
                let mut eight = "8".to_string();
                eight.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&eight));
            },
            31 => {
                let mut seven = "7".to_string();
                seven.push_str(diamond);
                print!("{} ", Yellow.bold().paint(&seven));
            },
            _ => panic!("Unknown card"),
        }
    };
    println!("");
}

fn main() {
    // randomly select player
    let player_number: u8 = rand::thread_rng().gen_range(0, 3);
    match player_number {
        0 => println!("player A:"),
        1 => println!("player B:"),
        2 => println!("player C:"),
        _ => panic!("Unknown player {}", player_number),
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
