use crate::types::{Hand, Player, Shoe, TWENTY_ONE};
use clap::Parser;
use std::io::stdin;
use std::num::ParseIntError;
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

    /// The initial amount of money for the player in dollars ($).
    #[clap(short, long, default_value_t = 2500f64)]
    buy_in_amount: f64,
}

struct GameConfig {
    sleep_duration: Duration,
    reshuffle_limit: u32,
}

fn main() {
    let args: BlackJack = BlackJack::parse();

    let mut shoe = Shoe::new(args.deck_count).expect("Failed to create shoe");
    shoe.shuffle();

    let mut player = Player::new(args.buy_in_amount);
    play_shoe(
        &mut shoe,
        &mut player,
        &GameConfig {
            reshuffle_limit: args.reshuffle_limit,
            sleep_duration: Duration::from_millis(args.delay as u64),
        },
    );
}

fn play_shoe(shoe: &mut Shoe, player: &mut Player, conf: &GameConfig) {
    loop {
        println!("============ ROUND BEGIN ============");
        place_bets(player);

        thread::sleep(conf.sleep_duration);

        play_round(shoe, player, conf);
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

fn play_round(shoe: &mut Shoe, player: &mut Player, conf: &GameConfig) {
    let mut dealer_hand = Hand::from_card(shoe.take_card());
    player.new_hand();
    player.hand.add_card(shoe.take_card());

    let dealer_face_down = shoe.take_card();
    player.hand.add_card(shoe.take_card());

    println!("Dealer: {}", dealer_hand);
    thread::sleep(conf.sleep_duration);

    player_turn(player, shoe);

    dealer_hand.add_card(dealer_face_down);
    println!("Dealer hand: {}", dealer_hand);

    // Check winnings
    if player.hand.is_blackjack() {
        if dealer_hand.is_blackjack() {
            println!("Push! You get your money back");
        } else {
            println!("BlackJack wins 3:2");
        }
        return;
    }

    dealer_turn(&mut dealer_hand, shoe, conf);

    let player_value = player.hand.calc_value();
    let dealer_value = dealer_hand.calc_value();

    thread::sleep(conf.sleep_duration);

    if player_value > TWENTY_ONE {
        println!("Player bust :(");
        return;
    }

    if dealer_value > TWENTY_ONE {
        println!("Dealer bust! winnings 1:1");
        return;
    }

    if player_value == dealer_value {
        println!("Push! You get your money back");
        return;
    }

    if dealer_value > player_value {
        println!("Dealer wins, better luck next time!");
        return;
    } else {
        println!("Congratulations! winnings 1:1");
        return;
    }
}

fn player_turn(player: &mut Player, shoe: &mut Shoe) {
    loop {
        println!("Hand: {}", player.hand);

        if player.hand.calc_value() >= TWENTY_ONE {
            if player.hand.is_blackjack() {
                println!("BlackJack!");
            }

            // Player is bust or at exactly 21!
            return;
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
                    player.hand.add_card(card);
                    break;
                }
                "s" => return,
                c => println!("Invalid choice '{}', please try again", c),
            }

            println!("Hand: {}", player.hand);
        }
    }
}

fn dealer_turn(hand: &mut Hand, shoe: &mut Shoe, conf: &GameConfig) {
    loop {
        if hand.calc_value() >= 17 {
            return;
        }

        thread::sleep(conf.sleep_duration);

        hand.add_card(shoe.take_card());
        println!("Dealer hit {}", hand);
    }
}

fn place_bets(player: &mut Player) {
    loop {
        let bet_amount = get_bet_amount(player);
        if player.place_bet(bet_amount) {
            return;
        } else {
            println!(
                "You cannot afford a bet of {}$, please try again with a smaller bet",
                bet_amount
            );
        }
    }
}

fn get_bet_amount(player: &mut Player) -> u64 {
    loop {
        println!("Place your bets for the round!");
        println!("Repeat last [r] / Amount [a]?");
        let mut choice = String::new();
        stdin()
            .read_line(&mut choice)
            .ok()
            .expect("Failed to read bet choice");

        match choice.as_str() {
            "r" => {
                if player.current_bet == 0 {
                    println!("You have not yet placed any bets, please place one before trying to repeat");
                } else {
                    return player.current_bet;
                }
            }
            "a" => {
                println!("How much would you like to bet of your {}$?", player.money);
                let mut amount = String::new();
                stdin()
                    .read_line(&mut amount)
                    .ok()
                    .expect("Failed to read amount");

                match amount.parse::<u64>() {
                    Ok(parsed_amount) => return parsed_amount,
                    Err(_) => println!("Invalid number {}, please try again", amount),
                }
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}
