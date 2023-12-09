pub fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse().unwrap()).collect())
        .collect()
}

#[allow(unused_variables)]
pub fn part1(input: &[Vec<i64>]) -> Option<u64> {
    let mut sum = 0;
    for nums in input {
        let mut saved: Vec<i64> = Vec::new();
        let mut current = nums.clone();
        let mut iteration = 0;
        let mut running = true;
        while running {
            saved.push(current[current.len() - 1 - iteration]);
            for i in 0..=current.len() - 2 - iteration {
                current[i] = current[i + 1] - current[i];
            }

            let slice = &current[0..current.len() - 1 - iteration];
            if !slice.iter().any(|&num| num != 0) {
                running = false;
            }
            iteration += 1;
        }
        sum += saved.iter().sum::<i64>();
    }

    Some(sum as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &[Vec<i64>]) -> Option<u64> {
    let mut sum = 0;
    for nums in input {
        let mut saved: Vec<i64> = Vec::new();
        let mut current = nums.clone();
        let mut iteration = 0;
        let mut running = true;
        while running {
            saved.push(current[0]);
            for i in 0..=current.len() - 2 - iteration {
                current[i] = current[i + 1] - current[i];
            }

            let slice = &current[0..current.len() - 1 - iteration];
            if !slice.iter().any(|&num| num != 0) {
                running = false;
            }
            iteration += 1;
        }
        sum += saved.iter().rev().fold(0, |acc, v| v - acc);
    }

    Some(sum as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day09/test.txt");
    #[test]
    fn test_day09_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(114));
    }

    #[test]
    fn test_day09_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(2));
    }
}
