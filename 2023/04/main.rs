use crate::ParseError::InvalidFormat;
use std::fmt::format;
use std::str::FromStr;

fn main() {
    println!("Hello day 4");
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
}

struct Game {
    cards: Vec<Card>,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Game {
            cards: s
                .trim()
                .lines()
                .map(|line| match Card::from_str(line) {
                    Err(_) => panic!("could not parse card"),
                    Ok(card) => card,
                })
                .collect(),
        })
    }
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    user_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("Card ") {
            return Err(InvalidFormat);
        }

        let mut parts = s.split(":");
        let id = parts
            .next()
            .expect("should exist")
            .split(" ")
            .last()
            .expect("card id should exist")
            .parse()
            .expect("card id should be number");

        let mut parts = parts
            .next()
            .expect("card numbers part should exist")
            .split("|");
        let winning_numbers: Vec<u32> = parts
            .next()
            .expect("should exist")
            .trim()
            .split(" ")
            .filter(|num| num.trim().len() > 0)
            .map(|num| num.parse().expect("should be a number"))
            .collect();
        let user_numbers: Vec<u32> = parts
            .next()
            .expect("should exist")
            .trim()
            .split(" ")
            .filter(|num| num.trim().len() > 0)
            .map(|num| num.parse().expect("should be a number"))
            .collect();

        println!(
            "Card {id}: WN: {:?} | UN: {:?}",
            winning_numbers, user_numbers
        );

        Ok(Card {
            id,
            winning_numbers,
            user_numbers,
        })
    }
}

#[cfg(test)]
mod test_2023_04 {
    use crate::Game;
    use std::str::FromStr;

    const TEST_EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_something() {
        let game = Game::from_str(TEST_EXAMPLE).expect("should exist");
    }
}
