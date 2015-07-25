extern crate ansi_term;
extern crate rand;

use ansi_term::Colour::*;
use rand::Rng;
use std::io;

enum Card {
    ClubsAce,      //  0
    ClubsTen,      //  1
    ClubsKing,     //  2
    ClubsQueen,    //  3
    ClubsJack,     //  4
    ClubsNine,     //  5
    ClubsEight,    //  6
    ClubsSeven,    //  7
    SpadesAce,     //  8
    SpadesTen,     //  9
    SpadesKing,    // 10
    SpadesQueen,   // 11
    SpadesJack,    // 12
    SpadesNine,    // 13
    SpadesEight,   // 14
    SpadesSeven,   // 15
    HeartsAce,     // 16
    HeartsTen,     // 17
    HeartsKing,    // 18
    HeartsQueen,   // 19
    HeartsJack,    // 20
    HeartsNine,    // 21
    HeartsEight,   // 22
    HeartsSeven,   // 23
    DiamondsAce,   // 24
    DiamondsTen,   // 25
    DiamondsKing,  // 26
    DiamondsQueen, // 27
    DiamondsJack,  // 28
    DiamondsNine,  // 29
    DiamondsEight, // 30
    DiamondsSeven, // 31
}

enum Game {
    Suit,
    Grand,
    Null,
}

enum PlayerId {
    A,
    B,
    C,
}

struct Player {
    id: u8,
    cards: Vec<u8>,
}

impl Player {
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
        clubs.sort();
        //clubs.reverse();
        spades.sort();
        //spades.reverse();
        hearts.sort();
        //hearts.reverse();
        diamonds.sort();
        //diamonds.reverse();
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
            max = diamonds.len() as u8;
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
}

struct PlayerBuilder {
    id: u8,
    cards: Vec<u8>,
}

impl PlayerBuilder {
    fn new() -> PlayerBuilder {
        PlayerBuilder { cards: Vec::new(), id: 3, }
    }

    fn add(&mut self, newCard: u8) -> &mut PlayerBuilder {
        self.cards.push(newCard);
        self
    }

    fn id(&mut self, newId: u8) -> &mut PlayerBuilder {
        self.id = newId;
        self
    }

    fn finalize(&self) -> Player {
        Player { id: self.id, cards: self.cards.to_vec(), }
    }
}

fn announce(player: u8) -> Game {
    Game::Suit
}

fn bid(dealer: &Player,
       responder: &Player,
       bidder: &Player) -> (u8, u8) {
    let winner: u8 = dealer.id;
    let highest: u8 = 18;
    // bidder sees his cards first
    bidder.print_cards();
    println!("");
    // ask for input
    let mut bidder_bid: u8 = 0u8;
    loop {
        println!("bid:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        bidder_bid = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }
    println!("bidder: {}", bidder_bid);
    // WORK
    (winner, highest)
}

fn deal(dealerId: u8) -> (Player, Player, Player) {
    let mut cards: Vec<u8> = (0..32).collect();
    // shuffle cards
    let mut upper: u8 = 32;
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
        .id(dealerId)
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
        .id((dealerId + 1) % 3)
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
        .id((dealerId + 2) % 3)
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
    println!("Skat:");
    for n in 30..32 {
        print_card(shuffled[n]);
    };
    println!("");
    (dealer, left, right)
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
    let player_id: u8 = rand::thread_rng().gen_range(0, 3);
    loop {
        // player with player_id is dealing
        let (dealer,
             responder,
             bidder) = deal(player_id);
        // bid
        let (declarer_id, game_value) = bid(&dealer,
                                            &responder,
                                            &bidder);
        // announce
        let game = announce(declarer_id);
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
