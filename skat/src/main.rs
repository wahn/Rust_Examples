extern crate ansi_term;
extern crate rand;

use ansi_term::Colour::*;
use rand::Rng;
use std::io;

// enum Card {
//     ClubsAce,      //  0
//     ClubsTen,      //  1
//     ClubsKing,     //  2
//     ClubsQueen,    //  3
//     ClubsJack,     //  4
//     ClubsNine,     //  5
//     ClubsEight,    //  6
//     ClubsSeven,    //  7
//     SpadesAce,     //  8
//     SpadesTen,     //  9
//     SpadesKing,    // 10
//     SpadesQueen,   // 11
//     SpadesJack,    // 12
//     SpadesNine,    // 13
//     SpadesEight,   // 14
//     SpadesSeven,   // 15
//     HeartsAce,     // 16
//     HeartsTen,     // 17
//     HeartsKing,    // 18
//     HeartsQueen,   // 19
//     HeartsJack,    // 20
//     HeartsNine,    // 21
//     HeartsEight,   // 22
//     HeartsSeven,   // 23
//     DiamondsAce,   // 24
//     DiamondsTen,   // 25
//     DiamondsKing,  // 26
//     DiamondsQueen, // 27
//     DiamondsJack,  // 28
//     DiamondsNine,  // 29
//     DiamondsEight, // 30
//     DiamondsSeven, // 31
// }

// enum Game {
//     Suit,
//     Grand,
//     Null,
// }

// enum PlayerId {
//     A,
//     B,
//     C,
// }

struct Player {
    id: u8,
    cards: Vec<u8>,
}

impl Player {
    fn allow_sorting(&mut self) -> char {
        let mut g: char = 'd';
        loop {
            println!("sort for?");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .ok()
                .expect("failed to read line");
            if input == "g\n".to_string() {
                println!("sort for Grand ...");
                g = 'g';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else if input == "n\n".to_string() {
                println!("sort for Null ...");
                g = 'n';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else if input == "c\n".to_string() {
                println!("sort for Clubs ...");
                g = 'c';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else if input == "s\n".to_string() {
                println!("sort for Spades ...");
                g = 's';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else if input == "h\n".to_string() {
                println!("sort for Hearts ...");
                g = 'h';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else if input == "d\n".to_string() {
                println!("sort for Diamonds ...");
                g = 'd';
                self.sort_cards_for(g);
                self.print_cards();
                println!("");
            } else {
                break;
            }
        }
        g
    }

    fn print_cards(&self) {
        match self.id {
            0 => println!("player A:"),
            1 => println!("player B:"),
            2 => println!("player C:"),
            _ => panic!("Unknown player {}", self.id),
        }
        for index in 0..10 {
            print!("{}:", index);
            print_card(self.cards[index]);
        }
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
                0 ... 7 => clubs.push(self.cards[n]),
                // Spades
                8 ... 15 => spades.push(self.cards[n]),
                // Hearts
                16 ... 23 => hearts.push(self.cards[n]),
                // Diamonds
                24 ... 31 => diamonds.push(self.cards[n]),
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
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            'n' => {
                // println!("Null");

                for n in 0..10 {
                    match self.cards[n] {
                        // Clubs
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            'c' => {
                // println!("Clubs");

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
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            's' => {
                // println!("Spades");

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
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            'h' => {
                // println!("Hearts");

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
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            'd' => {
                // println!("Diamonds");

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
                        0 ... 7 => clubs.push(self.cards[n]),
                        // Spades
                        8 ... 15 => spades.push(self.cards[n]),
                        // Hearts
                        16 ... 23 => hearts.push(self.cards[n]),
                        // Diamonds
                        24 ... 31 => diamonds.push(self.cards[n]),
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
            },
            _   => panic!("Unknown game {}", game),
        }
        self.cards = sorted;
    }
}

struct PlayerBuilder {
    id: u8,
    cards: Vec<u8>,
}

impl PlayerBuilder {
    fn new() -> PlayerBuilder {
        PlayerBuilder { cards: Vec::new(), id: 3, }
    }

    fn add(&mut self, new_card: u8) -> &mut PlayerBuilder {
        self.cards.push(new_card);
        self
    }

    fn drop1(&mut self) -> &mut PlayerBuilder {
        self.cards.sort();
        self.print_cards();
        println!("");
        loop {
            println!("drop:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            match input {
                0 ... 11 => {
                    println!("{} chosen ...", input);
                    self.cards.remove(input as usize);
                    break;
                },
                _ => continue,
            }
        }
        self
    }

    fn drop2(&mut self) -> &mut PlayerBuilder {
        self.cards.sort();
        self.print_cards();
        println!("");
        loop {
            println!("drop:");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .ok()
                .expect("failed to read line");
            let input: u8 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
            };
            match input {
                0 ... 10 => {
                    println!("{} chosen ...", input);
                    self.cards.remove(input as usize);
                    break;
                },
                _ => continue,
            }
        }
        self.print_cards();
        println!("");
        self
    }

    fn id(&mut self, new_id: u8) -> &mut PlayerBuilder {
        self.id = new_id;
        self
    }

    fn print_cards(&self) {
        let len = self.cards.len();
        for index in 0..len {
            print!("{}:", index);
            print_card(self.cards[index]);
        }
    }

    fn take(&mut self, skat: &Skat) -> &mut PlayerBuilder {
        self.cards.push(skat.first);
        self.cards.push(skat.second);
        self
    }

    fn finalize(&self) -> Player {
        Player { id: self.id, cards: self.cards.to_vec(), }
    }
}

struct Skat {
    first:  u8,
    second: u8,
}

impl Skat {
    fn print_cards(&self) {
        println!("Skat:");
        print_card(self.first);
        print_card(self.second);
        println!("");
    }
}

struct SkatBuilder {
    first:  u8,
    second: u8,
}

impl SkatBuilder {
    fn new() -> SkatBuilder {
        SkatBuilder { first: 0u8, second: 0u8, }
    }

    fn add(&mut self, f: u8, s: u8) -> &mut SkatBuilder {
        self.first  = f;
        self.second = s;
        self
    }

    fn finalize(&self) -> Skat {
        Skat { first: self.first, second: self.second, }
    }
}

fn announce(game: char) -> char {
    let mut g: char = game; // copy game value (can still be changed)
    print!("announce game [gncshd] or y for ");
    loop {
        match g {
            'g' => println!("Grand?"),
            'n' => println!("Null?"),
            'c' => println!("Clubs?"),
            's' => println!("Spades?"),
            'h' => println!("Hearts?"),
            'd' => println!("Diamonds?"),
            _   => { println!("Choose game [gncshd]:"); g = 'd'; },
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "y\n".to_string() || input == "\n" {
            return g;
        } else if input == "g\n".to_string() {
            g = 'g';
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
        }
    }
}

fn bid(dealer: &mut Player,
       responder: &mut Player,
       bidder: &mut Player) -> (u8, u8, char) {
    // return values
    let mut winner: u8;
    let mut lowest: u8;
    let mut game:char;
    // bidder sees his cards first
    bidder.print_cards();
    println!("");
    // ask for input
    let mut bidder_bid: u8;
    let mut g: char = 'd';
    loop {
        println!("bid (or [gncshd]):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("sort for Grand ...");
            g = 'g';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            bidder.sort_cards_for(g);
            bidder.print_cards();
            println!("");
        }
        bidder_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid(bidder_bid) {
            break;
        } else {
            continue;
        }
    }
    // keep selected (by sorting) game
    let bidder_game = g;
    println!("bidder: {}", bidder_bid);
    // responder is next
    responder.print_cards();
    println!("");
    // ask for input
    let mut responder_bid: u8;
    loop {
        println!("bid (or [gncshd]):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("sort for Grand ...");
            g = 'g';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            responder.sort_cards_for(g);
            responder.print_cards();
            println!("");
        }
        responder_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid(responder_bid) {
            break;
        } else {
            continue;
        }
    }
    let responder_game = g;
    println!("responder: {}", responder_bid);
    // ask dealer last
    dealer.print_cards();
    println!("");
    // ask for input
    let mut dealer_bid: u8;
    loop {
        println!("bid (or [gncshd]):");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("sort for Grand ...");
            g = 'g';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            dealer.sort_cards_for(g);
            dealer.print_cards();
            println!("");
        }
        dealer_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if is_valid(dealer_bid) {
            break;
        } else {
            continue;
        }
    }
    let dealer_game = g;
    println!("dealer: {}", dealer_bid);
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
                println!("Player {} wins bidding with {} ...",
                         winner_name, lowest);
                return (winner, lowest, game);
            }
        } else { // responder_bid > bidder_bid
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
    } else { // winner == bidder.id
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
                    if is_valid(lowest) {
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
    dealer.print_cards();
    println!("");
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
    left.print_cards();
    println!("");
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
    right.print_cards();
    println!("");
    // Skat
    let skat = SkatBuilder::new()
        .add(shuffled[30], shuffled[31])
        .finalize();
    println!("");
    skat.print_cards();
    println!("");
    (dealer, left, right, skat)
}

fn is_valid(bid: u8) -> bool {
    let mut valid;
    // Null game (have to be checked first)
    let nulls = [23, 35, 46, 59];
    valid = match nulls.iter().find(|&&x| x == bid) {
        Some(_) => true,
        None    => false,
    };
    // Suit game
    if (bid % 12) == 0 { valid = true; }
    if (bid % 11) == 0 { valid = true; }
    if (bid % 10) == 0 { valid = true; }
    if (bid %  9) == 0 { valid = true; }
    // 13 * 12 = 4 Jacks, 6 trumps, Game, Schneider, Schwarz
    let highest: u8 = 156;
    if bid > highest { valid = false; }
    // Grand game
    if (bid % 24) == 0 { valid = true; }
    // 7 * 24 = 4 Jacks, Game, Schneider, Schwarz
    let highest: u8 = 168;
    if bid > highest { valid = false; }
    valid
}

fn print_card(card: u8) {
    let club    = "♣";
    let spade   = "♠";
    let heart   = "♥";
    let diamond = "♦";
    match card {
        0 => {
            let mut ace = " A".to_string();
            ace.push_str(club);
            print!("{} ", Black.bold().paint(&ace));
        },
        1 => {
            let mut ten = "10".to_string();
            ten.push_str(club);
            print!("{} ", Black.bold().paint(&ten));
        },
        2 => {
            let mut king = " K".to_string();
            king.push_str(club);
            print!("{} ", Black.bold().paint(&king));
        },
        3 => {
            let mut queen = " Q".to_string();
            queen.push_str(club);
            print!("{} ", Black.bold().paint(&queen));
        },
        4 => {
            let mut jack = " J".to_string();
            jack.push_str(club);
            print!("{} ", Black.bold().paint(&jack));
        },
        5 => {
            let mut nine = " 9".to_string();
            nine.push_str(club);
            print!("{} ", Black.bold().paint(&nine));
        },
        6 => {
            let mut eight = " 8".to_string();
            eight.push_str(club);
            print!("{} ", Black.bold().paint(&eight));
        },
        7 => {
            let mut seven = " 7".to_string();
            seven.push_str(club);
            print!("{} ", Black.bold().paint(&seven));
        },
        8 => {
            let mut ace = " A".to_string();
            ace.push_str(spade);
            print!("{} ", Green.bold().paint(&ace));
        },
        9 => {
            let mut ten = "10".to_string();
            ten.push_str(spade);
            print!("{} ", Green.bold().paint(&ten));
        },
        10 => {
            let mut king = " K".to_string();
            king.push_str(spade);
            print!("{} ", Green.bold().paint(&king));
        },
        11 => {
            let mut queen = " Q".to_string();
            queen.push_str(spade);
            print!("{} ", Green.bold().paint(&queen));
        },
        12 => {
            let mut jack = " J".to_string();
            jack.push_str(spade);
            print!("{} ", Green.bold().paint(&jack));
        },
        13 => {
            let mut nine = " 9".to_string();
            nine.push_str(spade);
            print!("{} ", Green.bold().paint(&nine));
        },
        14 => {
            let mut eight = " 8".to_string();
            eight.push_str(spade);
            print!("{} ", Green.bold().paint(&eight));
        },
        15 => {
            let mut seven = " 7".to_string();
            seven.push_str(spade);
            print!("{} ", Green.bold().paint(&seven));
        },
        16 => {
            let mut ace = " A".to_string();
            ace.push_str(heart);
            print!("{} ", Red.bold().paint(&ace));
        },
        17 => {
            let mut ten = "10".to_string();
            ten.push_str(heart);
            print!("{} ", Red.bold().paint(&ten));
        },
        18 => {
            let mut king = " K".to_string();
            king.push_str(heart);
            print!("{} ", Red.bold().paint(&king));
        },
        19 => {
            let mut queen = " Q".to_string();
            queen.push_str(heart);
            print!("{} ", Red.bold().paint(&queen));
        },
        20 => {
            let mut jack = " J".to_string();
            jack.push_str(heart);
            print!("{} ", Red.bold().paint(&jack));
        },
        21 => {
            let mut nine = " 9".to_string();
            nine.push_str(heart);
            print!("{} ", Red.bold().paint(&nine));
        },
        22 => {
            let mut eight = " 8".to_string();
            eight.push_str(heart);
            print!("{} ", Red.bold().paint(&eight));
        },
        23 => {
            let mut seven = " 7".to_string();
            seven.push_str(heart);
            print!("{} ", Red.bold().paint(&seven));
        },
        24 => {
            let mut ace = " A".to_string();
            ace.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&ace));
        },
        25 => {
            let mut ten = "10".to_string();
            ten.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&ten));
        },
        26 => {
            let mut king = " K".to_string();
            king.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&king));
        },
        27 => {
            let mut queen = " Q".to_string();
            queen.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&queen));
        },
        28 => {
            let mut jack = " J".to_string();
            jack.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&jack));
        },
        29 => {
            let mut nine = " 9".to_string();
            nine.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&nine));
        },
        30 => {
            let mut eight = " 8".to_string();
            eight.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&eight));
        },
        31 => {
            let mut seven = " 7".to_string();
            seven.push_str(diamond);
            print!("{} ", Yellow.bold().paint(&seven));
        },
        _ => panic!("Unknown card"),
    }
}

fn main() {
    // randomly select player
    let mut player_id: u8 = rand::thread_rng().gen_range(0, 3);
    loop {
        // player with player_id is dealing
        let (mut dealer,
             mut responder,
             mut bidder,
             mut skat) = deal(player_id);
        // bid
        let (declarer_id, game_value, mut sorted_game) = bid(&mut dealer,
                                                             &mut responder,
                                                             &mut bidder);
        // print cards before announcing the game
        let mut declarer: &Player = &dealer; // assumes bid() returns a valid id
        if dealer.id == declarer_id {
            declarer = &dealer;
        } else if responder.id == declarer_id {
            declarer = &responder;
        } else if bidder.id == declarer_id {
            declarer = &bidder;
        }
        declarer.print_cards();
        println!("");
        println!("Do you want to see the Skat? Press '1' for yes:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        let input: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,
        };
        let mut player_builder = &mut PlayerBuilder::new();
        player_builder
            .id(declarer.id)
            // copy cards from declarer
            .add(declarer.cards[0])
            .add(declarer.cards[1])
            .add(declarer.cards[2])
            .add(declarer.cards[3])
            .add(declarer.cards[4])
            .add(declarer.cards[5])
            .add(declarer.cards[6])
            .add(declarer.cards[7])
            .add(declarer.cards[8])
            .add(declarer.cards[9]);
        if input == 1 {
            skat.print_cards();
            player_builder
                // take skat
                .take(&skat)
                // drop (interactively) two cards
                .drop1()
                .drop2();
        }
        let mut declarer = &mut player_builder.finalize();
        if input == 1 {
            // sort (again) after dropping cards
            sorted_game = declarer.allow_sorting();
        }
        // announce
        let game = announce(sorted_game);
        match game {
            'g' => println!("Grand announced ..."),
            'n' => println!("Null announced ..."),
            'c' => println!("Clubs announced ..."),
            's' => println!("Spades announced ..."),
            'h' => println!("Hearts announced ..."),
            'd' => println!("Diamonds announced ..."),
            _   => panic!("Unknown game announced"),
        }
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
        // next round
        player_id = (player_id + 1) % 3;
    }
}
