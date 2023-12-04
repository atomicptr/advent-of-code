use crate::ParseError::InvalidFormat;
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let game = Game::from_str(input).expect("should parse");
    println!("Part 1) Result: {}", game.total_ppints());
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
}

struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn total_ppints(&self) -> u32 {
        self.cards.iter().map(|card| card.points()).sum()
    }
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

impl Card {
    fn user_winning_numbers(&self) -> Vec<u32> {
        self.user_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .map(|num| num.clone())
            .collect()
    }

    fn calculate_points_for_matches(&self, number_matches: u32) -> u32 {
        match number_matches {
            0 => 0,
            1 => 1,
            num => self.calculate_points_for_matches(num - 1) * 2,
        }
    }

    fn points(&self) -> u32 {
        let matches = self.user_winning_numbers();
        self.calculate_points_for_matches(matches.len() as u32)
    }
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
    fn test_parse_example() {
        let game = Game::from_str(TEST_EXAMPLE).expect("should exist");

        let test_points = [8, 2, 2, 1, 0, 0];
        let test_total_points: u32 = test_points.iter().sum();

        for (index, card) in game.cards.iter().enumerate() {
            assert_eq!(test_points.get(index).unwrap().clone(), card.points());
        }

        assert_eq!(test_total_points, game.total_ppints());
    }
}
