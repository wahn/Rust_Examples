extern crate ansi_term;
extern crate rand;

use ansi_term::Colour::*;
use rand::Rng;
use std::io;

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
            } else if input == "n\n".to_string() {
                println!("sort for Null ...");
                g = 'n';
                self.sort_cards_for(g);
                self.print_cards();
            } else if input == "c\n".to_string() {
                println!("sort for Clubs ...");
                g = 'c';
                self.sort_cards_for(g);
                self.print_cards();
            } else if input == "s\n".to_string() {
                println!("sort for Spades ...");
                g = 's';
                self.sort_cards_for(g);
                self.print_cards();
            } else if input == "h\n".to_string() {
                println!("sort for Hearts ...");
                g = 'h';
                self.sort_cards_for(g);
                self.print_cards();
            } else if input == "d\n".to_string() {
                println!("sort for Diamonds ...");
                g = 'd';
                self.sort_cards_for(g);
                self.print_cards();
            } else {
                break;
            }
        }
        g
    }

    fn announce_game(&mut self, sorted_game: &mut char, skat: &Skat,
                     hand: &mut bool, overt: &mut bool)
                     -> Player {
        // print cards before announcing the game
        self.print_cards();
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
        if input == 1 {
            skat.print_cards();
            player_builder
                // take skat
                .take(&skat)
                // drop (interactively) two cards
                .drop1()
                .drop2();
        } else {
            // player still gets the Skat for counting
            player_builder.do_not_take(&skat);
            *hand = true;
        }
        let mut player = player_builder.finalize();
        if input == 1 {
            *sorted_game = player.allow_sorting();
        }
        // announce
        *sorted_game = announce(*sorted_game as char);
        match *sorted_game {
            // TODO: overt
            'g' => {
                if *hand {
                    println!("Grand Hand announced ...");
                } else {
                    println!("Grand announced ...");
                }
            },
            'n' => {
                if *hand {
                    println!("Null Hand announced ...");
                } else {
                    println!("Null announced ...");
                }
            },
            'c' => {
                if *hand {
                    println!("Clubs Hand announced ...");
                } else {
                    println!("Clubs announced ...");
                }
            },
            's' => {
                if *hand {
                    println!("Spades Hand announced ...");
                } else {
                    println!("Spades announced ...");
                }
            },
            'h' => {
                if *hand {
                    println!("Hearts Hand announced ...");
                } else {
                    println!("Hearts announced ...");
                }
            },
            'd' => {
                if *hand {
                    println!("Diamonds Hand announced ...");
                    } else {
                    println!("Diamonds announced ...");
                }
            },
            _   => panic!("Unknown game announced"),
        }
        player
    }

    fn count_cards(&self) -> u8 {
        self.counter
    }

    fn play_card(&mut self, mut played_cards: &mut Vec<u8>,
                 player: u8, game: char) {
        println!("player = {:?}", player);
        println!("played cards = {:?}", played_cards);
        let mut first_card;
        loop {
            println!("choose card [0-{}]:", self.cards.len() - 1);
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
                    let card: u8 = self.cards[input as usize];
                    if player != 0 {
                        first_card = played_cards[0];
                        if is_valid_card(card, first_card, game) {
                            println!("is valid");
                        } else {
                            println!("is NOT valid");
                        }
                    }
                    print_card(card);
                    println!("");
                    played_cards.push(self.cards.remove(input as usize));
                    break;
                },
                _ => continue,
            }
        }
    }

    fn print_cards(&self) {
        match self.id {
            0 => println!("player A:"),
            1 => println!("player B:"),
            2 => println!("player C:"),
            _ => panic!("Unknown player {}", self.id),
        }
        for index in 0..self.cards.len() {
            print!("{}:", index);
            print_card(self.cards[index]);
        }
        println!("");
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
        PlayerBuilder { cards: Vec::new(), id: 3u8, counter: 0u8 }
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
                    let card = self.cards.remove(input as usize);
                    self.counter += Player::value_of(card);
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
                    let card = self.cards.remove(input as usize);
                    self.counter += Player::value_of(card);
                    break;
                },
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

    fn print_cards(&self) {
        let len = self.cards.len();
        for index in 0..len {
            print!("{}:", index);
            print_card(self.cards[index]);
        }
        println!("");
    }

    fn take(&mut self, skat: &Skat) -> &mut PlayerBuilder {
        self.cards.push(skat.first);
        self.cards.push(skat.second);
        self
    }

    fn finalize(&self) -> Player {
        Player { id: self.id,
                 cards: self.cards.to_vec(),
                 tricks: Vec::new(),
                 counter: self.counter, }
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
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            bidder.sort_cards_for(g);
            bidder.print_cards();
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            bidder.sort_cards_for(g);
            bidder.print_cards();
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            bidder.sort_cards_for(g);
            bidder.print_cards();
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            bidder.sort_cards_for(g);
            bidder.print_cards();
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            bidder.sort_cards_for(g);
            bidder.print_cards();
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
    println!("bidder: {}", bidder_bid);
    // responder is next
    responder.print_cards();
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
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            responder.sort_cards_for(g);
            responder.print_cards();
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            responder.sort_cards_for(g);
            responder.print_cards();
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            responder.sort_cards_for(g);
            responder.print_cards();
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            responder.sort_cards_for(g);
            responder.print_cards();
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            responder.sort_cards_for(g);
            responder.print_cards();
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
    println!("responder: {}", responder_bid);
    // ask dealer last
    dealer.print_cards();
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
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            g = 'n';
            dealer.sort_cards_for(g);
            dealer.print_cards();
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            g = 'c';
            dealer.sort_cards_for(g);
            dealer.print_cards();
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            g = 's';
            dealer.sort_cards_for(g);
            dealer.print_cards();
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            g = 'h';
            dealer.sort_cards_for(g);
            dealer.print_cards();
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            g = 'd';
            dealer.sort_cards_for(g);
            dealer.print_cards();
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
    dealer.print_cards();
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
    // Skat
    let skat = SkatBuilder::new()
        .add(shuffled[30], shuffled[31])
        .finalize();
    println!("");
    skat.print_cards();
    println!("");
    (dealer, left, right, skat)
}

fn is_valid_bid(bid: u8) -> bool {
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

fn is_valid_card(card: u8, first_card: u8, game: char) -> bool {
    println!("is_valid_card({:?}, {:?}, {:?})",
             card, first_card, game);
    let rv = match first_card {
        // Clubs
        0 ... 7 => match card {
             0 ... 7 => return true,
            _ => return false,
        },
        // Spades
        8 ... 15 => match card {
             8 ... 15 => return true,
            _ => return false,
        },
        // Hearts
        16 ... 23 => match card {
            16 ... 23 => return true,
            _ => return false,
        },
        // Diamonds
        24 ... 31 => match card {
             24 ... 31 => return true,
            _ => return false,
        },
        _ => false,
    };
    rv
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
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                8 ... 15 => { // Spades
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
                },
                16 ... 23 => { // Hearts
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
                },
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
        'n' => {
            for n in 0..3 {
                match cards[n] {
                    // Clubs
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                8 ... 15 => { // Spades
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
                },
                16 ... 23 => { // Hearts
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
                },
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
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
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                8 ... 15 => { // Spades
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
                16 ... 23 => { // Hearts
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
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
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
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                },
                8 ... 15 => { // Spades
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
                },
                16 ... 23 => { // Hearts
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
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
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
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                },
                8 ... 15 => { // Spades
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
                },
                16 ... 23 => { // Hearts
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
                },
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
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
                    0 ... 7 => clubs.push(cards[n]),
                    // Spades
                    8 ... 15 => spades.push(cards[n]),
                    // Hearts
                    16 ... 23 => hearts.push(cards[n]),
                    // Diamonds
                    24 ... 31 => diamonds.push(cards[n]),
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
                0 ... 7 => { // Clubs
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
                },
                8 ... 15 => { // Spades
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
                },
                16 ... 23 => { // Hearts
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
                },
                24 ... 31 => { // Diamonds
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
                },
                _ => panic!("Unknown card"),
            }
        },
        _   => panic!("Unknown game {}", game),
    }
    sorted
}

fn who_wins_trick(played_cards: &Vec<u8>,
                  game: char) -> u8 {
    let sorted = sort_trick_for(played_cards, game);
    println!("played_cards: {:?}", played_cards);
    println!("sorted: {:?}", sorted);
    if sorted[0] == played_cards[0] { return 0u8; }
    if sorted[0] == played_cards[1] { return 1u8; }
    if sorted[0] == played_cards[2] { return 2u8; }
    0u8
}

fn main() {
    // randomly select player
    let mut player_id: u8 = rand::thread_rng().gen_range(0, 3);
    loop {
        // player with player_id is dealing
        let (mut dealer,
             mut responder,
             mut bidder,
             skat) = deal(player_id);
        // bid (_game_value not used)
        let (declarer_id, _game_value, mut sorted_game) = bid(&mut dealer,
                                                              &mut responder,
                                                              &mut bidder);
        // announce game
        let mut hand = false;
        let mut overt = false;
        if dealer.id == declarer_id {
            dealer = dealer.announce_game(&mut sorted_game, &skat,
                                          // return values
                                          &mut hand, &mut overt);
        } else if responder.id == declarer_id {
            responder = responder.announce_game(&mut sorted_game, &skat,
                                                // return values
                                                &mut hand, &mut overt);
        } else if bidder.id == declarer_id {
            bidder = bidder.announce_game(&mut sorted_game, &skat,
                                          // return values
                                          &mut hand, &mut overt);
        }
        // all players sort for game
        dealer.sort_cards_for(sorted_game);
        responder.sort_cards_for(sorted_game);
        bidder.sort_cards_for(sorted_game);
        // TMP
        println!("sorted_game = {}", sorted_game);
        println!("dealer:");
        dealer.print_cards();
        println!("responder:");
        responder.print_cards();
        println!("bidder:");
        bidder.print_cards();
        // TMP
        let mut leader_id: u8 = responder.id;
        // play 10 tricks in a row
        for trick in 0..10 {
            println!("#########");
            println!("trick #{}:", trick);
            println!("#########");
            let mut played_cards: Vec<u8> = Vec::new();
            // use player to detect first card played
            for player in 0..3 {
                println!("player = {:?}", player);
                if dealer.id == leader_id {
                    dealer.print_cards();
                    dealer.play_card(&mut played_cards,
                                     player, sorted_game);
                } else if responder.id == leader_id {
                    responder.print_cards();
                    responder.play_card(&mut played_cards,
                                        player, sorted_game);
                } else if bidder.id == leader_id {
                    bidder.print_cards();
                    bidder.play_card(&mut played_cards,
                                     player, sorted_game);
                }
                // select next player
                leader_id = (leader_id + 1) % 3;
            }
            println!("played cards: ");
            print_card(played_cards[0]);
            print_card(played_cards[1]);
            print_card(played_cards[2]);
            println!("");
            // who wins this trick?
            let winner_id: u8 = (leader_id +
                                 who_wins_trick(&played_cards,
                                                sorted_game)) % 3;
            println!("winner_id = {}", winner_id);
            let winner_name = match winner_id {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => panic!("Unknown player {}", winner_id),
            };
            println!("Player {} wins trick {} ...",
                     winner_name, trick);
            if dealer.id == winner_id {
                dealer.add_trick(&played_cards);
            } else if responder.id == winner_id {
                responder.add_trick(&played_cards);
            } else if bidder.id == winner_id {
                bidder.add_trick(&played_cards);
            }
            // set leader_id
            leader_id = winner_id;
        };
        // count cards
        let mut declarer_count: u8 = 0u8;
        let mut team_count: u8 = 0u8;
        let mut tricks_len: usize = 0usize;
        println!("dealer: {}", dealer.count_cards());
        println!("responder: {}", responder.count_cards());
        println!("bidder: {}", bidder.count_cards());
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
        if sorted_game == 'n' {
            // check Null first
            if declarer_count == 0 || (hand && tricks_len == 2) {
                // TODO: overt
                println!("declarer wins with {} to {}",
                         declarer_count, team_count);
            } else {
                println!("declarer looses with {} to {}",
                         declarer_count, team_count);
            }
        } else {
            if declarer_count > team_count {
                println!("declarer wins with {} to {}",
                         declarer_count, team_count);
            } else {
                println!("declarer looses with {} to {}",
                         declarer_count, team_count);
            }
        }
        // continue?
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
