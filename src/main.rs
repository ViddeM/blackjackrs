use crate::types::{Hand, Shoe, TWENTY_ONE};
use clap::Parser;
use std::io::stdin;
use std::thread;
use std::time::Duration;

mod types;

/// BlackJack card game
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct BlackJack {
    /// The number of decks in the shoe
    #[clap(short, long, default_value_t = 6)]
    deck_count: u32,

    /// The minimum number of cards required to play another round
    #[clap(short, long, default_value_t = 52)]
    reshuffle_limit: u32,

    /// Delay (in milliseconds) between moves to make it easier to follow the game, 0 means no delay.
    #[clap(long, default_value_t = 1000)]
    delay: u32,
}

struct GameConfig {
    sleep_duration: Duration,
    reshuffle_limit: u32,
}

fn main() {
    let args: BlackJack = BlackJack::parse();

    let shoe = Shoe::new(args.deck_count).expect("Failed to create shoe");
    let shoe = shoe.shuffle();

    play_shoe(
        shoe,
        &GameConfig {
            reshuffle_limit: args.reshuffle_limit,
            sleep_duration: Duration::from_millis(args.delay as u64),
        },
    );
}

fn play_shoe(shoe: Shoe, conf: &GameConfig) {
    let mut shoe = shoe;
    loop {
        println!("============ ROUND BEGIN ============");
        shoe = play_round(shoe, conf);
        thread::sleep(conf.sleep_duration);
        println!("============ ROUND END   ============ \n");

        println!(
            "Counts (running/true) {}/{:.1}\n",
            shoe.running_count, shoe.true_count
        );

        if shoe.num_cards() < conf.reshuffle_limit {
            println!("Shoe over");
            return;
        }
    }
}

fn play_round(shoe: Shoe, conf: &GameConfig) -> Shoe {
    let mut shoe = shoe.clone();

    let mut dealer_hand = Hand::from_card(shoe.take_card());
    let mut player_hand = Hand::from_card(shoe.take_card());
    let dealer_face_down = shoe.take_card();
    player_hand.add_card(shoe.take_card());

    println!("Dealer: {}", dealer_hand);
    thread::sleep(conf.sleep_duration);

    let (player_hand, shoe) = player_turn(player_hand, shoe);

    dealer_hand.add_card(dealer_face_down);
    println!("Dealer hand: {}", dealer_hand);

    // Check winnings
    if player_hand.is_blackjack() {
        if dealer_hand.is_blackjack() {
            println!("Push! You get your money back");
        } else {
            println!("BlackJack wins 3:2");
        }
        return shoe;
    }

    let (dealer_hand, shoe) = dealer_turn(dealer_hand, shoe, conf);

    let player_value = player_hand.calc_value();
    let dealer_value = dealer_hand.calc_value();

    thread::sleep(conf.sleep_duration);

    if player_value > TWENTY_ONE {
        println!("Player bust :(");
        return shoe;
    }

    if dealer_value > TWENTY_ONE {
        println!("Dealer bust! winnings 1:1");
        return shoe;
    }

    if player_value == dealer_value {
        println!("Push! You get your money back");
        return shoe;
    }

    if dealer_value > player_value {
        println!("Dealer wins, better luck next time!");
        return shoe;
    } else {
        println!("Congratulations! winnings 1:1");
        return shoe;
    }
}

fn player_turn(hand: Hand, shoe: Shoe) -> (Hand, Shoe) {
    let mut hand = hand;
    let mut shoe = shoe;

    loop {
        println!("Hand: {}", hand);

        if hand.calc_value() >= TWENTY_ONE {
            if hand.is_blackjack() {
                println!("BlackJack!");
            }

            // Player is bust or at exactly 21!
            return (hand, shoe);
        }

        let mut choice = String::new();
        loop {
            choice = String::new();
            println!("Move? [h/s]");
            stdin()
                .read_line(&mut choice)
                .ok()
                .expect("Failed to read choice");

            choice = String::from(
                choice
                    .strip_suffix('\n')
                    .expect("Choice did not end with newline?"),
            );

            match choice.as_str() {
                "h" => {
                    let card = shoe.take_card();
                    hand.add_card(card);
                    break;
                }
                "s" => return (hand, shoe),
                c => println!("Invalid choice '{}', please try again", c),
            }

            println!("Hand: {}", hand);
        }
    }
}

fn dealer_turn(hand: Hand, shoe: Shoe, conf: &GameConfig) -> (Hand, Shoe) {
    let mut hand = hand;
    let mut shoe = shoe;

    loop {
        if hand.calc_value() >= 17 {
            return (hand, shoe);
        }

        thread::sleep(conf.sleep_duration);

        hand.add_card(shoe.take_card());
        println!("Dealer hit {}", hand);
    }
}
