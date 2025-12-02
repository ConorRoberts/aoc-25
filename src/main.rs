use std::fs::read_to_string;

type SafeNumber = u32;

#[derive(Debug, Clone)]
enum SafeDirection {
    Left,
    Right,
}

#[derive(Debug)]
enum SafeDirectionError {
    InvalidDirection,
}

impl TryFrom<char> for SafeDirection {
    type Error = SafeDirectionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(SafeDirectionError::InvalidDirection),
        }
    }
}

const MAX_VALUE: SafeNumber = 100;

type SafeRotation = (SafeDirection, SafeNumber);

fn ten_to(exp: u32) -> u64 {
    10u64.pow(exp)
}

fn read_input() -> Vec<SafeRotation> {
    let file_result = read_to_string("./src/day-1/p1-input.txt")
        .map_err(|_| "Error reading file")
        .unwrap()
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let first = chars.next().unwrap();

            let num_count = chars.clone().count() - 1;
            let value: SafeNumber = chars.enumerate().fold(0, |acc, (i, x)| {
                let delta = (x as SafeNumber) - 48;

                if delta == 0 {
                    return acc + 0;
                }

                return acc + ten_to((num_count - i) as u32) as SafeNumber * delta;
            });

            (SafeDirection::try_from(first).unwrap(), value)
        })
        .collect();

    return file_result;
}

fn count_overflow(current: SafeNumber, rotation: &SafeRotation) -> u32 {
    match rotation.0 {
        SafeDirection::Right => (current + rotation.1) / MAX_VALUE,
        SafeDirection::Left => {
            let m = rotation.1 % MAX_VALUE;

            (rotation.1 / MAX_VALUE) + (if m >= current && current != 0 { 1 } else { 0 })
        }
    }
}

fn rotate_number(current: SafeNumber, (direction, value): SafeRotation) -> SafeNumber {
    match direction {
        SafeDirection::Right => (current + value) % MAX_VALUE,
        SafeDirection::Left => ((current + MAX_VALUE) - (value % MAX_VALUE)) % MAX_VALUE,
    }
}

// Goal, apply rotations and count how many times zero is pointed to
fn main() {
    let mut safe_number: SafeNumber = 50;
    let mut count = 0;

    let input = read_input();

    for action in input {
        count += count_overflow(safe_number, &action);

        safe_number = rotate_number(safe_number, action.clone());
    }

    println!("Final {:?}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_number() {
        let mut n: SafeNumber = 50;

        assert_eq!(count_overflow(n, &(SafeDirection::Left, 68)), 1);
        n = rotate_number(n, (SafeDirection::Left, 68));
        assert_eq!(n, 82);

        assert_eq!(count_overflow(n, &(SafeDirection::Left, 30)), 0);
        n = rotate_number(n, (SafeDirection::Left, 30));
        assert_eq!(n, 52);

        assert_eq!(count_overflow(n, &(SafeDirection::Right, 48)), 1);
        n = rotate_number(n, (SafeDirection::Right, 48));
        assert_eq!(n, 0);
    }

    #[test]
    fn test_mixed() {
        assert_eq!(count_overflow(1, &(SafeDirection::Right, 400)), 4);
        assert_eq!(count_overflow(1, &(SafeDirection::Right, 4)), 0);
        assert_eq!(count_overflow(1, &(SafeDirection::Right, 99)), 1);

        assert_eq!(count_overflow(1, &(SafeDirection::Left, 99)), 1);
        assert_eq!(count_overflow(0, &(SafeDirection::Left, 5)), 0);
        assert_eq!(count_overflow(55, &(SafeDirection::Left, 55)), 1);
        assert_eq!(count_overflow(1, &(SafeDirection::Left, 201)), 3);
        assert_eq!(count_overflow(52, &(SafeDirection::Right, 48)), 1);
        assert_eq!(count_overflow(50, &(SafeDirection::Right, 1000)), 10);
        assert_eq!(count_overflow(50, &(SafeDirection::Left, 1000)), 10);
    }
}
