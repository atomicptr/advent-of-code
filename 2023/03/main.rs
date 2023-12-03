use crate::SchematicError::EmptyString;
use std::str::FromStr;

fn main() {
    let input_str = include_str!("./input.txt");
    let schematic = Schematic::from_str(input_str).expect("should parse");
    println!("Result: {}", schematic.sum());
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Value {
    Empty,
    Symbol,
    Digit(usize),
}

struct Schematic {
    width: usize,
    height: usize,
    values: Vec<Vec<Value>>,
}

#[derive(Debug)]
enum SchematicError {
    EmptyString,
}

impl FromStr for Schematic {
    type Err = SchematicError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 0 {
            return Err(EmptyString);
        }

        let height = s.lines().filter(|str| str.trim().len() > 0).count();
        let width = s
            .lines()
            .last()
            .expect("should have at least one line")
            .trim()
            .len();

        let mut schematic = Schematic {
            width,
            height,
            values: vec![],
        };

        for line in s.lines() {
            let mut row = vec![];

            for char in line.trim().chars() {
                if char == '.' {
                    row.push(Value::Empty);
                    continue;
                }

                if char.is_numeric() {
                    let val = char.to_digit(10).unwrap() as usize;
                    row.push(Value::Digit(val));
                    continue;
                }

                row.push(Value::Symbol);
            }

            schematic.values.push(row);
        }

        Ok(schematic)
    }
}

impl Schematic {
    fn find_numbers(&self) -> Vec<usize> {
        let mut numbers = vec![];

        for (row_index, row) in self.values.iter().enumerate() {
            let mut digits = vec![];

            for (column_index, value) in row.iter().enumerate() {
                if let Value::Digit(digit) = value {
                    let column_start = column_index - digits.len();

                    digits.push(digit.clone());

                    if self.is_next_number(row_index, column_index) {
                        continue;
                    }

                    // this number has no adjacent symbols so we can ignore it
                    if !self.has_adjacent_symbol(row_index, column_start, column_index) {
                        digits.clear();
                        continue;
                    }

                    numbers.push(create_number_from_digits(digits.clone()));
                    digits.clear();
                    continue;
                }
            }
        }

        numbers
    }

    fn has_adjacent_symbol(&self, row: usize, column_start: usize, column_end: usize) -> bool {
        for col in column_start..=column_end {
            let start_row = if row == 0 { row } else { row - 1 };

            for row_index in start_row..=(row + 1) {
                let start_col = if col == 0 { col } else { col - 1 };

                for column_index in start_col..=(col + 1) {
                    let row = self.values.get(row_index);

                    if row == None {
                        continue;
                    }

                    let row = row.unwrap();
                    let value = row.get(column_index);

                    if value == None {
                        continue;
                    }

                    let value = value.unwrap();

                    if *value == Value::Symbol {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_next_number(&self, row: usize, column: usize) -> bool {
        if column + 1 >= self.width {
            return false;
        }

        let next = self.values[row][column + 1];

        next != Value::Symbol && next != Value::Empty
    }

    fn sum(&self) -> usize {
        self.find_numbers().iter().sum()
    }
}

fn create_number_from_digits(digits: Vec<usize>) -> usize {
    let mut value = 0;

    for (index, digit) in digits.iter().rev().enumerate() {
        value += 10_usize.pow(index as u32) * digit;
    }

    value
}

#[cfg(test)]
mod test_2023_03 {
    use crate::{Schematic, Value};
    use std::str::FromStr;

    const TEST_SIMPLIFIED_SCHEMATIC: &str = "\
12..
$..1
8...";

    #[test]
    fn test_parsing_simple_schematic() {
        let schematic = Schematic::from_str(TEST_SIMPLIFIED_SCHEMATIC).expect("should parse");

        assert_eq!(4, schematic.width);
        assert_eq!(3, schematic.height);

        let expected_values = [
            // 12..
            [Value::Digit(1), Value::Digit(2), Value::Empty, Value::Empty],
            // $...
            [Value::Symbol, Value::Empty, Value::Empty, Value::Digit(1)],
            // 8...
            [Value::Digit(8), Value::Empty, Value::Empty, Value::Empty],
        ];

        for (row_index, row) in schematic.values.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                assert_eq!(expected_values[row_index][column_index], *value);
            }
        }

        assert_eq!(20, schematic.sum());
    }

    const TEST_SCHEMATICS: &str = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parsing_example() {
        let schematic = Schematic::from_str(TEST_SCHEMATICS).expect("should parse");
        assert_eq!(4361, schematic.sum());
    }
}
