fn main() {
    let file = include_str!("./input.txt");

    let limits = (12, 13, 14);

    let mut sum_ids = 0;
    let mut num_required_cubes = 0;

    for line in file.lines() {
        if let Some(game) = Game::from_line(line) {
            if game.fits(limits.0, limits.1, limits.2) {
                sum_ids += game.number;
            }

            let (r, g, b) = game.max_cubes();
            num_required_cubes += r * g * b;
        }
    }

    println!("Sum of IDs: {sum_ids}");
    println!("Num required cubes: {num_required_cubes}");
}

pub struct Game {
    number: usize,
    sets: Vec<Set>,
}

pub struct Set(usize, usize, usize);

impl Game {
    fn from_line(line: &str) -> Option<Game> {
        if !line.starts_with("Game") {
            return None;
        }

        let mut parts = line.split(":");
        let game_part = parts.next().expect("should have game part");
        let sets_part = parts.next().expect("should have sets part");

        let game_number: usize = game_part
            .split(" ")
            .last()
            .expect("should have game number")
            .parse()
            .expect("should be digit");

        let mut game = Self {
            number: game_number,
            sets: vec![],
        };

        for set_part in sets_part.split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for item_pairs in set_part.trim().split(",") {
                let mut item_pair = item_pairs.trim().split(" ");
                let number: usize = item_pair
                    .next()
                    .expect("should have cube number")
                    .parse()
                    .expect("cube number should be a  number");
                let color = item_pair.next().expect("should have cube color");

                match color {
                    "red" => red += number,
                    "green" => green += number,
                    "blue" => blue += number,
                    unknown => panic!("Unknown color name: {}", unknown),
                };
            }

            game.sets.push(Set(red, green, blue));
        }

        Some(game)
    }

    fn max_cubes(&self) -> (usize, usize, usize) {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for set in self.sets.iter() {
            if set.0 > max_red {
                max_red = set.0;
            }

            if set.1 > max_green {
                max_green = set.1;
            }

            if set.2 > max_blue {
                max_blue = set.2;
            }
        }

        (max_red, max_green, max_blue)
    }

    fn fits(&self, red: usize, green: usize, blue: usize) -> bool {
        let (r, g, b) = self.max_cubes();
        r <= red && g <= green && b <= blue
    }
}

#[cfg(test)]
mod test_2023_02 {
    use crate::Game;

    const TEST_LINES: [(&str, (usize, usize, usize)); 5] = [
        (
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            (4, 2, 6),
        ),
        (
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            (1, 3, 4),
        ),
        (
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            (20, 13, 6),
        ),
        (
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            (14, 3, 15),
        ),
        (
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            (6, 3, 2),
        ),
    ];

    #[test]
    fn test_parsing_lines() {
        for (line, (expected_red, expected_green, expected_blue)) in TEST_LINES.iter() {
            let game = Game::from_line(line).expect("should parse");

            let (r, g, b) = game.max_cubes();

            assert_eq!(
                *expected_red, r,
                "max red cubes for line: {} :: should be {} but is {}",
                line, expected_red, r
            );
            assert_eq!(
                *expected_green, g,
                "max green cubes for line: {} :: should be {} but is {}",
                line, expected_green, g
            );
            assert_eq!(
                *expected_blue, b,
                "max blue cubes for line: {} :: should be {} but is {}",
                line, expected_blue, b
            );
        }
    }

    const TEST_LINES_FIT: [(&str, (usize, usize, usize), bool); 5] = [
        (TEST_LINES[0].0, (12, 13, 14), true),
        (TEST_LINES[1].0, (12, 13, 14), true),
        (TEST_LINES[2].0, (12, 13, 14), false),
        (TEST_LINES[3].0, (12, 13, 14), false),
        (TEST_LINES[4].0, (12, 13, 14), true),
    ];
    const TEST_LINES_POSSIBLE_ID_SUM: usize = 8;

    #[test]
    fn test_parsed_lines_fit() {
        let mut sum = 0;

        for (line, (red_cubes, green_cubes, blue_cubes), expect_fits) in TEST_LINES_FIT.iter() {
            let game = Game::from_line(line).expect("should parse");

            let (r, g, b) = game.max_cubes();
            let fits = game.fits(*red_cubes, *green_cubes, *blue_cubes);

            assert_eq!(
                *expect_fits, fits,
                "Game {} ({}, {}, {}) fits within ({}, {}, {})? Result: {} but expected: {}",
                game.number, r, g, b, red_cubes, green_cubes, blue_cubes, fits, expect_fits
            );

            if fits {
                sum += game.number;
            }
        }

        assert_eq!(TEST_LINES_POSSIBLE_ID_SUM, sum);
    }
}
