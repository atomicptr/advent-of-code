use crate::ParseError::InvalidFormat;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let input = include_str!("./input.txt");
    let game = Game::from_str(input).expect("should parse");
    println!("Part 1) Result: {}", game.total_ppints());
    println!("Part 2) Result: {}", game.scratch_cards_total());
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
}

struct Game {
    cards: HashMap<u32, Card>,
}

impl Game {
    fn total_ppints(&self) -> u32 {
        self.cards.values().map(|card| card.points()).sum()
    }

    fn scratch_cards(&self) -> HashMap<u32, u32> {
        let mut cards = HashMap::new();
        let mut scratch_cards: Vec<u32> = self.cards.values().map(|card| card.id).collect();

        while let Some(card_id) = scratch_cards.pop() {
            scratch_cards.extend(self.scratch_card_ids_for(card_id.clone()));
            let val = cards.entry(card_id).or_insert(0);
            *val += 1;
        }

        cards
    }

    fn scratch_cards_total(&self) -> u32 {
        self.scratch_cards().values().sum()
    }

    fn scratch_card_ids_for(&self, id: u32) -> Vec<u32> {
        if let Some(card) = self.cards.get(&id) {
            let count = card.user_winning_numbers().len();
            return (1..=count).map(|num| id + (num as u32)).collect();
        }

        vec![]
    }
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = HashMap::new();

        for line in s.trim().lines() {
            match Card::from_str(line) {
                Ok(card) => cards.insert(card.id, card),
                Err(err) => return Err(err),
            };
        }

        Ok(Game { cards })
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
    use std::collections::HashMap;
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

        let test_points = HashMap::from([(1, 8), (2, 2), (3, 2), (4, 1), (5, 0), (6, 0)]);
        let test_total_points: u32 = test_points.values().sum();

        for (id, card) in game.cards.iter() {
            assert_eq!(test_points.get(id).unwrap().clone(), card.points());
        }

        assert_eq!(test_total_points, game.total_ppints());
    }

    const TEST_EXAMPLE_PART_2: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse_example_part2() {
        let game = Game::from_str(TEST_EXAMPLE_PART_2).expect("should exist");

        let scratch_card_counts = game.scratch_cards();

        let expected_counts = HashMap::from([(1, 1), (2, 2), (3, 4), (4, 8), (5, 14), (6, 1)]);
        let expected_total = 30;

        for (id, num) in scratch_card_counts.iter() {
            let expected = expected_counts.get(id).unwrap();
            assert_eq!(expected, num, "ID: {id} expected: {expected} but got {num}");
        }

        assert_eq!(expected_total, game.scratch_cards_total());
    }
}
