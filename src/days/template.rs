pub fn parse_input(input: &str) -> &str {
    input
}

#[allow(unused_variables)]
pub fn part1(input: &str) -> Option<u64> {
    None
}

#[allow(unused_variables)]
pub fn part2(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day00/test.txt");
    #[test]
    fn test_day00_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(input);

        assert_eq!(resp, None);
    }

    #[test]
    fn test_day00_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
