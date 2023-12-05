use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<i32> {
    let mut output = Vec::new();
    for line in input.lines() {
        let it = line.chars();
        let skip = line.find(": ").unwrap();
        let mut it = it.skip(skip + 1);

        let mut flag = true;
        let mut winners: HashSet<u64> = HashSet::new();
        let mut count = 0;
        while let Some(c) = it.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u64 - '0' as u64;
                    for c in it.by_ref() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u64 - '0' as u64;
                        } else {
                            break;
                        }
                    }

                    if flag {
                        winners.insert(v);
                    } else if winners.contains(&v) {
                        count += 1;
                    }
                }
                '|' => flag = false,
                _ => continue,
            }
        }

        output.push(count);
    }

    output
}

#[allow(unused_variables)]
pub fn part1(input: &[i32]) -> Option<u64> {
    let sum = input.iter().filter(|&&c| c > 0).map(|c| 1 << (c - 1)).sum();
    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[i32]) -> Option<u64> {
    let mut counts_map = HashMap::new();
    for (i, &count) in input.iter().enumerate() {
        let self_instances = *counts_map.get(&i).unwrap_or(&1);
        counts_map.insert(i, self_instances);
        for j in (i + 1)..=(i + count as usize) {
            let curr_count: u64 = *counts_map.get(&j).unwrap_or(&1);
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

        let resp = part1(&input);

        assert_eq!(resp, Some(13));
    }

    #[test]
    fn test_day4_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(30));
    }
}
