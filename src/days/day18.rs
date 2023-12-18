#[derive(Debug)]
pub struct Command {
    x1: i64,
    y1: i64,
    dist1: i64,
    x2: i64,
    y2: i64,
    dist2: i64,
}

fn area_of_points(points: &[(i64, i64)]) -> u64 {
    let mut sum = 0;
    for i in 0..points.len() - 1 {
        let p1 = points[i];
        let p2 = points[i + 1];

        let area = (p1.0 - p2.0) * (p1.1 + p2.1);
        sum += area;
    }

    if sum < 0 {
        sum = -sum;
    }

    sum >>= 1;
    sum as u64
}

pub fn parse_input(input: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();

        let dir = chars.next().unwrap();
        chars.next();

        let mut dist1 = chars.next().unwrap() as i64 - '0' as i64;
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                dist1 = 10 * dist1 + c as i64 - '0' as i64;
            } else {
                break;
            }
        }
        let (x1, y1) = match dir {
            'U' => (0, 1),
            'D' => (0, -1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => unreachable!(),
        };

        chars.next();
        chars.next(); // Skip past '#' char

        let s: String = chars.clone().take(5).collect();
        let dist2: i64 = i64::from_str_radix(&s, 16).unwrap();
        let dir2 = chars.nth(5).unwrap();

        let (x2, y2) = match dir2 {
            '3' => (0, 1),
            '1' => (0, -1),
            '2' => (-1, 0),
            '0' => (1, 0),
            _ => unreachable!(),
        };

        let command = Command {
            x1,
            y1,
            dist1,
            x2,
            y2,
            dist2,
        };
        commands.push(command);
    }

    commands
}

#[allow(unused_variables)]
pub fn part1(input: &[Command]) -> Option<u64> {
    let mut curr = (0, 0);
    let mut points = vec![(0, 0)];

    let mut line_sum = 0;
    for command in input {
        let next_point = (
            curr.0 + command.x1 * command.dist1,
            curr.1 + command.y1 * command.dist1,
        );
        points.push(next_point);
        curr = next_point;
        line_sum += command.dist1;
    }
    line_sum >>= 1;

    let sum = area_of_points(&points) + line_sum as u64 + 1;

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Command]) -> Option<u64> {
    let mut curr = (0, 0);
    let mut points = vec![(0, 0)];

    let mut line_sum = 0;
    for command in input {
        let next_point = (
            curr.0 + command.x2 * command.dist2,
            curr.1 + command.y2 * command.dist2,
        );
        points.push(next_point);
        curr = next_point;
        line_sum += command.dist2;
    }
    line_sum >>= 1;

    let sum = area_of_points(&points) + line_sum as u64 + 1;

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day18/test.txt");
    #[test]
    fn test_day18_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(62));
    }

    #[test]
    fn test_day18_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(952408144115));
    }
}
