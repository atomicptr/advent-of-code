fn main() {
    let input = include_str!("input.txt");
    println!("Part 1 result is: {}", calibrate(input));
    println!("Part 2 result is: {}", calibrate_with_words(input));
}

// part 1
fn calibrate(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        // iterate from front until the first numeric char appears add that to res multiplied
        // by 10 because assuming as2df1g this should be 21 so the first number is basically
        // 20 and the last one 1 making this 21
        for char in line.chars() {
            if char.is_numeric() {
                let val: u32 = char.to_digit(10).unwrap().into();
                result += val * 10;
                break;
            }
        }

        // for the second number we do exactly the same just backwards
        for char in line.chars().rev() {
            if char.is_numeric() {
                let val: u32 = char.to_digit(10).unwrap().into();
                result += val;
                break;
            }
        }
    }

    result
}

// part 2
fn word_to_value(input: &str) -> Option<u32> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn find_word_value(input: &str) -> Option<u32> {
    let num = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for n in num.iter() {
        if !input.contains(n) {
            continue;
        }

        if let Some(val) = word_to_value(n) {
            return Some(val);
        }
    }

    None
}

fn calibrate_with_words(input: &str) -> u32 {
    let mut result = 0;

    for line in input.lines() {
        let mut str = String::new();

        // same as above from the front
        for char in line.chars() {
            // if str contains a valid value for word_to_value use that
            if let Some(val) = word_to_value(str.as_str()) {
                result += val * 10;
                break;
            }

            // same as above if its a number use that
            if char.is_numeric() {
                let val: u32 = char.to_digit(10).unwrap().into();
                result += val * 10;
                break;
            }

            // if not, add current char to str
            str.push(char);

            if let Some(val) = find_word_value(str.as_str()) {
                result += val * 10;
                break;
            }
        }

        let mut str = String::new();

        // same as above from the back
        for char in line.chars().rev() {
            // if str contains a valid value for word_to_value use that
            if let Some(val) = word_to_value(str.as_str()) {
                result += val;
                break;
            }

            // same as above if its a number use that
            if char.is_numeric() {
                let val: u32 = char.to_digit(10).unwrap().into();
                result += val;
                break;
            }

            // if not, add current char to str at the font
            str.insert(0, char);

            if let Some(val) = find_word_value(str.as_str()) {
                result += val;
                break;
            }
        }
    }

    result
}

#[cfg(test)]
mod test_2023_01 {
    use crate::{calibrate, calibrate_with_words};

    const TEST_INPUT_CALIBRATE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_calibrate() {
        assert_eq!(calibrate(TEST_INPUT_CALIBRATE), 142);
    }

    const TEST_INPUT_CALIBRATE_WITH_WORDS: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_calibrate_with_words() {
        assert_eq!(calibrate_with_words(TEST_INPUT_CALIBRATE_WITH_WORDS), 281);
    }
}
