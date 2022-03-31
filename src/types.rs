use rand::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    InvalidCardVal(u32),
}

pub const TWENTY_ONE: u32 = 21;
pub const DECK_SIZE: usize = 52;

#[derive(Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn calc_value(&self) -> u32 {
        let aces = self
            .cards
            .iter()
            .filter(|c| c.value == Value::Ace)
            .collect::<Vec<&Card>>()
            .len() as u32;

        let val_without_aces = self
            .cards
            .iter()
            .filter(|c| c.value != Value::Ace)
            .map(|c| c.value.value())
            .sum();

        let mut val = val_without_aces;
        for n in 0..=aces {
            let num_aces = aces - n;
            val = val_without_aces + (num_aces * Value::ACE_HIGH_VAL) + (n * Value::ACE_LOW_VAL);

            if val <= TWENTY_ONE {
                return val;
            }
        }
        return val;
    }

    pub fn from_card(card: Card) -> Self {
        Self { cards: vec![card] }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn is_blackjack(&self) -> bool {
        let num_aces = self.cards.iter().filter(|c| c.value == Value::Ace).count();
        num_aces == 1 && self.cards.len() == 2 && self.calc_value() == TWENTY_ONE
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.calc_value(),
        )
    }
}

#[derive(Clone)]
pub struct Shoe {
    pub cards: Vec<Card>,
    pub running_count: i32,
    pub true_count: f32,
}

impl Shoe {
    pub fn new(num_decks: u32) -> Result<Self, Error> {
        let deck_cards = (0..num_decks)
            .into_iter()
            .map(|d| match Deck::new() {
                Ok(d) => Ok(d.cards),
                Err(e) => Err(e),
            })
            .collect::<Result<Vec<Vec<Card>>, Error>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<Card>>();

        Ok(Shoe {
            cards: deck_cards,
            running_count: 0,
            true_count: 0f32,
        })
    }

    pub fn take_card(&mut self) -> Card {
        let card = self.cards.pop().expect("Out of cards!");
        let card_val = card.value.value();
        let count_change = if card_val <= 6 {
            1
        } else if card_val >= 10 {
            -1
        } else {
            0
        };

        self.running_count += count_change;
        let remaining_decks = (self.cards.len() / DECK_SIZE) as f32;
        self.true_count = self.running_count as f32 / remaining_decks;

        card
    }

    pub fn num_cards(&self) -> u32 {
        self.cards.len() as u32
    }

    pub fn shuffle(self) -> Self {
        let mut rng = thread_rng();
        let mut new_cards = self.cards.clone();
        new_cards.shuffle(&mut rng);

        Self {
            cards: new_cards,
            running_count: 0,
            true_count: 0f32,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Result<Self, Error> {
        let all_values = (2u32..=14)
            .into_iter()
            .map(|v| Value::try_from(v))
            .collect::<Result<Vec<Value>, Error>>()?;

        let all_suites = vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs];
        let cards = all_suites
            .into_iter()
            .map(|suit| {
                all_values
                    .clone()
                    .into_iter()
                    .map(|value| Card { suit, value })
                    .collect::<Vec<Card>>()
            })
            .flatten()
            .collect::<Vec<Card>>();

        Ok(Deck { cards })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Suit::Spades => "♠",
                Suit::Hearts => "♥",
                Suit::Diamonds => "♦",
                Suit::Clubs => "♣",
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    pub const ACE_HIGH_VAL: u32 = 11;
    pub const ACE_LOW_VAL: u32 = 1;
}

impl TryFrom<u32> for Value {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Error> {
        Ok(match value {
            1 => Self::Ace,
            2 => Self::Two,
            3 => Self::Three,
            4 => Self::Four,
            5 => Self::Five,
            6 => Self::Six,
            7 => Self::Seven,
            8 => Self::Eight,
            9 => Self::Nine,
            10 => Self::Ten,
            11 => Self::Jack,
            12 => Self::Queen,
            13 => Self::King,
            14 => Self::Ace,
            _ => return Err(Error::InvalidCardVal(value)),
        })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Two => "2",
                Value::Three => "3",
                Value::Four => "4",
                Value::Five => "5",
                Value::Six => "6",
                Value::Seven => "7",
                Value::Eight => "8",
                Value::Nine => "9",
                Value::Ten => "10",
                Value::Jack => "J",
                Value::Queen => "Q",
                Value::King => "K",
                Value::Ace => "A",
            }
        )
    }
}

impl Value {
    pub fn value(&self) -> u32 {
        match self {
            Value::Two => 2,
            Value::Three => 3,
            Value::Four => 4,
            Value::Five => 5,
            Value::Six => 6,
            Value::Seven => 7,
            Value::Eight => 8,
            Value::Nine => 9,
            Value::Ten => 10,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Card, Suit, Value};
    use crate::Hand;

    #[test]
    fn test_hand_add_card() {
        let mut hand = Hand::default();
        let card = Card {
            suit: Suit::Spades,
            value: Value::Two,
        };
        hand.add_card(card.clone());
        assert_eq!(hand.cards.len(), 1);
        assert_eq!(hand.cards[0], card);
    }

    #[test]
    fn test_hand_calc_val() {
        let two_spades = Card {
            suit: Suit::Spades,
            value: Value::Two,
        };
        let king_hearts = Card {
            suit: Suit::Hearts,
            value: Value::King,
        };
        let four_diamonds = Card {
            suit: Suit::Diamonds,
            value: Value::Four,
        };
        let five_clubs = Card {
            suit: Suit::Clubs,
            value: Value::Five,
        };

        let mut hand = Hand::from_card(two_spades);
        hand.add_card(king_hearts);
        hand.add_card(four_diamonds);
        hand.add_card(five_clubs);

        assert_eq!(hand.calc_value(), 21);
    }

    #[test]
    fn test_hand_calc_val_high_ace() {
        let ace = Card {
            suit: Suit::Spades,
            value: Value::Ace,
        };
        let king = Card {
            suit: Suit::Diamonds,
            value: Value::King,
        };

        let mut hand = Hand::from_card(ace);
        hand.add_card(king);

        assert_eq!(hand.calc_value(), 21);
    }

    #[test]
    fn test_hand_calc_val_low_ace() {
        let ace = Card {
            suit: Suit::Spades,
            value: Value::Ace,
        };
        let king = Card {
            suit: Suit::Diamonds,
            value: Value::King,
        };
        let jack = Card {
            suit: Suit::Clubs,
            value: Value::Jack,
        };

        let mut hand = Hand::from_card(ace);
        hand.add_card(king);
        hand.add_card(jack);

        assert_eq!(hand.calc_value(), 21);
    }
}
