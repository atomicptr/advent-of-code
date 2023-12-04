use crate::SchematicError::EmptyString;
use std::str::FromStr;

fn main() {
    let input_str = include_str!("./input.txt");
    let schematic = Schematic::from_str(input_str).expect("should parse");
    println!("Part 1) Result: {}", schematic.sum());
    println!("Part 2) Gear Part Sum: {}", schematic.gear_part_sum());
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Value {
    Empty,
    Symbol,
    Gear,
    Digit(usize),
}

impl Value {
    fn is_symbol(&self) -> bool {
        *self == Value::Symbol || *self == Value::Gear
    }
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

                if char == '*' {
                    row.push(Value::Gear);
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

#[derive(Copy, Clone)]
enum FindAdjacentValueSearchParam {
    IsSymbol,
    IsDigit,
}

impl Schematic {
    fn find_part_numbers(&self) -> Vec<usize> {
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
                    if !self.has_adjacent_symbol_range(row_index, column_start, column_index) {
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

    fn find_gear_ratios(&self) -> Vec<usize> {
        let mut numbers = vec![];

        for (row_index, row) in self.values.iter().enumerate() {
            for (column_index, value) in row.iter().enumerate() {
                if value.clone() != Value::Gear {
                    continue;
                }

                let digits = self.find_adjacent_symbols(
                    row_index,
                    column_index,
                    FindAdjacentValueSearchParam::IsDigit,
                );

                let mut gear_numbers = vec![];
                let mut prev_coords: Option<(usize, usize)> = None;

                for (digit_row, digit_col, _) in digits.iter() {
                    if prev_coords != None {
                        let prev_coords = prev_coords.unwrap();

                        // skip aligned coords
                        if prev_coords.0 == digit_row.clone()
                            && (prev_coords.1 < digit_col.clone()
                                || digit_col.clone() < prev_coords.1)
                        {
                            continue;
                        }
                    }

                    prev_coords = Some((digit_row.clone(), digit_col.clone()));
                    let num = self.find_number_from_digit(digit_row.clone(), digit_col.clone());
                    gear_numbers.push(num);
                }

                if gear_numbers.len() < 2 {
                    continue;
                }

                // check if digits belong to the same number and filter out
                // find number
                // multiply numbers
                numbers.push(gear_numbers.iter().product());
            }
        }

        numbers
    }

    fn find_number_from_digit(&self, row: usize, column: usize) -> usize {
        // find actual start
        let mut col_start = column;
        if column > 0 {
            for col in (0..=column).rev() {
                let value = self.get(row, col).unwrap();

                if let Value::Digit(_) = value {
                    col_start = col;
                    continue;
                }

                break;
            }
        }

        // find actual end
        let mut col_end = column;
        if column < self.width {
            for col in column..self.width {
                let value = self.get(row, col).unwrap();

                if let Value::Digit(_) = value {
                    col_end = col;
                    continue;
                }

                break;
            }
        }

        let mut digits = vec![];
        for col in col_start..=col_end {
            if let Value::Digit(digit) = self.get(row, col).unwrap() {
                digits.push(digit);
            }
        }

        create_number_from_digits(digits)
    }

    fn has_adjacent_symbol_range(
        &self,
        row: usize,
        column_start: usize,
        column_end: usize,
    ) -> bool {
        self.find_adjacent_symbols_range(
            row,
            column_start,
            column_end,
            FindAdjacentValueSearchParam::IsSymbol,
        )
        .len()
            > 0
    }

    fn find_adjacent_symbols_range(
        &self,
        row: usize,
        column_start: usize,
        column_end: usize,
        search_value: FindAdjacentValueSearchParam,
    ) -> Vec<(usize, usize, Value)> {
        let mut values = vec![];

        for col in column_start..=column_end {
            let found = self.find_adjacent_symbols(row, col, search_value.clone());
            if found.len() > 0 {
                values.extend(found);
            }
        }

        values
    }

    fn get(&self, row: usize, column: usize) -> Option<Value> {
        if row > self.width || column > self.height {
            return None;
        }

        let values_column = self.values.get(row);

        if values_column == None {
            return None;
        }

        let values_column = values_column.unwrap();

        if let Some(value) = values_column.get(column) {
            return Some(value.clone());
        }

        None
    }

    fn find_adjacent_symbols(
        &self,
        row: usize,
        column: usize,
        search_value: FindAdjacentValueSearchParam,
    ) -> Vec<(usize, usize, Value)> {
        let mut values = vec![];

        let start_row = if row == 0 { row } else { row - 1 };

        for row_index in start_row..=row + 1 {
            let start_col = if column == 0 { column } else { column - 1 };

            for col_index in start_col..=column + 1 {
                let value = self.get(row_index, col_index);

                if value == None {
                    continue;
                }

                let value = value.unwrap();

                match search_value {
                    FindAdjacentValueSearchParam::IsSymbol => {
                        if value.is_symbol() {
                            values.push((row_index, col_index, value));
                        }
                    }
                    FindAdjacentValueSearchParam::IsDigit => {
                        if let Value::Digit(_) = value {
                            values.push((row_index, col_index, value));
                        }
                    }
                };
            }
        }

        values
    }

    fn is_next_number(&self, row: usize, column: usize) -> bool {
        if column + 1 >= self.width {
            return false;
        }

        let next = self.values[row][column + 1];

        !next.is_symbol() && next != Value::Empty
    }

    fn sum(&self) -> usize {
        self.find_part_numbers().iter().sum()
    }

    fn gear_part_sum(&self) -> usize {
        self.find_gear_ratios().iter().sum()
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

    const TEST_SCHEMATIC_GEAR_TEST: &str = "\
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
    fn test_parsing_gear_part_example() {
        let schematic = Schematic::from_str(TEST_SCHEMATIC_GEAR_TEST).expect("should parse");
        assert_eq!(467835, schematic.gear_part_sum());
    }
}
