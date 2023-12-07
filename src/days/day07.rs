use std::{cmp::Ordering, collections::HashMap};

pub struct Hand {
    raw: [char; 5],
    bid: u64,
}

pub fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let mut raw = ['0'; 5];
        for i in 0..5 {
            let c = chars.next().unwrap();
            raw[i] = c;
        }

        chars.next();
        let mut bid = 0;
        for c in chars {
            bid = 10 * bid + c as u64 - '0' as u64;
        }

        let hand = Hand { raw, bid };
        hands.push(hand);
    }

    hands
}

#[allow(unused_variables)]
pub fn part1(input: &[Hand]) -> Option<u64> {
    let mut parsed_hands = Vec::new();
    for hand in input {
        let mut ranked = [0; 5];
        let mut map = HashMap::new();
        for (i, c) in hand.raw.iter().enumerate() {
            let rank = match c {
                '0'..='9' => *c as u8 - '0' as u8,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            };
            ranked[i] = rank;
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }

        let mut counts = [0; 5];
        for (i, &count) in map.values().enumerate() {
            counts[i] = count;
        }
        counts.sort_by(|a, b| b.cmp(a));

        parsed_hands.push((counts, ranked, hand.bid));
    }

    parsed_hands.sort_by(|(a_counts, a_ranked, _), (b_counts, b_ranked, _)| {
        let ord = a_counts.cmp(b_counts);
        match ord {
            Ordering::Less | Ordering::Greater => ord,
            Ordering::Equal => a_ranked.cmp(b_ranked),
        }
    });

    let sum = parsed_hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as u64 + 1) * bid)
        .sum();
    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Hand]) -> Option<u64> {
    let mut parsed_hands = Vec::new();
    for hand in input {
        let mut ranked = [0; 5];
        let mut map = HashMap::new();
        let mut jokers = 0;
        for (i, c) in hand.raw.iter().enumerate() {
            let rank = match c {
                '0'..='9' => *c as u8 - '0' as u8,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            };
            ranked[i] = rank;

            if *c == 'J' {
                jokers += 1;
            } else {
                map.entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }

        let mut counts = [0; 5];
        for (i, &count) in map.values().enumerate() {
            counts[i] = count;
        }
        counts.sort_by(|a, b| b.cmp(a));
        counts[0] += jokers;

        parsed_hands.push((counts, ranked, hand.bid));
    }

    parsed_hands.sort_by(|(a_counts, a_ranked, _), (b_counts, b_ranked, _)| {
        let ord = a_counts.cmp(b_counts);
        match ord {
            Ordering::Less | Ordering::Greater => ord,
            Ordering::Equal => a_ranked.cmp(b_ranked),
        }
    });

    let sum = parsed_hands
        .iter()
        .enumerate()
        .map(|(rank, (_, _, bid))| (rank as u64 + 1) * bid)
        .sum();
    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day07/test.txt");
    #[test]
    fn test_day07_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(6440));
    }

    #[test]
    fn test_day07_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(5905));
    }
}
