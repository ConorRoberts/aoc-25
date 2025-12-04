use std::fs;

use aoc_25::*;

const DIGITS: usize = 12;
const ZERO_CHAR_CODE: usize = 48;

fn main() {
    let file_name = get_input_file();

    let total = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .fold(0, |acc, line| acc + largest_joltage(line, DIGITS));

    println!("Total {total}");
}

fn char_value(c: char) -> usize {
    (c as usize) - ZERO_CHAR_CODE
}

fn largest_joltage(value: &str, digits: usize) -> u64 {
    let length = value.len();

    // Make the largest two digit number
    if length <= 2 {
        return value.parse::<u64>().unwrap();
    }

    let chars = value.chars().rev();

    let mut pointers: Vec<char> = chars
        .clone()
        .rev()
        .skip(length - digits)
        .take(digits)
        .collect();

    for (_i, c) in chars.enumerate().skip(digits) {
        let cv = char_value(c);

        let first = char_value(pointers[0]);

        if cv >= first {
            // Remove a value to find an increase
            // Or remove the first instance of the lowest value

            for (j, p) in pointers.clone().iter().enumerate() {
                if j < digits - 1 {
                    let next = pointers[j + 1];

                    if char_value(next) > char_value(*p) {
                        pointers.remove(j);

                        break;
                    }
                }
            }

            if pointers.len() == digits {
                let m = pointers.iter().reduce(|acc, x| acc.min(x));

                if m.is_some() {
                    for (j, p) in pointers.clone().iter().enumerate() {
                        if p == m.unwrap() {
                            pointers.remove(j);
                            break;
                        }
                    }
                }
            }

            pointers.insert(0, c);
        }
    }

    parse_chars(pointers)
}

fn parse_chars(chars: Vec<char>) -> u64 {
    let length = chars.len();

    chars.iter().enumerate().fold(0, |acc, (i, x)| {
        acc + char_value(*x) as u64 * ten_to((length - i - 1) as u32)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chars() {
        assert_eq!(parse_chars(vec!['1', '0', '0', '0']), 1000);
        assert_eq!(parse_chars(vec!['9', '0', '0', '9']), 9009);
        assert_eq!(parse_chars(vec!['1']), 1);
    }

    #[test]
    fn test_joltage() {
        assert_eq!(largest_joltage("987654321111111", DIGITS), 987654321111);
        assert_eq!(largest_joltage("811111111111119", DIGITS), 811111111119);
        assert_eq!(largest_joltage("234234234234278", DIGITS), 434234234278);
        assert_eq!(largest_joltage("818181911112111", DIGITS), 888911112111);
    }
}
