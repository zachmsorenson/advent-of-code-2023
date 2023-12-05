use std::collections::hash_map;

pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut sum = 0;
    for l in input.lines() {
        let mut found_first = false;
        let mut first = 0;
        let mut last = 0;
        for c in l.chars() {
            match c {
                '0'..='9' => {
                    if !found_first {
                        first = c as u64 - '0' as u64;
                        found_first = true;
                    }
                    last = c as u64 - '0' as u64;
                }
                _ => continue,
            }
        }
        sum += first * 10 + last;
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let number_strs = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut sum = 0;
    for l in input.lines() {
        let length = l.len();
        let mut first = None;
        let mut last = None;
        for (i, c) in l.chars().enumerate() {
            if c.is_ascii_digit() {
                if first.is_none() {
                    first = Some(c as u64 - '0' as u64);
                }
                last = Some(c as u64 - '0' as u64);
                continue;
            };
            for (j, &number) in number_strs.iter().enumerate() {
                if number.len() > length - i {
                    continue;
                }
                if &l[i..i + number.len()] == number {
                    if first.is_none() {
                        first = Some(j as u64);
                    }
                    last = Some(j as u64);
                    break;
                }
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day01/test.txt");
    #[test]
    fn test_day1_part_one() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, None);
    }
}
