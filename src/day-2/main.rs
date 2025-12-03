use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_name = args.get(1).unwrap_or_else(|| panic!("Invalid file"));

    let total = fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("Error reading file"))
        .split(",")
        .fold(0, |acc, x| {
            let mut split_str = x.split("-");
            let first = split_str
                .next()
                .unwrap_or_else(|| panic!("First chunk missing"))
                .parse::<u64>()
                .unwrap();

            let second = split_str
                .next()
                .unwrap_or_else(|| panic!("First chunk missing"))
                .parse::<u64>()
                .unwrap();

            let sum = get_invalid_ids(first, second)
                .iter()
                .fold(0, |acc, x| acc + x);

            acc + sum
        });

    println!("Total {total}");
}

fn get_invalid_ids(start: u64, end: u64) -> Vec<u64> {
    return (start..end + 1).filter(|x| !is_valid_id(&x)).collect();
}

fn is_valid_id(id: &u64) -> bool {
    let s = id.to_string();
    let length = s.len();

    if length < 2 {
        return true;
    }

    let mut chunk_size = length / 2;

    while chunk_size > 0 {
        match chunks(&s, chunk_size) {
            Some(x) => {
                if same_chunks(&x) {
                    return false;
                }
            }
            None => (),
        }

        chunk_size -= 1;
    }

    true
}

fn same_chunks(chunks: &Vec<&str>) -> bool {
    let first = chunks[0];

    chunks.iter().all(|x| *x == first)
}

fn chunks(value: &str, chunk_size: usize) -> Option<Vec<&str>> {
    let length = value.len();

    // We can't evenly chunk if the length does not evenly divide by the chunk size
    if length % chunk_size != 0 {
        return None;
    }

    let n_chunks = length / chunk_size;

    let mut chunk_list: Vec<&str> = Vec::new();

    for i in 0..n_chunks {
        let start = chunk_size * i;
        let end = start + chunk_size;

        let slice = &value[start..end];

        chunk_list.push(slice);
    }

    Some(chunk_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_id() {
        assert_eq!(is_valid_id(&11), false);
        assert_eq!(is_valid_id(&22), false);
        assert_eq!(is_valid_id(&1212), false);
        assert_eq!(is_valid_id(&111), false);
        assert_eq!(is_valid_id(&950950), false);
        assert_eq!(is_valid_id(&2121212118), true);
    }

    #[test]
    fn test_count() {
        assert_eq!(get_invalid_ids(11, 22), vec![11, 12]);
    }

    #[test]
    fn test_chunking() {
        assert_eq!(chunks("111", 1), Some(vec!["1", "1", "1"]));
        assert_eq!(chunks("111", 2), None);
        assert_eq!(chunks("252525", 2), Some(vec!["25", "25", "25"]));
        assert_eq!(chunks("252525", 3), Some(vec!["252", "525"]));
    }
}
