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

enum Player {
    A,
    B,
    C,
}

fn main() {
    // randomly select player
    let player_number = rand::thread_rng().gen_range(0, 3);
    match player_number {
        0 => println!("player A:"),
        1 => println!("player B:"),
        2 => println!("player C:"),
        _ => panic!("Uknown player {}", player_number),
    }
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!("your input: {}", input);
    }
}
