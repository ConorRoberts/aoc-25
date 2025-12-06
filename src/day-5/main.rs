use aoc_25::get_input_file;
use std::fs;

type RangeValue = u64;

#[derive(Debug, PartialEq)]
struct FreshnessRange {
    from: RangeValue,
    to: RangeValue,
}

impl From<&str> for FreshnessRange {
    fn from(value: &str) -> Self {
        let mut split = value.split("-");

        let from = split.next().unwrap().parse::<RangeValue>().unwrap();
        let to = split.next().unwrap().parse::<RangeValue>().unwrap();

        Self { from, to }
    }
}

#[derive(PartialEq, Debug)]
enum RangeContains {
    Start,
    End,
    Both,
}

impl FreshnessRange {
    fn contains_range(&self, range: &FreshnessRange) -> Option<RangeContains> {
        let contains_from = self.from <= range.from && self.to >= range.from;
        let contains_to = self.from <= range.to && self.to >= range.to;

        if contains_from && contains_to {
            return Some(RangeContains::Both);
        }

        if contains_from {
            return Some(RangeContains::Start);
        }

        if contains_to {
            return Some(RangeContains::End);
        }

        None
    }
}

fn merge_ranges(ranges: Vec<FreshnessRange>) -> Vec<FreshnessRange> {
    let ranges_acc = Vec::<FreshnessRange>::new();

    return ranges.into_iter().fold(ranges_acc, |mut acc, x| {
        // Find a range that contains the start or end value
        let matched_ranges: Vec<usize> = acc
            .iter()
            .enumerate()
            .filter(|(_, r)| x.contains_range(&r).is_some())
            .map(|(i, _)| i)
            .collect();

        // println!("Matched {:?} to {:?}", matched_ranges, x);

        if matched_ranges.len() == 0 {
            acc.push(x);

            return acc;
        }

        let mut new_record = FreshnessRange {
            from: x.from,
            to: x.to,
        };

        for i in matched_ranges.iter() {
            // is it the start or end that was matched? pick one

            let r = &acc[*i];

            let contains = new_record.contains_range(&r);

            if let Some(c) = contains {
                // println!("Merging {:?} into {:?}, {:?}", r, new_record, c);
                if c == RangeContains::Start || c == RangeContains::Both {
                    // The start of current range is within this matched range
                    // Take the greater of the two end values
                    new_record.to = new_record.to.max(r.to);
                }

                if c == RangeContains::End || c == RangeContains::Both {
                    new_record.from = new_record.from.min(r.from);
                }
            }
        }

        for i in matched_ranges.iter().rev() {
            acc.remove(*i);
        }

        acc.push(new_record);

        acc
    });
}

fn main() {
    let file_name = get_input_file();

    let file_content = fs::read_to_string(file_name).unwrap();

    let mut lines = file_content.lines();

    let ranges = merge_ranges(
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| FreshnessRange::from(line))
            .collect(),
    );

    // let mut count = 0;

    // for x in lines {
    //     let n = x.parse::<RangeValue>().unwrap();

    //     // Is the number in one of the ranges?

    //     let r = ranges.iter().find(|range| n >= range.from && n <= range.to);

    //     if r.is_some() {
    //         count += 1;
    //     }
    // }

    // 369264761051837 TOO HIGH
    // 369264761051955 TOO HIGH
    // 338307215105550 TOO LOW

    // 352037459601696 WRONG

    // println!("{:?}", ranges);
    let count = ranges.iter().fold(0, |acc, x| {
        // println!("{:?} {}", x, x.to - x.from + 1);

        acc + (x.to - x.from + 1)
    });

    println!("Total {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_ranges() {
        let input = vec![
            FreshnessRange { from: 5, to: 6 },
            FreshnessRange { from: 15, to: 25 },
            FreshnessRange { from: 0, to: 15 },
        ];
        let output = vec![FreshnessRange { from: 0, to: 25 }];

        let result = merge_ranges(input);

        println!("{:?}", result);

        assert_eq!(result.len(), output.len());
        assert!(result.into_iter().enumerate().all(|(i, x)| x == output[i]),);
    }
}
