use std::str;

pub struct MyHashMap<'a> {
    data: Vec<Vec<(&'a str, u8)>>,
}

impl<'a> Default for MyHashMap<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> MyHashMap<'a> {
    pub fn new() -> MyHashMap<'a> {
        let data = vec![Vec::new(); 256];
        MyHashMap { data }
    }

    pub fn insert(&mut self, k: &'a str, v: u8) {
        let hash = hash(k);
        let entries = &mut self.data[hash as usize];
        for (i, &(ek, _)) in entries.iter().enumerate() {
            if ek == k {
                entries[i] = (k, v);
                return;
            }
        }
        entries.push((k, v));
    }

    pub fn remove(&mut self, k: &'a str) {
        let hash = hash(k);
        let entries = &mut self.data[hash as usize];
        for (i, &(ek, _)) in entries.iter().enumerate() {
            if ek == k {
                entries.remove(i);
                return;
            }
        }
    }
}

pub fn hash(s: &str) -> u8 {
    let mut curr = 0;
    for c in s.chars() {
        curr += c as u64;
        curr *= 17;
        curr %= 256;
    }
    curr as u8
}

pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut sum = 0;
    let mut curr = 0;
    for c in input.chars() {
        match c {
            ',' => {
                sum += curr;
                curr = 0;
            }
            '\n' => {
                sum += curr;
                break;
            }
            _ => {
                curr += c as u64;
                curr *= 17;
                curr %= 256;
            }
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let mut hashmap = MyHashMap::new();
    let mut bytes = input.as_bytes().iter();
    for op_slice in input.trim_end().split(',') {
        let op_idx = op_slice.find(|c| c == '=' || c == '-').unwrap();
        let op = op_slice.chars().nth(op_idx).unwrap();
        let label = &op_slice[0..op_idx];

        match op {
            '=' => {
                let val = op_slice.chars().nth(op_idx + 1).unwrap() as u8 - '0' as u8;
                hashmap.insert(label, val);
            }
            '-' => {
                hashmap.remove(label);
            }
            _ => unreachable!(),
        }
    }

    let mut sum = 0;
    for (i, entries) in hashmap.data.iter().enumerate() {
        for (j, &(_, val)) in entries.iter().enumerate() {
            sum += (i as u64 + 1) * (j as u64 + 1) * val as u64;
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day15/test.txt");
    #[test]
    fn test_day15_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(1320));
    }

    #[test]
    fn test_day15_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(145));
    }
}
