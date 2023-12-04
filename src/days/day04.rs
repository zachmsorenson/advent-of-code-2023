use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let it = line.chars();
        let skip = line.find(": ").unwrap();
        let mut it = it.skip(skip + 1);

        let mut hashset: HashSet<u32> = HashSet::new();
        while let Some(mut c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u32 - '0' as u32;
                    while let Some(c) = it.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u32 - '0' as u32;
                        } else {
                            break;
                        }
                    }
                    hashset.insert(v);
                }
                '|' => break,
                _ => continue,
            }
        }

        let mut count = 0;
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u32 - '0' as u32;
                    while let Some(c) = it.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u32 - '0' as u32;
                        } else {
                            break;
                        }
                    }
                    if hashset.contains(&v) {
                        count += 1;
                    }
                }
                '\n' => break,
                _ => continue,
            }
        }

        if count > 0 {
            sum += 1 << (count - 1);
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u32> {
    let mut counts_map = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let it = line.chars();
        let skip = line.find(": ").unwrap();
        let mut it = it.skip(skip + 1);

        let mut hashset: HashSet<u32> = HashSet::new();
        while let Some(mut c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u32 - '0' as u32;
                    while let Some(c) = it.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u32 - '0' as u32;
                        } else {
                            break;
                        }
                    }
                    hashset.insert(v);
                }
                '|' => break,
                _ => continue,
            }
        }

        let mut count = 0;
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u32 - '0' as u32;
                    while let Some(c) = it.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u32 - '0' as u32;
                        } else {
                            break;
                        }
                    }
                    if hashset.contains(&v) {
                        count += 1;
                    }
                }
                '\n' => break,
                _ => continue,
            }
        }
        let self_instances = *counts_map.get(&i).unwrap_or(&1);
        counts_map.insert(i, self_instances);
        for j in (i + 1)..=(i + count) {
            let curr_count: u32 = *counts_map.get(&j).unwrap_or(&1);
            counts_map.insert(j, curr_count + self_instances);
        }
    }

    let sum = counts_map.values().sum();
    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day04/test.txt");
    #[test]
    fn test_day4_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(13));
    }

    #[test]
    fn test_day4_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(30));
    }
}
