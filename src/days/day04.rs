use std::collections::{HashMap, HashSet};

pub struct Card {
    winners: HashSet<u32>,
    numbers: Vec<u32>,
    count: u32,
}

pub fn parse_input(input: &str) -> Vec<Card> {
    let mut output = Vec::new();
    for line in input.lines() {
        let it = line.chars();
        let skip = line.find(": ").unwrap();
        let mut it = it.skip(skip + 1);

        let mut flag = true;
        let mut winners: HashSet<u32> = HashSet::new();
        let mut numbers: Vec<u32> = Vec::new();
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

                    if flag {
                        winners.insert(v);
                    } else {
                        numbers.push(v);
                        if winners.contains(&v) {
                            count += 1;
                        }
                    }
                }
                '|' => flag = false,
                _ => continue,
            }
        }

        output.push(Card {
            winners,
            numbers,
            count,
        });
    }

    output
}

#[allow(unused_variables)]
pub fn part1(input: &[Card]) -> Option<u32> {
    let sum = input
        .iter()
        .map(|c| c.count)
        .filter(|&c| c > 0)
        .map(|c| 1 << (c - 1))
        .sum();
    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Card]) -> Option<u32> {
    let mut counts_map = HashMap::new();
    for (i, card) in input.iter().enumerate() {
        let self_instances = *counts_map.get(&i).unwrap_or(&1);
        counts_map.insert(i, self_instances);
        for j in (i + 1)..=(i + card.count as usize) {
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
