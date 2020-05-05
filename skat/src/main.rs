extern crate ansi_term;
extern crate getopts;
extern crate rand;

use ansi_term::Colour::*;
use getopts::Options;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::io::BufReader;
use std::path::Path;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

struct Player {
    id: u8,
    cards: Vec<u8>,
    tricks: Vec<u8>,
    counter: u8,
}

impl Player {
    fn add_trick(&mut self, played_cards: &Vec<u8>) {
        self.tricks.push(played_cards[0]);
        self.tricks.push(played_cards[1]);
        self.tricks.push(played_cards[2]);
        self.counter += Player::value_of(played_cards[0]);
        self.counter += Player::value_of(played_cards[1]);
        self.counter += Player::value_of(played_cards[2]);
        println!("id{} has {:?}", self.id, self.counter);
    }

    fn allow_sorting(&mut self) -> char {
        let mut g: char = 'd';
        loop {
            println!("Sort for?");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            if input == "g\n".to_string() {
                println!("Sort for Grand ...");
                g = 'g';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "b\n".to_string() {
                println!("Sort for Sächsische Spitze ...");
                g = 'b';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "n\n".to_string() {
                println!("Sort for Null ...");
                g = 'n';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "c\n".to_string() {
                println!("Sort for Clubs ...");
                g = 'c';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "s\n".to_string() {
                println!("Sort for Spades ...");
                g = 's';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "h\n".to_string() {
                println!("Sort for Hearts ...");
                g = 'h';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else if input == "d\n".to_string() {
                println!("Sort for Diamonds ...");
                g = 'd';
                self.sort_cards_for(g);
                self.print_cards(false);
            } else {
                break;
            }
        }
        g
    }

    fn announce_game(&mut self,
                     sorted_game: &mut char,
                     skat: &Skat,
                     hand: &mut bool,
                     ouvert: &mut bool,
                     matadors: &mut u8)
                     -> Player {
        // print cards before announcing the game
        self.print_cards(false);
        println!("Do you want to see the Skat? Press '1' for yes:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let player_builder = &mut PlayerBuilder::new();
        player_builder
            .id(self.id)
            // copy cards from self
            .add(self.cards[0])
            .add(self.cards[1])
            .add(self.cards[2])
            .add(self.cards[3])
            .add(self.cards[4])
            .add(self.cards[5])
            .add(self.cards[6])
            .add(self.cards[7])
            .add(self.cards[8])
            .add(self.cards[9]);
        let tuple_matadors: (u8, u8);
        if input == 1 {
            // Matadors jack strait
            tuple_matadors = player_builder.matadors_jack_strait(&skat);
            *matadors = tuple_matadors.0;
            // print Skat
            skat.print_cards();
            // deal with Skat
            player_builder
                // take skat
                .take(&skat)
                // drop (interactively) two cards
                .drop1()
                .drop2();
        } else {
            // Matadors jack strait
            tuple_matadors = player_builder.matadors_jack_strait(&skat);
            *matadors = tuple_matadors.0;
            // player still gets the Skat for counting
            player_builder.do_not_take(&skat);
            *hand = true;
        }
        let mut player = player_builder.finalize();
        if input == 1 {
            *sorted_game = player.allow_sorting();
        }
        // announce
        *sorted_game = announce(*sorted_game as char, ouvert);
        match *sorted_game {
            // TODO: ouvert
            'g' => {
                if *hand {
                    println!("Grand Hand announced ...");
                } else {
                    println!("Grand announced ...");
                }
            }
            'b' => {
                if *hand {
                    println!("Sächsische Spitze Hand announced ...");
                } else {
                    println!("Sächsische Spitze announced ...");
                }
                *matadors = tuple_matadors.1; // overwrite
            }
            'n' => {
                if *hand {
                    println!("Null Hand announced ...");
                } else {
                    println!("Null announced ...");
                }
            }
            'c' => {
                if *hand {
                    println!("Clubs Hand announced ...");
                } else {
                    println!("Clubs announced ...");
                }
            }
            's' => {
                if *hand {
                    println!("Spades Hand announced ...");
                } else {
                    println!("Spades announced ...");
                }
            }
            'h' => {
                if *hand {
                    println!("Hearts Hand announced ...");
                } else {
                    println!("Hearts announced ...");
                }
            }
            'd' => {
                if *hand {
                    println!("Diamonds Hand announced ...");
                } else {
                    println!("Diamonds announced ...");
                }
            }
            _ => panic!("Unknown game announced"),
        }
        player
    }

    fn count_cards(&self) -> u8 {
        self.counter
    }

    fn play_card(&mut self, played_cards: &mut Vec<u8>, player: u8, game: char) {
        let mut first_card;
        loop {
            println!("choose card [0-{}]:", self.cards.len() - 1);
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            match input {
                0..=9 => {
                    if input >= self.cards.len() as u8 {
                        continue;
                    }
                    let card: u8 = self.cards[input as usize];
                    if player != 0 {
                        first_card = played_cards[0];
                        if !is_valid_card(card, first_card, game, &self.cards) {
                            println!("is NOT valid");
                            continue;
                        }
                    }
                    print_card(card, true);
                    println!("");
                    played_cards.push(self.cards.remove(input as usize));
                    break;
                }
                _ => continue,
            }
        }
    }

    fn print_leader(&self, leader_id: u8) {
        match leader_id {
            0 => println!("Player A is leading the first trick ..."),
            1 => println!("Player B is leading the first trick ..."),
            2 => println!("Player C is leading the first trick ..."),
            _ => panic!("Unknown player {}", leader_id),
        }
    }

    fn print_cards(&self, print_value: bool) -> String {
        let mut counter: u8 = 0u8;
        match self.id {
            0 => println!("Player A:"),
            1 => println!("Player B:"),
            2 => println!("Player C:"),
            _ => panic!("Unknown player {}", self.id),
        }
        let mut ret_str = String::new();
        for index in 0..self.cards.len() {
            print!("{}:", index);
            if print_value {
                counter += Player::value_of(self.cards[index]);
            }
            print_card(self.cards[index], true);
            let s: String = self.cards[index].to_string();
            ret_str.push_str(&s);
            ret_str.push(' ');
        }
        if print_value {
            print!("({})", counter);
        }
        println!("");
        ret_str.push('\n');
        ret_str
    }

    fn print_counter(&self) {
        println!("id{} has {:?}", self.id, self.counter);
    }

    fn sort_cards(&mut self) {
        let mut sorted: Vec<u8> = Vec::new();
        let mut clubs: Vec<u8> = Vec::new();
        let mut spades: Vec<u8> = Vec::new();
        let mut hearts: Vec<u8> = Vec::new();
        let mut diamonds: Vec<u8> = Vec::new();
        // first find Jacks
        for n in 0..10 {
            match self.cards[n] {
                // ClubsJack
                4 => sorted.push(self.cards[n]),
                // SpadesJack
                12 => sorted.push(self.cards[n]),
                // HeartsJack
                20 => sorted.push(self.cards[n]),
                // DiamondsJack
                28 => sorted.push(self.cards[n]),
                // Clubs
                0..=7 => clubs.push(self.cards[n]),
                // Spades
                8..=15 => spades.push(self.cards[n]),
                // Hearts
                16..=23 => hearts.push(self.cards[n]),
                // Diamonds
                24..=31 => diamonds.push(self.cards[n]),
                _ => panic!("Unknown card"),
            }
        }
        // order Jacks
        sorted.sort();
        // order suits
        clubs.sort();
        spades.sort();
        hearts.sort();
        diamonds.sort();
        // then add the suit with the highest count
        let empty: Vec<u8> = Vec::new();
        let mut highest: &Vec<u8> = &empty;
        let mut max: u8 = 0u8;
        if clubs.len() > max as usize {
            max = clubs.len() as u8;
            highest = &clubs;
        }
        if spades.len() > max as usize {
            max = spades.len() as u8;
            highest = &spades;
        }
        if hearts.len() > max as usize {
            max = hearts.len() as u8;
            highest = &hearts;
        }
        if diamonds.len() > max as usize {
            // max = diamonds.len() as u8;
            highest = &diamonds;
        }
        // append highest
        if highest == &clubs {
            // append clubs
            for n in 0..clubs.len() {
                sorted.push(clubs[n]);
            }
        }
        if highest == &spades {
            // append spades
            for n in 0..spades.len() {
                sorted.push(spades[n]);
            }
        }
        if highest == &hearts {
            // append hearts
            for n in 0..hearts.len() {
                sorted.push(hearts[n]);
            }
        }
        if highest == &diamonds {
            // append diamonds
            for n in 0..diamonds.len() {
                sorted.push(diamonds[n]);
            }
        }
        // append others
        if highest != &clubs {
            // append clubs
            for n in 0..clubs.len() {
                sorted.push(clubs[n]);
            }
        }
        if highest != &spades {
            // append spades
            for n in 0..spades.len() {
                sorted.push(spades[n]);
            }
        }
        if highest != &hearts {
            // append hearts
            for n in 0..hearts.len() {
                sorted.push(hearts[n]);
            }
        }
        if highest != &diamonds {
            // append diamonds
            for n in 0..diamonds.len() {
                sorted.push(diamonds[n]);
            }
        }
        self.cards = sorted;
    }

    fn sort_cards_for(&mut self, game: char) {
        let mut sorted: Vec<u8> = Vec::new();
        let mut clubs: Vec<u8> = Vec::new();
        let mut spades: Vec<u8> = Vec::new();
        let mut hearts: Vec<u8> = Vec::new();
        let mut diamonds: Vec<u8> = Vec::new();
        match game {
            'g' => {
                // println!("Grand");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append clubs
                for n in 0..clubs.len() {
                    sorted.push(clubs[n]);
                }
                // append spades
                for n in 0..spades.len() {
                    sorted.push(spades[n]);
                }
                // append hearts
                for n in 0..hearts.len() {
                    sorted.push(hearts[n]);
                }
                // append diamonds
                for n in 0..diamonds.len() {
                    sorted.push(diamonds[n]);
                }
            }
            'b' => {
                // println!("Sächsische Spitze");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                sorted.reverse();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append clubs
                let mut sorted2: Vec<u8> = Vec::new();
                let mut pushed_ten = false;
                let mut ten_found = false;
                let mut ten = 1; // ClubsTen
                for n in 0..clubs.len() {
                    // change position for 10
                    if clubs[n] != ten {
                        if clubs[n] >= 5 {
                            if ten_found && !pushed_ten {
                                sorted2.push(ten);
                                pushed_ten = true;
                            }
                            sorted2.push(clubs[n]);
                        } else {
                            sorted2.push(clubs[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted2.push(ten);
                }
                sorted2.reverse();
                sorted.append(&mut sorted2);
                // append hearts
                let mut sorted2: Vec<u8> = Vec::new();
                pushed_ten = false;
                ten_found = false;
                ten = 17; // HeartsTen
                for n in 0..hearts.len() {
                    // change position for 10
                    if hearts[n] != ten {
                        if hearts[n] >= 21 {
                            if ten_found && !pushed_ten {
                                sorted2.push(ten);
                                pushed_ten = true;
                            }
                            sorted2.push(hearts[n]);
                        } else {
                            sorted2.push(hearts[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted2.push(ten);
                }
                sorted2.reverse();
                sorted.append(&mut sorted2);
                // append spades
                let mut sorted2: Vec<u8> = Vec::new();
                pushed_ten = false;
                ten_found = false;
                ten = 9; // SpadesTen
                for n in 0..spades.len() {
                    // change position for 10
                    if spades[n] != ten {
                        if spades[n] >= 13 {
                            if ten_found && !pushed_ten {
                                sorted2.push(ten);
                                pushed_ten = true;
                            }
                            sorted2.push(spades[n]);
                        } else {
                            sorted2.push(spades[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted2.push(ten);
                }
                sorted2.reverse();
                sorted.append(&mut sorted2);
                // append diamonds
                let mut sorted2: Vec<u8> = Vec::new();
                pushed_ten = false;
                ten_found = false;
                ten = 25; // DiamondsTen
                for n in 0..diamonds.len() {
                    // change position for 10
                    if diamonds[n] != ten {
                        if diamonds[n] >= 29 {
                            if ten_found && !pushed_ten {
                                sorted2.push(ten);
                                pushed_ten = true;
                            }
                            sorted2.push(diamonds[n]);
                        } else {
                            sorted2.push(diamonds[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted2.push(ten);
                }
                sorted2.reverse();
                sorted.append(&mut sorted2);
            }
            'n' => {
                // println!("Null");

                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append clubs
                let mut pushed_ten = false;
                let mut ten_found = false;
                let mut ten = 1; // ClubsTen
                for n in 0..clubs.len() {
                    // change position for 10
                    if clubs[n] != ten {
                        if clubs[n] >= 5 {
                            if ten_found && !pushed_ten {
                                sorted.push(ten);
                                pushed_ten = true;
                            }
                            sorted.push(clubs[n]);
                        } else {
                            sorted.push(clubs[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted.push(ten);
                }
                // append hearts
                pushed_ten = false;
                ten_found = false;
                ten = 17; // HeartsTen
                for n in 0..hearts.len() {
                    // change position for 10
                    if hearts[n] != ten {
                        if hearts[n] >= 21 {
                            if ten_found && !pushed_ten {
                                sorted.push(ten);
                                pushed_ten = true;
                            }
                            sorted.push(hearts[n]);
                        } else {
                            sorted.push(hearts[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted.push(ten);
                }
                // append spades
                pushed_ten = false;
                ten_found = false;
                ten = 9; // SpadesTen
                for n in 0..spades.len() {
                    // change position for 10
                    if spades[n] != ten {
                        if spades[n] >= 13 {
                            if ten_found && !pushed_ten {
                                sorted.push(ten);
                                pushed_ten = true;
                            }
                            sorted.push(spades[n]);
                        } else {
                            sorted.push(spades[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted.push(ten);
                }
                // append diamonds
                pushed_ten = false;
                ten_found = false;
                ten = 25; // DiamondsTen
                for n in 0..diamonds.len() {
                    // change position for 10
                    if diamonds[n] != ten {
                        if diamonds[n] >= 29 {
                            if ten_found && !pushed_ten {
                                sorted.push(ten);
                                pushed_ten = true;
                            }
                            sorted.push(diamonds[n]);
                        } else {
                            sorted.push(diamonds[n]);
                        }
                    } else {
                        // we have a 10 in current set?
                        ten_found = true;
                    }
                }
                if ten_found && !pushed_ten {
                    sorted.push(ten);
                }
            }
            'c' => {
                // println!("Clubs");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append clubs
                for n in 0..clubs.len() {
                    sorted.push(clubs[n]);
                }
                // append hearts
                for n in 0..hearts.len() {
                    sorted.push(hearts[n]);
                }
                // append spades
                for n in 0..spades.len() {
                    sorted.push(spades[n]);
                }
                // append diamonds
                for n in 0..diamonds.len() {
                    sorted.push(diamonds[n]);
                }
            }
            's' => {
                // println!("Spades");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append spades
                for n in 0..spades.len() {
                    sorted.push(spades[n]);
                }
                // append hearts
                for n in 0..hearts.len() {
                    sorted.push(hearts[n]);
                }
                // append clubs
                for n in 0..clubs.len() {
                    sorted.push(clubs[n]);
                }
                // append diamonds
                for n in 0..diamonds.len() {
                    sorted.push(diamonds[n]);
                }
            }
            'h' => {
                // println!("Hearts");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append hearts
                for n in 0..hearts.len() {
                    sorted.push(hearts[n]);
                }
                // append clubs
                for n in 0..clubs.len() {
                    sorted.push(clubs[n]);
                }
                // append diamonds
                for n in 0..diamonds.len() {
                    sorted.push(diamonds[n]);
                }
                // append spades
                for n in 0..spades.len() {
                    sorted.push(spades[n]);
                }
            }
            'd' => {
                // println!("Diamonds");

                // first find Jacks
                for n in 0..self.cards.len() {
                    match self.cards[n] {
                        // ClubsJack
                        4 => sorted.push(self.cards[n]),
                        // SpadesJack
                        12 => sorted.push(self.cards[n]),
                        // HeartsJack
                        20 => sorted.push(self.cards[n]),
                        // DiamondsJack
                        28 => sorted.push(self.cards[n]),
                        // Clubs
                        0..=7 => clubs.push(self.cards[n]),
                        // Spades
                        8..=15 => spades.push(self.cards[n]),
                        // Hearts
                        16..=23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24..=31 => diamonds.push(self.cards[n]),
                        _ => panic!("Unknown card"),
                    }
                }
                // order Jacks
                sorted.sort();
                // order suits
                clubs.sort();
                spades.sort();
                hearts.sort();
                diamonds.sort();
                // append diamonds
                for n in 0..diamonds.len() {
                    sorted.push(diamonds[n]);
                }
                // append clubs
                for n in 0..clubs.len() {
                    sorted.push(clubs[n]);
                }
                // append hearts
                for n in 0..hearts.len() {
                    sorted.push(hearts[n]);
                }
                // append spades
                for n in 0..spades.len() {
                    sorted.push(spades[n]);
                }
            }
            _ => panic!("Unknown game {}", game),
        }
        self.cards = sorted;
    }

    fn value_of(card: u8) -> u8 {
        let value = match card {
            // Ace
            0 | 8 | 16 | 24 => 11u8,
            // Ten
            1 | 9 | 17 | 25 => 10u8,
            // King
            2 | 10 | 18 | 26 => 4u8,
            // Queen
            3 | 11 | 19 | 27 => 3u8,
            // Jack
            4 | 12 | 20 | 28 => 2u8,
            // others (7, 8, 9)
            _ => 0u8,
        };
        value
    }
}

struct PlayerBuilder {
    id: u8,
    cards: Vec<u8>,
    counter: u8,
}

impl PlayerBuilder {
    fn new() -> PlayerBuilder {
        PlayerBuilder {
            cards: Vec::new(),
            id: 3u8,
            counter: 0u8,
        }
    }

    fn add(&mut self, new_card: u8) -> &mut PlayerBuilder {
        self.cards.push(new_card);
        self
    }

    fn do_not_take(&mut self, skat: &Skat) -> &mut PlayerBuilder {
        self.counter += Player::value_of(skat.first);
        self.counter += Player::value_of(skat.second);
        self
    }

    fn drop1(&mut self) -> &mut PlayerBuilder {
        self.cards.sort();
        self.print_cards();
        loop {
            println!("drop:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            match input {
                0..=11 => {
                    println!("{} chosen ...", input);
                    let card = self.cards.remove(input as usize);
                    self.counter += Player::value_of(card);
                    break;
                }
                _ => continue,
            }
        }
        self
    }

    fn drop2(&mut self) -> &mut PlayerBuilder {
        self.cards.sort();
        self.print_cards();
        loop {
            println!("drop:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            match input {
                0..=10 => {
                    println!("{} chosen ...", input);
                    let card = self.cards.remove(input as usize);
                    self.counter += Player::value_of(card);
                    break;
                }
                _ => continue,
            }
        }
        self.print_cards();
        self
    }

    fn id(&mut self, new_id: u8) -> &mut PlayerBuilder {
        self.id = new_id;
        self
    }

    fn matadors_jack_strait(&self, skat: &Skat) -> (u8, u8) {
        let mut matadors: u8 = 0u8;
        let mut b_matadors: u8 = 0u8;
        let mut jacks: Vec<u8> = Vec::new();
        // first find Jacks
        for n in 0..10 {
            match self.cards[n] {
                // ClubsJack
                4 => jacks.push(self.cards[n]),
                // SpadesJack
                12 => jacks.push(self.cards[n]),
                // HeartsJack
                20 => jacks.push(self.cards[n]),
                // DiamondsJack
                28 => jacks.push(self.cards[n]),
                _ => continue,
            }
        }
        // first card of Skat
        let mut skat_card = skat.first;
        match skat_card {
            // ClubsJack
            4 => jacks.push(skat_card),
            // SpadesJack
            12 => jacks.push(skat_card),
            // HeartsJack
            20 => jacks.push(skat_card),
            // DiamondsJack
            28 => jacks.push(skat_card),
            _ => {} // do nothing
        }
        // second card of Skat
        skat_card = skat.second;
        match skat_card {
            // ClubsJack
            4 => jacks.push(skat_card),
            // SpadesJack
            12 => jacks.push(skat_card),
            // HeartsJack
            20 => jacks.push(skat_card),
            // DiamondsJack
            28 => jacks.push(skat_card),
            _ => {} // do nothing
        }
        // order Jacks
        jacks.sort();
        let mut jacks_copy = jacks.to_vec();
        jacks_copy.reverse();
        // count matadors for Sächsische Spitze indepently
        let mut with = false;
        let jacks_copy_len = jacks_copy.len();
        // _index not used
        for _index in 0..jacks_copy_len {
            let jack = jacks_copy[0]; // first
            match jack {
                // DiamondsJack
                28 => {
                    with = true;
                    b_matadors = 1;
                    jacks_copy.remove(0);
                }
                // HeartsJack
                20 => {
                    if with {
                        b_matadors = 2;
                        jacks_copy.remove(0);
                    } else {
                        b_matadors = 1;
                        break;
                    }
                }
                // SpadesJack
                12 => {
                    if with {
                        if b_matadors == 2 {
                            b_matadors = 3;
                            jacks_copy.remove(0);
                        } else {
                            break;
                        }
                    } else {
                        b_matadors = 2;
                        break;
                    }
                }
                // ClubsJack
                4 => {
                    if with {
                        if b_matadors == 3 {
                            b_matadors = 4;
                            jacks_copy.remove(0);
                        } else {
                            break;
                        }
                    } else {
                        b_matadors = 3;
                        break;
                    }
                }
                _ => panic!("no Jack found"),
            }
        }
        if jacks_copy_len == 0 {
            b_matadors = 4; // without 4
        }
        // count matadors
        with = false;
        let jacks_len = jacks.len();
        // _index not used
        for _index in 0..jacks_len {
            let jack = jacks[0]; // first
            match jack {
                // ClubsJack
                4 => {
                    with = true;
                    matadors = 1;
                    jacks.remove(0);
                }
                // SpadesJack
                12 => {
                    if with {
                        matadors = 2;
                        jacks.remove(0);
                    } else {
                        matadors = 1;
                        return (matadors, b_matadors);
                    }
                }
                // HeartsJack
                20 => {
                    if with {
                        if matadors == 2 {
                            matadors = 3;
                            jacks.remove(0);
                        } else {
                            return (matadors, b_matadors);
                        }
                    } else {
                        matadors = 2;
                        return (matadors, b_matadors);
                    }
                }
                // DiamondsJack
                28 => {
                    if with {
                        if matadors == 3 {
                            matadors = 4;
                            jacks.remove(0);
                        } else {
                            return (matadors, b_matadors);
                        }
                    } else {
                        matadors = 3;
                        return (matadors, b_matadors);
                    }
                }
                _ => panic!("no Jack found"),
            }
        }
        if jacks_len == 0 {
            matadors = 4; // without 4
        }
        (matadors, b_matadors)
    }

    fn print_cards(&self) {
        let len = self.cards.len();
        for index in 0..len {
            print!("{}:", index);
            print_card(self.cards[index], true);
        }
        println!("");
    }

    fn take(&mut self, skat: &Skat) -> &mut PlayerBuilder {
        self.cards.push(skat.first);
        self.cards.push(skat.second);
        self
    }

    fn finalize(&self) -> Player {
        Player {
            id: self.id,
            cards: self.cards.to_vec(),
            tricks: Vec::new(),
            counter: self.counter,
        }
    }
}

struct Skat {
    first: u8,
    second: u8,
}

impl Skat {
    fn print_cards(&self) {
        println!("Skat:");
        print_card(self.first, true);
        print_card(self.second, true);
        println!("");
    }
}

struct SkatBuilder {
    first: u8,
    second: u8,
}

impl SkatBuilder {
    fn new() -> SkatBuilder {
        SkatBuilder {
            first: 0u8,
            second: 0u8,
        }
    }

    fn add(&mut self, f: u8, s: u8) -> &mut SkatBuilder {
        self.first = f;
        self.second = s;
        self
    }

    fn finalize(&self) -> Skat {
        Skat {
            first: self.first,
            second: self.second,
        }
    }
}

struct Record {
    cards: Vec<u8>,
    played: Vec<bool>,
}

impl Record {
    fn print_cards(&self, game: char) {
        println!("Cards:");
        if game == 'n' {
            // Null
            for suit in 0..4 {
                // Aces first ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    match index % 8 == 0 {
                        true => {
                            if index < 10 {
                                print!(" {}:", index);
                            } else {
                                print!("{}:", index);
                            }
                            print_card(self.cards[index], !self.played[index]);
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                // Kings, Queens, Jacks ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    match index % 8 >= 2 && index % 8 <= 4 {
                        true => {
                            if index < 10 {
                                print!(" {}:", index);
                            } else {
                                print!("{}:", index);
                            }
                            print_card(self.cards[index], !self.played[index]);
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                // Tens ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    match index % 8 == 1 {
                        true => {
                            if index < 10 {
                                print!(" {}:", index);
                            } else {
                                print!("{}:", index);
                            }
                            print_card(self.cards[index], !self.played[index]);
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                // Nines, Eights, Sevens ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    match index % 8 >= 5 && index % 8 <= 7 {
                        true => {
                            if index < 10 {
                                print!(" {}:", index);
                            } else {
                                print!("{}:", index);
                            }
                            print_card(self.cards[index], !self.played[index]);
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                println!("");
            }
        } else {
            // Grand or Suit
            for suit in 0..4 {
                // Jacks first ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    if index % 8 == 4 {
                        if index < 10 {
                            print!(" {}:", index);
                        } else {
                            print!("{}:", index);
                        }
                        print_card(self.cards[index], !self.played[index]);
                    }
                }
                // then the others ...
                for c in 0..8 {
                    let index = suit * 8 + c;
                    if index % 8 != 4 {
                        if index < 10 {
                            print!(" {}:", index);
                        } else {
                            print!("{}:", index);
                        }
                        print_card(self.cards[index], !self.played[index]);
                    }
                }
                println!("");
            }
        }
        println!("");
    }

    fn is_valid(&mut self, index: u8) -> bool {
        if !self.played[index as usize] {
            self.played[index as usize] = true;
            true
        } else {
            false
        }
    }
}

struct RecordBuilder {
    cards: Vec<u8>,
    played: Vec<bool>,
}

impl RecordBuilder {
    fn new() -> RecordBuilder {
        RecordBuilder {
            cards: Vec::new(),
            played: Vec::new(),
        }
    }

    fn add(&mut self, new_card: u8) -> &mut RecordBuilder {
        self.cards.push(new_card);
        self.played.push(false);
        self
    }
    fn finalize(&self) -> Record {
        Record {
            cards: self.cards.to_vec(),
            played: self.played.to_vec(),
        }
    }
}

fn announce(game: char, ouvert: &mut bool) -> char {
    let mut g: char = game; // copy game value (can still be changed)
    print!("announce game [gbncshd], ouvert [o] or y for ");
    loop {
        match g {
            'g' => println!("Grand?"),
            'b' => println!("Sächsische Spitze?"),
            'n' => println!("Null?"),
            'c' => println!("Clubs?"),
            's' => println!("Spades?"),
            'h' => println!("Hearts?"),
            'd' => println!("Diamonds?"),
            _ => {
                println!("Choose game [gbncshd]:");
                g = 'd';
            }
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "y\n".to_string() || input == "\n" {
            return g;
        } else if input == "g\n".to_string() {
            g = 'g';
            return g;
        } else if input == "b\n".to_string() {
            g = 'b';
            return g;
        } else if input == "n\n".to_string() {
            g = 'n';
            return g;
        } else if input == "c\n".to_string() {
            g = 'c';
            return g;
        } else if input == "s\n".to_string() {
            g = 's';
            return g;
        } else if input == "h\n".to_string() {
            g = 'h';
            return g;
        } else if input == "d\n".to_string() {
            g = 'd';
            return g;
        } else if input == "o\n".to_string() {
            println!("Ouvert announced, but tell me the game:");
            // g doesn't change
            *ouvert = true;
            // keep asking for game
            continue;
        }
    }
}

fn bid(dealer: &mut Player,
       responder: &mut Player,
       bidder: &mut Player,
       leader_id: u8)
       -> (u8, u8, char) {
    // return values
    let mut winner: u8;
    let mut lowest: u8;
    let game: char;
    // bidder sees his cards first
    bidder.print_leader(leader_id);
    bidder.print_cards(true);
    // ask for input
    let mut bidder_bid: u8;
    let mut g: char = 'd';
    loop {
        println!("bid (or [gbncshd]):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("Sort for Grand ...");
            g = 'g';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "b\n".to_string() {
            println!("Sort for Sächsische Spitze ...");
            g = 'b';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "n\n".to_string() {
            println!("Sort for Null ...");
            g = 'n';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "c\n".to_string() {
            println!("Sort for Clubs ...");
            g = 'c';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "s\n".to_string() {
            println!("Sort for Spades ...");
            g = 's';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "h\n".to_string() {
            println!("Sort for Hearts ...");
            g = 'h';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        } else if input == "d\n".to_string() {
            println!("Sort for Diamonds ...");
            g = 'd';
            bidder.sort_cards_for(g);
            bidder.print_cards(false);
        }
        bidder_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid_bid(bidder_bid) {
            break;
        } else {
            continue;
        }
    }
    // keep selected (by sorting) game
    let bidder_game = g;
    println!("Bidder: {}", bidder_bid);
    // responder is next
    responder.print_leader(leader_id);
    responder.print_cards(true);
    // ask for input
    let mut responder_bid: u8;
    loop {
        println!("bid (or [gbncshd]):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("Sort for Grand ...");
            g = 'g';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "b\n".to_string() {
            println!("Sort for Sächsische Spitze ...");
            g = 'b';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "n\n".to_string() {
            println!("Sort for Null ...");
            g = 'n';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "c\n".to_string() {
            println!("Sort for Clubs ...");
            g = 'c';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "s\n".to_string() {
            println!("Sort for Spades ...");
            g = 's';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "h\n".to_string() {
            println!("Sort for Hearts ...");
            g = 'h';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        } else if input == "d\n".to_string() {
            println!("Sort for Diamonds ...");
            g = 'd';
            responder.sort_cards_for(g);
            responder.print_cards(false);
        }
        responder_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid_bid(responder_bid) {
            break;
        } else {
            continue;
        }
    }
    let responder_game = g;
    println!("Responder: {}", responder_bid);
    // ask dealer last
    dealer.print_leader(leader_id);
    dealer.print_cards(true);
    // ask for input
    let mut dealer_bid: u8;
    loop {
        println!("bid (or [gbncshd]):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("Sort for Grand ...");
            g = 'g';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "b\n".to_string() {
            println!("Sort for Sächsische Spitze ...");
            g = 'b';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "n\n".to_string() {
            println!("Sort for Null ...");
            g = 'n';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "c\n".to_string() {
            println!("Sort for Clubs ...");
            g = 'c';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "s\n".to_string() {
            println!("Sort for Spades ...");
            g = 's';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "h\n".to_string() {
            println!("Sort for Hearts ...");
            g = 'h';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        } else if input == "d\n".to_string() {
            println!("Sort for Diamonds ...");
            g = 'd';
            dealer.sort_cards_for(g);
            dealer.print_cards(false);
        }
        dealer_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid_bid(dealer_bid) {
            break;
        } else {
            continue;
        }
    }
    let dealer_game = g;
    println!("Dealer: {}", dealer_bid);
    // who wins bidding?
    // first between bidder and responder
    if bidder_bid > responder_bid {
        // bidder wins (so far)
        lowest = 18u8; // bidder said at least 18
        if responder_bid > lowest {
            lowest = responder_bid;
        }
        winner = bidder.id;
    } else {
        // special case: both bids are 0 (pass)
        if bidder_bid == 0u8 && responder_bid == 0u8 {
            if dealer_bid == 0u8 {
                // nobody wants to play
                return (dealer.id, 0u8, 'd');
            } else {
                // dealer wants to play and all others passed
                lowest = 18u8; // dealer said at least 18
                winner = dealer.id;
                game = dealer_game;
                let winner_name = match winner {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => panic!("Unknown player {}", winner),
                };
                println!("Player {} wins bidding with {} ...", winner_name, lowest);
                return (winner, lowest, game);
            }
        } else {
            // responder_bid > bidder_bid
            lowest = bidder_bid; // can still be 0
            if bidder_bid == 0u8 {
                lowest = 18u8; // responder said at least 18
            }
            // responder wins (so far)
            winner = responder.id;
        }
    }
    // now dealer gets his chance
    if winner == responder.id {
        // responder didn't pass and said at least 18, dealer listens
        if dealer_bid > responder_bid {
            if responder_bid > lowest {
                lowest = responder_bid;
            }
            winner = dealer.id;
            game = dealer_game;
        } else {
            if dealer_bid > lowest {
                lowest = dealer_bid;
            }
            winner = responder.id;
            game = responder_game;
        }
    } else {
        // winner == bidder.id
        // responder passed, dealer can pass or has to say more than bidder
        if dealer_bid > bidder_bid {
            // dealer wins
            lowest = responder_bid;
            if responder_bid == 0u8 {
                lowest = 18u8; // dealer said at least 18
            }
            if bidder_bid > lowest {
                // dealer has to say more than bidder
                lowest = bidder_bid;
                loop {
                    lowest += 1;
                    if is_valid_bid(lowest) {
                        break;
                    }
                }
            }
            winner = dealer.id;
            game = dealer_game;
        } else {
            if dealer_bid > lowest {
                lowest = dealer_bid;
            }
            winner = bidder.id;
            game = bidder_game;
        }
    }
    let winner_name = match winner {
        0 => "A",
        1 => "B",
        2 => "C",
        _ => panic!("Unknown player {}", winner),
    };
    println!("Player {} wins bidding with {} ...", winner_name, lowest);
    (winner, lowest, game)
}

fn deal(dealer_id: u8) -> (Player, Player, Player, Skat) {
    let mut cards: Vec<u8> = (0..32).collect();
    // shuffle cards
    let upper: u8 = 32;
    let mut shuffled: Vec<u8> = Vec::new();
    for n in 0..32 {
        // randomly select card
        let pile_index: u8 = rand::thread_rng().gen_range(0, upper - n);
        // remove selected card from old pile
        let card = cards.remove(pile_index as usize);
        // add selected card to new pile
        shuffled.push(card);
    }
    // create dealer with the first 10 cards
    let mut dealer = PlayerBuilder::new()
        .id(dealer_id)
        .add(shuffled[0])
        .add(shuffled[1])
        .add(shuffled[2])
        .add(shuffled[3])
        .add(shuffled[4])
        .add(shuffled[5])
        .add(shuffled[6])
        .add(shuffled[7])
        .add(shuffled[8])
        .add(shuffled[9])
        .finalize();
    // let the dealer print his own cards
    dealer.sort_cards();
    println!("");
    dealer.print_cards(false);
    // create the left player (will play first card)
    let mut left = PlayerBuilder::new()
        .id((dealer_id + 1) % 3)
        .add(shuffled[10])
        .add(shuffled[11])
        .add(shuffled[12])
        .add(shuffled[13])
        .add(shuffled[14])
        .add(shuffled[15])
        .add(shuffled[16])
        .add(shuffled[17])
        .add(shuffled[18])
        .add(shuffled[19])
        .finalize();
    left.sort_cards();
    println!("");
    left.print_cards(false);
    // create the right player (will play bid first)
    let mut right = PlayerBuilder::new()
        .id((dealer_id + 2) % 3)
        .add(shuffled[20])
        .add(shuffled[21])
        .add(shuffled[22])
        .add(shuffled[23])
        .add(shuffled[24])
        .add(shuffled[25])
        .add(shuffled[26])
        .add(shuffled[27])
        .add(shuffled[28])
        .add(shuffled[29])
        .finalize();
    right.sort_cards();
    println!("");
    right.print_cards(false);
    // Skat
    let skat = SkatBuilder::new()
        .add(shuffled[30], shuffled[31])
        .finalize();
    println!("");
    skat.print_cards();
    println!("");
    (dealer, left, right, skat)
}

fn is_in(card: u8, cards: &Vec<u8>) -> bool {
    for n in 0..cards.len() {
        if card == cards[n] {
            return true;
        }
    }
    false
}

fn is_valid_bid(bid: u8) -> bool {
    let mut valid;
    // Null game (have to be checked first)
    let nulls = [23, 35, 46, 59];
    valid = match nulls.iter().find(|&&x| x == bid) {
        Some(_) => true,
        None => false,
    };
    // Suit game
    if (bid % 12) == 0 {
        valid = true;
    }
    if (bid % 11) == 0 {
        valid = true;
    }
    if (bid % 10) == 0 {
        valid = true;
    }
    if (bid % 9) == 0 {
        valid = true;
    }
    // 13 * 12 = 4 Jacks, 6 trumps, Game, Schneider, Schwarz
    let highest: u8 = 156;
    if bid > highest {
        valid = false;
    }
    // Grand game
    if (bid % 24) == 0 {
        valid = true;
    }
    // 7 * 24 = 4 Jacks, Game, Schneider, Schwarz
    let highest: u8 = 168;
    if bid > highest {
        valid = false;
    }
    valid
}

fn is_valid_card(card: u8, first_card: u8, game: char, cards: &Vec<u8>) -> bool {
    // sort cards
    let mut jacks: Vec<u8> = Vec::new();
    let mut clubs: Vec<u8> = Vec::new();
    let mut spades: Vec<u8> = Vec::new();
    let mut hearts: Vec<u8> = Vec::new();
    let mut diamonds: Vec<u8> = Vec::new();
    for n in 0..cards.len() {
        match cards[n] {
            // ClubsJack
            4 => jacks.push(cards[n]),
            // SpadesJack
            12 => jacks.push(cards[n]),
            // HeartsJack
            20 => jacks.push(cards[n]),
            // DiamondsJack
            28 => jacks.push(cards[n]),
            // Clubs
            0..=7 => clubs.push(cards[n]),
            // Spades
            8..=15 => spades.push(cards[n]),
            // Hearts
            16..=23 => hearts.push(cards[n]),
            // Diamonds
            24..=31 => diamonds.push(cards[n]),
            _ => panic!("Unknown card"),
        }
    }
    match game {
        'g' => {
            // Grand
            match first_card {
                4 | 12 | 20 | 28 => {
                    // Jack
                    if jacks.len() > 0usize && !is_in(card, &jacks) {
                        // has Jacks, but didn't play one
                        return false;
                    } else {
                        // has Jacks and played one of it
                        // or doesn't have any Jacks
                        return true;
                    }
                }
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'b' => {
            // Sächsische Spitze
            match first_card {
                4 | 12 | 20 | 28 => {
                    // Jack
                    if jacks.len() > 0usize && !is_in(card, &jacks) {
                        // has Jacks, but didn't play one
                        return false;
                    } else {
                        // has Jacks and played one of it
                        // or doesn't have any Jacks
                        return true;
                    }
                }
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'n' => {
            // Null
            for n in 0..jacks.len() {
                match jacks[n] {
                    4 => clubs.push(jacks[n]),
                    12 => spades.push(jacks[n]),
                    20 => hearts.push(jacks[n]),
                    28 => diamonds.push(jacks[n]),
                    _ => println!("Unknown Jack"),
                }
            }
            match first_card {
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'c' => {
            // Clubs
            match first_card {
                0..=7 | 12 | 20 | 28 => {
                    // Jack or Clubs
                    if (jacks.len() > 0usize || clubs.len() > 0usize) && !is_in(card, &jacks) &&
                       !is_in(card, &clubs) {
                        // has Jacks or suit, but didn't play one
                        return false;
                    } else {
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        's' => {
            // Spades
            match first_card {
                4 | 8..=15 | 20 | 28 => {
                    // Jack or Spades
                    if (jacks.len() > 0usize || spades.len() > 0usize) && !is_in(card, &jacks) &&
                       !is_in(card, &spades) {
                        // has Jacks or suit, but didn't play one
                        return false;
                    } else {
                        return true;
                    }
                }
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'h' => {
            // Hearts
            match first_card {
                4 | 12 | 16..=23 | 28 => {
                    // Jack or Hearts
                    if (jacks.len() > 0usize || hearts.len() > 0usize) && !is_in(card, &jacks) &&
                       !is_in(card, &hearts) {
                        // has Jacks or suit, but didn't play one
                        return false;
                    } else {
                        return true;
                    }
                }
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                24..=31 => {
                    // Diamonds
                    if diamonds.len() > 0usize && !is_in(card, &diamonds) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'd' => {
            // Diamonds
            match first_card {
                4 | 12 | 20 | 24..=31 => {
                    // Jack or Diamonds
                    if (jacks.len() > 0usize || diamonds.len() > 0usize) &&
                       !is_in(card, &jacks) && !is_in(card, &diamonds) {
                        // has Jacks or suit, but didn't play one
                        return false;
                    } else {
                        return true;
                    }
                }
                0..=7 => {
                    // Clubs
                    if clubs.len() > 0usize && !is_in(card, &clubs) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                8..=15 => {
                    // Spades
                    if spades.len() > 0usize && !is_in(card, &spades) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                16..=23 => {
                    // Hearts
                    if hearts.len() > 0usize && !is_in(card, &hearts) {
                        // has suit, but didn't play one
                        return false;
                    } else {
                        // free to play any other card
                        return true;
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        _ => panic!("Unknown game {}", game),
    }
}

fn print_card(card: u8, in_color: bool) {
    let club = "♣";
    let spade = "♠";
    let heart = "♥";
    let diamond = "♦";
    match card {
        0 => {
            let mut ace = " A".to_string();
            ace.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&ace));
            } else {
                print!("{} ", White.bold().paint(&ace));
            }
        }
        1 => {
            let mut ten = "10".to_string();
            ten.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&ten));
            } else {
                print!("{} ", White.bold().paint(&ten));
            }
        }
        2 => {
            let mut king = " K".to_string();
            king.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&king));
            } else {
                print!("{} ", White.bold().paint(&king));
            }
        }
        3 => {
            let mut queen = " Q".to_string();
            queen.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&queen));
            } else {
                print!("{} ", White.bold().paint(&queen));
            }
        }
        4 => {
            let mut jack = " J".to_string();
            jack.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&jack));
            } else {
                print!("{} ", White.bold().paint(&jack));
            }
        }
        5 => {
            let mut nine = " 9".to_string();
            nine.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&nine));
            } else {
                print!("{} ", White.bold().paint(&nine));
            }
        }
        6 => {
            let mut eight = " 8".to_string();
            eight.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&eight));
            } else {
                print!("{} ", White.bold().paint(&eight));
            }
        }
        7 => {
            let mut seven = " 7".to_string();
            seven.push_str(&club);
            if in_color {
                print!("{} ", Black.bold().paint(&seven));
            } else {
                print!("{} ", White.bold().paint(&seven));
            }
        }
        8 => {
            let mut ace = " A".to_string();
            ace.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&ace));
            } else {
                print!("{} ", White.bold().paint(&ace));
            }
        }
        9 => {
            let mut ten = "10".to_string();
            ten.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&ten));
            } else {
                print!("{} ", White.bold().paint(&ten));
            }
        }
        10 => {
            let mut king = " K".to_string();
            king.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&king));
            } else {
                print!("{} ", White.bold().paint(&king));
            }
        }
        11 => {
            let mut queen = " Q".to_string();
            queen.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&queen));
            } else {
                print!("{} ", White.bold().paint(&queen));
            }
        }
        12 => {
            let mut jack = " J".to_string();
            jack.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&jack));
            } else {
                print!("{} ", White.bold().paint(&jack));
            }
        }
        13 => {
            let mut nine = " 9".to_string();
            nine.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&nine));
            } else {
                print!("{} ", White.bold().paint(&nine));
            }
        }
        14 => {
            let mut eight = " 8".to_string();
            eight.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&eight));
            } else {
                print!("{} ", White.bold().paint(&eight));
            }
        }
        15 => {
            let mut seven = " 7".to_string();
            seven.push_str(spade);
            if in_color {
                print!("{} ", Green.bold().paint(&seven));
            } else {
                print!("{} ", White.bold().paint(&seven));
            }
        }
        16 => {
            let mut ace = " A".to_string();
            ace.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&ace));
            } else {
                print!("{} ", White.bold().paint(&ace));
            }
        }
        17 => {
            let mut ten = "10".to_string();
            ten.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&ten));
            } else {
                print!("{} ", White.bold().paint(&ten));
            }
        }
        18 => {
            let mut king = " K".to_string();
            king.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&king));
            } else {
                print!("{} ", White.bold().paint(&king));
            }
        }
        19 => {
            let mut queen = " Q".to_string();
            queen.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&queen));
            } else {
                print!("{} ", White.bold().paint(&queen));
            }
        }
        20 => {
            let mut jack = " J".to_string();
            jack.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&jack));
            } else {
                print!("{} ", White.bold().paint(&jack));
            }
        }
        21 => {
            let mut nine = " 9".to_string();
            nine.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&nine));
            } else {
                print!("{} ", White.bold().paint(&nine));
            }
        }
        22 => {
            let mut eight = " 8".to_string();
            eight.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&eight));
            } else {
                print!("{} ", White.bold().paint(&eight));
            }
        }
        23 => {
            let mut seven = " 7".to_string();
            seven.push_str(heart);
            if in_color {
                print!("{} ", Red.bold().paint(&seven));
            } else {
                print!("{} ", White.bold().paint(&seven));
            }
        }
        24 => {
            let mut ace = " A".to_string();
            ace.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&ace));
            } else {
                print!("{} ", White.bold().paint(&ace));
            }
        }
        25 => {
            let mut ten = "10".to_string();
            ten.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&ten));
            } else {
                print!("{} ", White.bold().paint(&ten));
            }
        }
        26 => {
            let mut king = " K".to_string();
            king.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&king));
            } else {
                print!("{} ", White.bold().paint(&king));
            }
        }
        27 => {
            let mut queen = " Q".to_string();
            queen.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&queen));
            } else {
                print!("{} ", White.bold().paint(&queen));
            }
        }
        28 => {
            let mut jack = " J".to_string();
            jack.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&jack));
            } else {
                print!("{} ", White.bold().paint(&jack));
            }
        }
        29 => {
            let mut nine = " 9".to_string();
            nine.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&nine));
            } else {
                print!("{} ", White.bold().paint(&nine));
            }
        }
        30 => {
            let mut eight = " 8".to_string();
            eight.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&eight));
            } else {
                print!("{} ", White.bold().paint(&eight));
            }
        }
        31 => {
            let mut seven = " 7".to_string();
            seven.push_str(diamond);
            if in_color {
                print!("{} ", Yellow.bold().paint(&seven));
            } else {
                print!("{} ", White.bold().paint(&seven));
            }
        }
        _ => panic!("Unknown card"),
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_version(program: &str) {
    println!("{} {}", program, VERSION);
}

fn sort_trick_for(cards: &Vec<u8>, game: char) -> Vec<u8> {
    let mut sorted: Vec<u8> = Vec::new();
    let mut clubs: Vec<u8> = Vec::new();
    let mut spades: Vec<u8> = Vec::new();
    let mut hearts: Vec<u8> = Vec::new();
    let mut diamonds: Vec<u8> = Vec::new();
    let first_card_played: u8 = cards[0];
    match game {
        'g' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                8..=15 => {
                    // Spades
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'b' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            sorted.reverse();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append clubs
                    let mut sorted2: Vec<u8> = Vec::new();
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(clubs[n]);
                            } else {
                                sorted2.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append hearts
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(hearts[n]);
                            } else {
                                sorted2.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append spades
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(spades[n]);
                            } else {
                                sorted2.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append diamonds
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(diamonds[n]);
                            } else {
                                sorted2.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                }
                8..=15 => {
                    // Spades
                    // append spades
                    let mut sorted2: Vec<u8> = Vec::new();
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(spades[n]);
                            } else {
                                sorted2.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append clubs
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(clubs[n]);
                            } else {
                                sorted2.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append hearts
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(hearts[n]);
                            } else {
                                sorted2.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append diamonds
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(diamonds[n]);
                            } else {
                                sorted2.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                }
                16..=23 => {
                    // Hearts
                    // append hearts
                    let mut sorted2: Vec<u8> = Vec::new();
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(hearts[n]);
                            } else {
                                sorted2.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append clubs
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(clubs[n]);
                            } else {
                                sorted2.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append spades
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(spades[n]);
                            } else {
                                sorted2.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append diamonds
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(diamonds[n]);
                            } else {
                                sorted2.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                }
                24..=31 => {
                    // Diamonds
                    // append diamonds
                    let mut sorted2: Vec<u8> = Vec::new();
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(diamonds[n]);
                            } else {
                                sorted2.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append clubs
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(clubs[n]);
                            } else {
                                sorted2.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append hearts
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(hearts[n]);
                            } else {
                                sorted2.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                    // append spades
                    let mut sorted2: Vec<u8> = Vec::new();
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted2.push(ten);
                                    pushed_ten = true;
                                }
                                sorted2.push(spades[n]);
                            } else {
                                sorted2.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted2.push(ten);
                    }
                    sorted2.reverse();
                    sorted.append(&mut sorted2);
                }
                _ => panic!("Unknown card"),
            }
        }
        'n' => {
            for n in 0..3 {
                match cards[n] {
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append clubs
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(clubs[n]);
                            } else {
                                sorted.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append hearts
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(hearts[n]);
                            } else {
                                sorted.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append spades
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(spades[n]);
                            } else {
                                sorted.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append diamonds
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(diamonds[n]);
                            } else {
                                sorted.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                }
                8..=15 => {
                    // Spades
                    // append spades
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(spades[n]);
                            } else {
                                sorted.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append diamonds
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(diamonds[n]);
                            } else {
                                sorted.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append clubs
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(clubs[n]);
                            } else {
                                sorted.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append hearts
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(hearts[n]);
                            } else {
                                sorted.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append hearts
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(hearts[n]);
                            } else {
                                sorted.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append spades
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(spades[n]);
                            } else {
                                sorted.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append clubs
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(clubs[n]);
                            } else {
                                sorted.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append diamonds
                    pushed_ten = false;
                    ten_found = false;
                    ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(diamonds[n]);
                            } else {
                                sorted.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append diamonds
                    let mut pushed_ten = false;
                    let mut ten_found = false;
                    let mut ten = 25; // DiamondsTen
                    for n in 0..diamonds.len() {
                        // change position for 10
                        if diamonds[n] != ten {
                            if diamonds[n] >= 29 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(diamonds[n]);
                            } else {
                                sorted.push(diamonds[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append clubs
                    pushed_ten = false;
                    ten_found = false;
                    ten = 1; // ClubsTen
                    for n in 0..clubs.len() {
                        // change position for 10
                        if clubs[n] != ten {
                            if clubs[n] >= 5 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(clubs[n]);
                            } else {
                                sorted.push(clubs[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append hearts
                    pushed_ten = false;
                    ten_found = false;
                    ten = 17; // HeartsTen
                    for n in 0..hearts.len() {
                        // change position for 10
                        if hearts[n] != ten {
                            if hearts[n] >= 21 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(hearts[n]);
                            } else {
                                sorted.push(hearts[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                    // append spades
                    pushed_ten = false;
                    ten_found = false;
                    ten = 9; // SpadesTen
                    for n in 0..spades.len() {
                        // change position for 10
                        if spades[n] != ten {
                            if spades[n] >= 13 {
                                if ten_found && !pushed_ten {
                                    sorted.push(ten);
                                    pushed_ten = true;
                                }
                                sorted.push(spades[n]);
                            } else {
                                sorted.push(spades[n]);
                            }
                        } else {
                            // we have a 10 in current set?
                            ten_found = true;
                        }
                    }
                    if ten_found && !pushed_ten {
                        sorted.push(ten);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'c' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                8..=15 => {
                    // Spades
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        's' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                8..=15 => {
                    // Spades
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'h' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                8..=15 => {
                    // Spades
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        'd' => {
            // first find Jacks
            for n in 0..3 {
                match cards[n] {
                    // ClubsJack
                    4 => sorted.push(cards[n]),
                    // SpadesJack
                    12 => sorted.push(cards[n]),
                    // HeartsJack
                    20 => sorted.push(cards[n]),
                    // DiamondsJack
                    28 => sorted.push(cards[n]),
                    // Clubs
                    0..=7 => clubs.push(cards[n]),
                    // Spades
                    8..=15 => spades.push(cards[n]),
                    // Hearts
                    16..=23 => hearts.push(cards[n]),
                    // Diamonds
                    24..=31 => diamonds.push(cards[n]),
                    _ => panic!("Unknown card"),
                }
            }
            // order Jacks
            sorted.sort();
            // order suits
            clubs.sort();
            spades.sort();
            hearts.sort();
            diamonds.sort();
            // append suit of first card first
            match first_card_played {
                0..=7 => {
                    // Clubs
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                8..=15 => {
                    // Spades
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                16..=23 => {
                    // Hearts
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                }
                24..=31 => {
                    // Diamonds
                    // append diamonds
                    for n in 0..diamonds.len() {
                        sorted.push(diamonds[n]);
                    }
                    // append clubs
                    for n in 0..clubs.len() {
                        sorted.push(clubs[n]);
                    }
                    // append spades
                    for n in 0..spades.len() {
                        sorted.push(spades[n]);
                    }
                    // append hearts
                    for n in 0..hearts.len() {
                        sorted.push(hearts[n]);
                    }
                }
                _ => panic!("Unknown card"),
            }
        }
        _ => panic!("Unknown game {}", game),
    }
    sorted
}

fn who_wins_trick(played_cards: &Vec<u8>, game: char) -> u8 {
    let sorted = sort_trick_for(played_cards, game);
    if sorted[0] == played_cards[0] {
        return 0u8;
    }
    if sorted[0] == played_cards[1] {
        return 1u8;
    }
    if sorted[0] == played_cards[2] {
        return 2u8;
    }
    0u8
}

fn main() {
    // handle command line options
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("i", "", "display a recorded Skat game", "FILE");
    opts.optflag("r", "record", "record a Skat game");
    opts.optflag("v", "version", "print version number");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    } else if matches.opt_present("i") {
        let infile = matches.opt_str("i");
        match infile {
            Some(x) => {
                println!("FILE = {}", x);
                let y = x.clone();
                let f = File::open(x).unwrap();
                let reader = BufReader::new(f);
                let mut expect_first_skat = false;
                let mut expect_second_skat = false;
                let mut expect_first = false;
                let mut expect_second = false;
                let mut expect_third = false;
                let mut got_skat = false;
                // build a record with all cards (and none being played)
                let mut record_builder = RecordBuilder::new();
                let cards: Vec<u8> = (0..32).collect();
                for n in 0..32 {
                    record_builder.add(cards[n]);
                }
                let mut record = record_builder.finalize();
                // create three player builders (to collect played cards)
                let mut players: Vec<PlayerBuilder> = Vec::new();
                let player1 = PlayerBuilder::new();
                players.push(player1);
                let player2 = PlayerBuilder::new();
                players.push(player2);
                let player3 = PlayerBuilder::new();
                players.push(player3);
                // provide IDs (are the same as index)
                players[0].id(0);
                players[1].id(1);
                players[2].id(2);
                let mut g: char = 'x';
                let mut leader_id: u8 = 0u8;
                let mut player_id: u8 = leader_id;
                let mut tricks: Vec<Vec<u8>> = Vec::new();
                let mut skat_first: u8 = 0;
                let mut skat_second: u8 = 0;
                let mut first: u8 = 0;
                let mut second: u8 = 0;
                let mut third: u8;
                let mut card_counter: u8 = 0;
                for line in reader.lines() {
                    let input = line.unwrap();
                    // announced game
                    if input == "g".to_string() {
                        println!("Grand announced ...");
                        g = 'g';
                    } else if input == "b".to_string() {
                        println!("Sächsische Spitze announced ...");
                        g = 'b';
                    } else if input == "n".to_string() {
                        println!("Null announced ...");
                        g = 'n';
                    } else if input == "c".to_string() {
                        println!("Clubs announced ...");
                        g = 'c';
                    } else if input == "s".to_string() {
                        println!("Spades announced ...");
                        g = 's';
                    } else if input == "h".to_string() {
                        println!("Hearts announced ...");
                        g = 'h';
                    } else if input == "d".to_string() {
                        println!("Diamonds announced ...");
                        g = 'd';
                    } else {
                        if input == "sf" {
                            // sf = Skat first
                            expect_first_skat = true;
                            println!("Skat first");
                        } else if input == "sl" {
                            // sl = Skat last
                            expect_first = true;
                            println!("Skat last");
                        } else {
                            let card: u8 = match input.trim().parse() {
                                Ok(num) => num,
                                _ => panic!("ERROR: number [0-31] expected {}", input),
                            };
                            if record.is_valid(card) {
                                if expect_first_skat {
                                    skat_first = card;
                                    card_counter += 1;
                                    expect_first_skat = false;
                                    expect_second_skat = true;
                                } else if expect_second_skat {
                                    skat_second = card;
                                    got_skat = true;
                                    card_counter += 1;
                                    expect_second_skat = false;
                                    expect_first = true;
                                } else if card_counter == 30 && expect_first {
                                    // expect_first_skat = true;
                                    skat_first = card;
                                    card_counter += 1;
                                    expect_first_skat = false;
                                    expect_second_skat = true;
                                } else if expect_first {
                                    first = card;
                                    card_counter += 1;
                                    // store played card with current player
                                    players[player_id as usize].add(card);
                                    // select next player
                                    player_id = (player_id + 1) % 3;
                                    // next card
                                    expect_first = false;
                                    expect_second = true;
                                    expect_third = false;
                                } else if expect_second {
                                    second = card;
                                    card_counter += 1;
                                    // store played card with current player
                                    players[player_id as usize].add(card);
                                    // select next player
                                    player_id = (player_id + 1) % 3;
                                    // next card
                                    expect_first = false;
                                    expect_second = false;
                                    expect_third = true;
                                } else if expect_third {
                                    third = card;
                                    card_counter += 1;
                                    // store played card with current player
                                    players[player_id as usize].add(card);
                                    // store trick
                                    let trick = vec![leader_id, first, second, third];
                                    let played_cards = vec![first, second, third];
                                    tricks.push(trick);
                                    // who wins this trick?
                                    leader_id = (leader_id + who_wins_trick(&played_cards, g)) % 3;
                                    player_id = leader_id;
                                    // next card
                                    expect_first = true;
                                    expect_second = false;
                                    expect_third = false;
                                } else {
                                    println!("This is not supposed to happen !!!");
                                }
                            }
                        }
                    }
                }
                // reconstructed distribution of cards

                // example how an input file would look like:
                // static TMP_TEXT: &'static str =
                // "g
                // 12 28 1 2 7 8 16 24 26 27
                // 0 3 5 6 10 13 21 23 25 31
                // 4 20 9 11 14 17 18 22 29 30
                // ";

                // create a name for the .dst file
                let dst_file: String;
                // assume filename ends with ".txt"
                if y.ends_with(".txt") {
                    // replace file extension
                    dst_file = y.replace(".txt", ".dst");
                } else {
                    // use a default name
                    dst_file = String::from("skat_card_distribution.dst");
                }
                let path = Path::new(&dst_file);
                let display = path.display();
                let mut file = match File::create(&path) {
                    Err(why) => panic!("couldn't create {}: {}", display, why.to_string()),
                    Ok(file) => file,
                };
                // collect info here
                let mut text = String::new();
                // start with current game (in one line)
                text.push(g);
                text.push('\n');
                for m in 0..3 {
                    let mut player = players[m].finalize();
                    player.sort_cards_for(g);
                    let cards_str = player.print_cards(false);
                    text.push_str(&cards_str);
                }
                match file.write_all(text.as_bytes()) {
                    Err(why) => {
                        panic!("couldn't write to {}: {}",
                               display,
                               why.to_string())
                    }
                    Ok(_) => println!("successfully wrote to {}", display),
                }
                for m in 0..tricks.len() {
                    let ref trick = &tricks[m];
                    match trick[0] {
                        0 => print!("A:"),
                        1 => print!("B:"),
                        2 => print!("C:"),
                        _ => panic!("Unknown player {}", trick[0]),
                    }
                    print_card(trick[1], true);
                    print_card(trick[2], true);
                    print_card(trick[3], true);
                    println!("");
                }
                if got_skat {
                    // print Skat
                    let skat = SkatBuilder::new()
                        .add(skat_first, skat_second)
                        .finalize();
                    skat.print_cards();
                }
            }
            None => panic!("no input file name"),
        }
        return;
    } else if matches.opt_present("r") {
        loop {
            // highest bid
            println!("highest bid:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let bid: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            println!("highest bid = {:?}", bid);
            // ask for announced game
            println!("announced game [gbncshd]:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let g: char;
            if input == "g\n".to_string() {
                println!("Grand announced ...");
                g = 'g';
            } else if input == "b\n".to_string() {
                println!("Sächsische Spitze announced ...");
                g = 'b';
            } else if input == "n\n".to_string() {
                println!("Null announced ...");
                g = 'n';
            } else if input == "c\n".to_string() {
                println!("Clubs announced ...");
                g = 'c';
            } else if input == "s\n".to_string() {
                println!("Spades announced ...");
                g = 's';
            } else if input == "h\n".to_string() {
                println!("Hearts announced ...");
                g = 'h';
            } else if input == "d\n".to_string() {
                println!("Diamonds announced ...");
                g = 'd';
            } else {
                break;
            }
            // Hand?
            let mut hand = false;
            println!("Hand? Press '1' for yes:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if input == 1 {
                hand = true;
                println!("Hand = yes");
            } else {
                println!("Hand = no");
            }
            // Declarer?
            let mut declarer = false;
            println!("Are you the declarer? Press '1' for yes:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            if input == 1 {
                declarer = true;
                println!("Declarer = yes");
            } else {
                println!("Declarer = no");
            }
            // build a record with all cards (and none being played)
            let mut record_builder = RecordBuilder::new();
            let cards: Vec<u8> = (0..32).collect();
            for n in 0..32 {
                record_builder.add(cards[n]);
            }
            let mut record = record_builder.finalize();
            // create three player builders (to collect played cards)
            let mut players: Vec<PlayerBuilder> = Vec::new();
            let player1 = PlayerBuilder::new();
            players.push(player1);
            let player2 = PlayerBuilder::new();
            players.push(player2);
            let player3 = PlayerBuilder::new();
            players.push(player3);
            // provide IDs (are the same as index)
            players[0].id(0);
            players[1].id(1);
            players[2].id(2);
            // print cards
            record.print_cards(g);
            // do we know what's in the Skat?
            if declarer && !hand {
                // mark Skat as played
                for _m in 0..2 {
                    loop {
                        // ask for card to play
                        println!("card to play?");
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .ok()
                            .expect("failed to read line");
                        let input: u8 = match input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => 0,
                        };
                        if record.is_valid(input) {
                            record.print_cards(g);
                            break;
                        }
                    }
                }
            }
            // for each trick
            let mut abort: bool = false;
            let mut leader_id: u8 = 0u8;
            let mut tricks: Vec<Vec<u8>> = Vec::new();
            for _n in 0..10 {
                let mut played_cards: Vec<u8> = Vec::new();
                // for each player
                let mut player_id: u8 = leader_id;
                let mut trick: Vec<u8> = Vec::new();
                trick.push(leader_id);
                for _m in 0..3 {
                    loop {
                        // ask for card to play
                        println!("card to play?");
                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .ok()
                            .expect("failed to read line");
                        let input: u8 = match input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => 0,
                        };
                        if input == 32u8 {
                            abort = true;
                            break;
                        }
                        if record.is_valid(input) {
                            trick.push(input);
                            // store played card with current player
                            players[player_id as usize].add(input);
                            played_cards.push(input);
                            // print cards
                            record.print_cards(g);
                            // select next player
                            player_id = (player_id + 1) % 3;
                            break;
                        }
                    }
                    if abort {
                        break;
                    }
                }
                if abort {
                    break;
                } else {
                    // store trick
                    tricks.push(trick);
                    // who wins this trick?
                    leader_id = (leader_id + who_wins_trick(&played_cards, g)) % 3;
                }
            }
            // reconstructed distribution of cards
            for m in 0..3 {
                let mut player = players[m].finalize();
                player.sort_cards_for(g);
                player.print_cards(false);
            }
            for m in 0..tricks.len() {
                let ref trick = &tricks[m];
                match trick[0] {
                    0 => print!("A:"),
                    1 => print!("B:"),
                    2 => print!("C:"),
                    _ => panic!("Unknown player {}", trick[0]),
                }
                print_card(trick[1], true);
                print_card(trick[2], true);
                print_card(trick[3], true);
                println!("");
            }
            // continue?
            println!("New game? [press 'q' to quit]");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .ok()
                .expect("failed to read line");
            if input == "q\n".to_string() {
                break;
            }
        }
        return;
    } else if matches.opt_present("v") {
        print_version(&program);
        return;
    }
    // keep scores
    let mut score: [i32; 3] = [0, 0, 0];
    let mut round_counter = 0u16;
    // randomly select player
    let mut player_id: u8 = rand::thread_rng().gen_range(0, 3);
    loop {
        round_counter += 1;
        // player with player_id is dealing
        let (mut dealer, mut responder, mut bidder, skat) = deal(player_id);
        // bid
        let (declarer_id, game_value, mut sorted_game) = bid(&mut dealer,
                                                             &mut responder,
                                                             &mut bidder,
                                                             (player_id + 1) % 3);
        if game_value == 0 {
            println!("nobody wants to play ... continue with next player");
            // next round
            player_id = (player_id + 1) % 3;
            // continue with next player
            continue;
        }
        // announce game
        let mut hand = false;
        let mut ouvert = false;
        let mut matadors: u8 = 0u8;
        if dealer.id == declarer_id {
            dealer = dealer.announce_game(&mut sorted_game,
                                          &skat,
                                          // return values
                                          &mut hand,
                                          &mut ouvert,
                                          &mut matadors);
        } else if responder.id == declarer_id {
            responder = responder.announce_game(&mut sorted_game,
                                                &skat,
                                                // return values
                                                &mut hand,
                                                &mut ouvert,
                                                &mut matadors);
        } else if bidder.id == declarer_id {
            bidder = bidder.announce_game(&mut sorted_game,
                                          &skat,
                                          // return values
                                          &mut hand,
                                          &mut ouvert,
                                          &mut matadors);
        }
        // all players sort for game
        dealer.sort_cards_for(sorted_game);
        responder.sort_cards_for(sorted_game);
        bidder.sort_cards_for(sorted_game);
        let mut leader_id: u8 = responder.id;
        // play 10 tricks in a row
        let player_name = match declarer_id {
            0 => "A",
            1 => "B",
            2 => "C",
            _ => panic!("Unknown player {}", declarer_id),
        };
        let mut hand_announced = " ".to_string();
        let mut ouvert_announced = "".to_string();
        let game = match sorted_game {
            'g' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Grand"
            }
            'b' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Sächsische Spitze"
            }
            'n' => {
                if hand {
                    hand_announced.push_str("Hand ");
                }
                if ouvert {
                    ouvert_announced.push_str("Ouvert ");
                    if hand {
                        // Null Ouvert Hand (instead of Null Hand Ouvert)
                        hand_announced = " Ouvert Hand ".to_string();
                        ouvert_announced = "".to_string(); // see above
                    }
                }
                "Null"
            }
            'c' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Clubs"
            }
            's' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Spades"
            }
            'h' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Hearts"
            }
            'd' => {
                if hand {
                    hand_announced.push_str("Hand ");
                    if ouvert {
                        ouvert_announced.push_str("Ouvert ");
                    } else {
                        // there is only Grand Hand Ouvert
                        ouvert = false;
                    }
                }
                "Diamonds"
            }
            _ => panic!("Unknown game {}", sorted_game),
        };
        'trick_loop: for trick in 0..10 {
            println!("#########");
            println!("trick #{}:", trick);
            println!("#########");
            println!("Player {} plays {}{}{}bidding {}",
                     player_name,
                     game,
                     hand_announced,
                     ouvert_announced,
                     game_value);
            responder.print_counter();
            bidder.print_counter();
            dealer.print_counter();
            let mut played_cards: Vec<u8> = Vec::new();
            // use player to detect first card played
            for player in 0..3 {
                if dealer.id == leader_id {
                    dealer.print_cards(false);
                    dealer.play_card(&mut played_cards, player, sorted_game);
                } else if responder.id == leader_id {
                    responder.print_cards(false);
                    responder.play_card(&mut played_cards, player, sorted_game);
                } else if bidder.id == leader_id {
                    bidder.print_cards(false);
                    bidder.play_card(&mut played_cards, player, sorted_game);
                }
                // select next player
                leader_id = (leader_id + 1) % 3;
            }
            println!("played cards: ");
            print_card(played_cards[0], true);
            print_card(played_cards[1], true);
            print_card(played_cards[2], true);
            println!("");
            // who wins this trick?
            let winner_id: u8 = (leader_id + who_wins_trick(&played_cards, sorted_game)) % 3;
            let winner_name = match winner_id {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => panic!("Unknown player {}", winner_id),
            };
            println!("Player {} wins trick {} ...", winner_name, trick);
            if dealer.id == winner_id {
                dealer.add_trick(&played_cards);
            } else if responder.id == winner_id {
                responder.add_trick(&played_cards);
            } else if bidder.id == winner_id {
                bidder.add_trick(&played_cards);
            }
            // set leader_id
            leader_id = winner_id;
            // if we play Null break loop early
            if sorted_game == 'n' && winner_id == declarer_id {
                break 'trick_loop;
            }
        }
        // count cards
        let mut declarer_count: u8 = 0u8;
        let mut team_count: u8 = 0u8;
        let mut tricks_len: usize = 0usize;
        println!("Dealer: {}", dealer.count_cards());
        println!("Responder: {}", responder.count_cards());
        println!("Bidder: {}", bidder.count_cards());
        if dealer.id == declarer_id {
            declarer_count = dealer.count_cards();
            tricks_len = dealer.tricks.len();
            team_count = responder.count_cards() + bidder.count_cards();
        } else if responder.id == declarer_id {
            declarer_count = responder.count_cards();
            tricks_len = responder.tricks.len();
            team_count = dealer.count_cards() + bidder.count_cards();
        } else if bidder.id == declarer_id {
            declarer_count = bidder.count_cards();
            tricks_len = bidder.tricks.len();
            team_count = responder.count_cards() + dealer.count_cards();
        }
        // announce winner of this round
        println!("###########################");
        println!("hand = {:?}", hand);
        println!("ouvert = {:?}", ouvert);
        println!("matadors = {:?}", matadors);
        // declarer's game value
        let schneider_announced: bool = false; // TODO
        let schwarz_announced: bool = false; // TODO
        let mut decl_game_value: u8 = matadors;
        if hand {
            // print Skat
            skat.print_cards();
            // game
            decl_game_value += 1;
            // hand
            decl_game_value += 1;
            if declarer_count >= 90u8 {
                // Schneider
                decl_game_value += 1;
                if schneider_announced {
                    // Schneider announced
                    decl_game_value += 1;
                }
            }
            if declarer_count == 120u8 && tricks_len == 30 {
                // Schwarz
                decl_game_value += 1;
                if schwarz_announced {
                    // Schwarz announced
                    decl_game_value += 1;
                }
            }
            if ouvert {
                // Ouvert
                decl_game_value += 1;
            }
            match sorted_game {
                'g' => {
                    decl_game_value *= 24;
                }
                'b' => {
                    decl_game_value *= 20;
                }
                'n' => {
                    if ouvert {
                        // Null Ouvert Hand
                        decl_game_value = 59;
                    } else {
                        // Null Hand
                        decl_game_value = 35;
                    }
                }
                'c' => {
                    decl_game_value *= 12;
                }
                's' => {
                    decl_game_value *= 11;
                }
                'h' => {
                    decl_game_value *= 10;
                }
                'd' => {
                    decl_game_value *= 9;
                }
                _ => panic!("Unknown game {}", sorted_game),
            }
        } else {
            // game
            decl_game_value += 1;
            if declarer_count >= 90u8 {
                // Schneider
                decl_game_value += 1;
            }
            if declarer_count == 120u8 && tricks_len == 30 {
                // Schwarz
                decl_game_value += 1;
            }
            match sorted_game {
                'g' => {
                    decl_game_value *= 24;
                }
                'b' => {
                    decl_game_value *= 20;
                }
                'n' => {
                    if ouvert {
                        // Null Ouvert
                        decl_game_value = 46;
                    } else {
                        // Null
                        decl_game_value = 23;
                    }
                }
                'c' => {
                    decl_game_value *= 12;
                }
                's' => {
                    decl_game_value *= 11;
                }
                'h' => {
                    decl_game_value *= 10;
                }
                'd' => {
                    decl_game_value *= 9;
                }

                _ => panic!("Unknown game {}", sorted_game),
            }
        }
        if sorted_game == 'n' {
            // check Null first
            if declarer_count == 0 || tricks_len == 0 {
                println!("declarer wins {}{}{}with {} to {}",
                         game,
                         hand_announced,
                         ouvert_announced,
                         declarer_count,
                         team_count);
            } else {
                println!("declarer looses {}{}{}with {} to {}",
                         game,
                         hand_announced,
                         ouvert_announced,
                         declarer_count,
                         team_count);
            }
        } else {
            if declarer_count > team_count {
                println!("declarer wins {}{}{}with {} to {}",
                         game,
                         hand_announced,
                         ouvert_announced,
                         declarer_count,
                         team_count);
            } else {
                println!("declarer looses {}{}{}with {} to {}",
                         game,
                         hand_announced,
                         ouvert_announced,
                         declarer_count,
                         team_count);
            }
        }
        println!("game_value = {:?}", game_value);
        println!("decl_game_value = {:?}", decl_game_value);
        // summarize ...
        // ... before
        println!("before round {}: {:?}", round_counter, score);
        if sorted_game == 'n' {
            // check Null first
            if declarer_count == 0 || tricks_len == 0 {
                // declarer wins
                match decl_game_value {
                    23 => {
                        // Null
                        score[declarer_id as usize] += 23;
                    }
                    35 => {
                        // Null Hand
                        score[declarer_id as usize] += 35;
                    }
                    46 => {
                        // Null Ouvert
                        score[declarer_id as usize] += 46;
                    }
                    59 => {
                        // Null Ouvert Hand
                        score[declarer_id as usize] += 59;
                    }
                    _ => panic!("Unknown game value {}, Null expected", decl_game_value),
                }
            } else {
                // declarer looses
                score[declarer_id as usize] -= 2 * decl_game_value as i32;
            }
        } else {
            if declarer_count > team_count {
                if decl_game_value >= game_value {
                    // declarer wins
                    score[declarer_id as usize] += decl_game_value as i32;
                } else {
                    // declarer looses
                    score[declarer_id as usize] -= 2 * decl_game_value as i32;
                }
            } else {
                // declarer looses
                score[declarer_id as usize] -= 2 * decl_game_value as i32;
            }
        }
        // ... and after
        println!("after round {}: {:?}", round_counter, score);
        // continue?
        println!("New game? [press 'q' to quit]");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "q\n".to_string() {
            break;
        }
        // next round
        player_id = (player_id + 1) % 3;
    }
}
