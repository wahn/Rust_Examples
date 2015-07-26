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
    fn sort_cards_for(&// mut
                      self, game: char) {
        match game {
            'g' => println!("Grand"),
            'n' => println!("Null"),
            'c' => println!("Clubs"),
            's' => println!("Spades"),
            'h' => println!("Hearts"),
            'd' => println!("Diamonds"),
            _   => panic!("Unknown game {}", game),
        }
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

    fn id(&mut self, new_id: u8) -> &mut PlayerBuilder {
        self.id = new_id;
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

// fn announce(player: u8) -> Game {
//     Game::Suit
// }

fn bid(dealer: &Player,
       responder: &Player,
       bidder: &Player) -> (u8, u8) {
    let mut winner: u8;
    // bidder sees his cards first
    bidder.print_cards();
    println!("");
    // ask for input
    let mut bidder_bid: u8;
    loop {
        println!("bid:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
        if input == "g\n".to_string() {
            println!("sort for Grand ...");
            let g: char = 'g';
            bidder.sort_cards_for(g);
            continue;
        } else if input == "n\n".to_string() {
            println!("sort for Null ...");
            let g: char = 'n';
            bidder.sort_cards_for(g);
            continue;
        } else if input == "c\n".to_string() {
            println!("sort for Clubs ...");
            let g: char = 'c';
            bidder.sort_cards_for(g);
            continue;
        } else if input == "s\n".to_string() {
            println!("sort for Spades ...");
            let g: char = 's';
            bidder.sort_cards_for(g);
            continue;
        } else if input == "h\n".to_string() {
            println!("sort for Hearts ...");
            let g: char = 'h';
            bidder.sort_cards_for(g);
            continue;
        } else if input == "d\n".to_string() {
            println!("sort for Diamonds ...");
            let g: char = 'd';
            bidder.sort_cards_for(g);
            continue;
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
    println!("bidder: {}", bidder_bid);
    // responder is next
    responder.print_cards();
    println!("");
    // ask for input
    let mut responder_bid: u8;
    loop {
        println!("bid:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
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
    println!("responder: {}", responder_bid);
    // ask dealer last
    dealer.print_cards();
    println!("");
    // ask for input
    let mut dealer_bid: u8;
    loop {
        println!("bid:");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .ok()
            .expect("failed to read line");
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
    println!("dealer: {}", dealer_bid);
    // who wins bidding?
    let mut lowest: u8;
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
                return (dealer.id, 0u8);
            } else {
                // dealer wants to play and all others passed
                lowest = 18u8; // dealer said at least 18
                winner = dealer.id;
                let winner_name = match winner {
                    0 => "A",
                    1 => "B",
                    2 => "C",
                    _ => panic!("Unknown player {}", winner),
                };
                println!("Player {} wins bidding with {} ...",
                         winner_name, lowest);
                return (winner, lowest);
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
        } else {
            if dealer_bid > lowest {
                lowest = dealer_bid;
            }
            winner = responder.id;
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
        } else {
            if dealer_bid > lowest {
                lowest = dealer_bid;
            }
            winner = bidder.id;
        }
    }
    let winner_name = match winner {
        0 => "A",
        1 => "B",
        2 => "C",
        _ => panic!("Unknown player {}", winner),
    };
    println!("Player {} wins bidding with {} ...", winner_name, lowest);
    (winner, lowest)
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
        let (dealer,
             responder,
             bidder,
             skat) = deal(player_id);
        // bid
        let (declarer_id, game_value) = bid(&dealer,
                                            &responder,
                                            &bidder);
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
        if input == 1 {
            skat.print_cards();
        }
        // announce
        // WORK: let game = announce(declarer_id);
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
