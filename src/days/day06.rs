pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    let mut nums = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u64 - '0' as u64;
                    while let Some(c) = chars.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u64 - '0' as u64;
                        } else {
                            break;
                        }
                    }
                    nums.push(v);
                }
                _ => continue,
            }
        }
    }

    let mut vals = Vec::new();
    let count = nums.len() >> 1;
    for i in 0..count {
        vals.push((nums[i], nums[i + count]));
    }

    let mut prod = 1;
    for (time, dist) in vals {
        let possible = (0..time)
            .map(|i| (time - i) * i)
            .filter(|&d| d > dist)
            .count();
        prod *= possible as u64;
    }

    Some(prod)
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    let mut nums = [0; 2];
    let mut i = 0;
    for line in input.lines() {
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '0'..='9' => {
                    let mut v = c as u64 - '0' as u64;
                    while let Some(c) = chars.next() {
                        if c.is_ascii_digit() {
                            v = v * 10 + c as u64 - '0' as u64;
                        } else {
                            continue;
                        }
                    }
                    nums[i] = v;
                    i += 1;
                }
                _ => continue,
            }
        }
    }
    let time = nums[0];
    let dist = nums[1];

    // Use quadratic formula and find roots of formula x * (time - x) = dist
    let first_root = (time as f64 - f64::sqrt((time * time - 4 * dist) as f64)) / 2.0;
    let second_root = time - f64::ceil(first_root) as u64;

    Some(second_root - first_root as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day06/test.txt");
    #[test]
    fn test_day06_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, Some(288));
    }

    #[test]
    fn test_day06_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(input);

        assert_eq!(resp, Some(71503));
    }
}
