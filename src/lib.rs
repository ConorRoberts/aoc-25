use std::env;

pub fn ten_to(exp: u32) -> u64 {
    10u64.pow(exp)
}

pub fn parse_number(s: &str) -> u64 {
    let chars = s.chars();
    let num_count = chars.clone().count();

    let value: u64 = chars.enumerate().fold(0, |acc, (i, x)| {
        let delta = (x as u64) - 48;

        if delta == 0 {
            return acc + (if i > 0 { 1 } else { delta });
        }

        return acc + ten_to((num_count - i) as u32) as u64 * delta;
    });

    return value;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_parse_number() {
//         assert_eq!(parse_number("1000"), 1000);
//         assert_eq!(parse_number("11"), 11);
//     }
// }

pub fn get_input_file() -> String {
    let args: Vec<String> = env::args().collect();

    let file_name = args.get(1).unwrap_or_else(|| panic!("Invalid file"));

    file_name.clone()
}
