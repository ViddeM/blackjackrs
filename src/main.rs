use crate::types::{Hand, Shoe, TWENTY_ONE};
use clap::Parser;
use std::io::stdin;

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
}

fn main() {
    let args: BlackJack = BlackJack::parse();

    let shoe = Shoe::new(args.deck_count).expect("Failed to create shoe");
    let shoe = shoe.shuffle();

    play_shoe(shoe, args.reshuffle_limit);
}

fn play_shoe(shoe: Shoe, reshuffle_limit: u32) {
    let mut shoe = shoe;
    loop {
        println!("============ ROUND BEGIN ============");
        shoe = play_round(shoe);
        println!("============ ROUND END   ============ \n\n");

        if shoe.num_cards() < reshuffle_limit {
            println!("Shoe over");
            return;
        }
    }
}

fn play_round(shoe: Shoe) -> Shoe {
    let mut shoe = shoe.clone();

    let mut dealer_hand = Hand::from_card(shoe.take_card());
    let mut player_hand = Hand::from_card(shoe.take_card());
    let dealer_face_down = shoe.take_card();
    player_hand.add_card(shoe.take_card());

    println!("Dealer: {}", dealer_hand);

    let (player_hand, shoe) = player_turn(player_hand, shoe);

    dealer_hand.add_card(dealer_face_down);
    println!("Dealer hand: {}", dealer_hand);

    // Check winnings
    if player_hand.is_blackjack() {
        if dealer_hand.is_blackjack() {
            println!("Push");
        } else {
            println!("BlackJack wins 3:2");
        }
    }

    let (dealer_hand, shoe) = dealer_turn(dealer_hand, shoe);

    let player_value = player_hand.calc_value();
    let dealer_value = dealer_hand.calc_value();

    if player_value > TWENTY_ONE {
        println!("Bust :(");
        return shoe;
    }

    if dealer_value > TWENTY_ONE {
        println!("Dealer bust! winnings 1:1");
        return shoe;
    }

    if player_value == dealer_value {
        println!("Push!");
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
                    hand.add_card(shoe.take_card());
                    break;
                }
                "s" => return (hand, shoe),
                c => println!("Invalid choice '{}', please try again", c),
            }

            println!("Hand: {}", hand);
        }
    }
}

fn dealer_turn(hand: Hand, shoe: Shoe) -> (Hand, Shoe) {
    let mut hand = hand;
    let mut shoe = shoe;

    loop {
        if hand.calc_value() >= 17 {
            return (hand, shoe);
        }

        hand.add_card(shoe.take_card());
        println!("Dealer hit {}", hand);
    }
}
