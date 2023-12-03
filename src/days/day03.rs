use std::collections::HashMap;

type Point = (i32, i32);

#[derive(Debug)]
pub enum Item {
    Number { value: u32, length: usize },
    Symbol(char),
}

pub fn parse_input(input: &str) -> HashMap<Point, Item> {
    let mut map: HashMap<Point, Item> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let bytes = line.as_bytes();
        let width = line.len();
        let mut j = 0;
        while j < width {
            let mut c = bytes[j] as char;
            match c {
                '0'..='9' => {
                    let start = j;
                    let mut num = 0;
                    let mut count = 0;
                    while c.is_ascii_digit() {
                        num = num * 10 + c as u32 - '0' as u32;
                        count += 1;
                        j += 1;
                        if j >= width {
                            break;
                        }
                        c = bytes[j] as char;
                    }
                    map.insert(
                        (i as i32, start as i32),
                        Item::Number {
                            value: num,
                            length: count,
                        },
                    );
                }
                '.' => {
                    j += 1;
                }
                _ => {
                    map.insert((i as i32, j as i32), Item::Symbol(c));
                    j += 1;
                }
            }
        }
    }

    map
}

#[allow(unused_variables)]
pub fn part1(input: &HashMap<Point, Item>) -> Option<u32> {
    let mut sum = 0;
    for (&(i, j), symbol) in input {
        if let Item::Number { value, length } = &symbol {
            let mut points = vec![(i, j - 1), (i, j + *length as i32)];
            for x in (j - 1)..=(j + *length as i32) {
                points.push((i - 1, x));
                points.push((i + 1, x));
            }

            for point in &points {
                match input.get(point) {
                    Some(Item::Symbol(_)) => {
                        sum += value;
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &HashMap<Point, Item>) -> Option<u32> {
    let mut map: HashMap<(i32, i32), (u32, u32, u32)> = HashMap::new();
    for (&(i, j), symbol) in input {
        if let Item::Number { value, length } = &symbol {
            let mut points = vec![(i, j - 1), (i, j + *length as i32)];
            for x in (j - 1)..=(j + *length as i32) {
                points.push((i - 1, x));
                points.push((i + 1, x));
            }

            for point in &points {
                match input.get(point) {
                    Some(Item::Symbol('*')) => {
                        let (mut count, mut num1, mut num2) = map.get(point).unwrap_or(&(0, 0, 0));
                        count += 1;
                        if count == 1 {
                            num1 = *value;
                        }
                        if count == 2 {
                            num2 = *value;
                        }
                        map.insert(*point, (count, num1, num2));
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }

    let sum = map
        .iter()
        .filter(|(_, (count, _, _))| *count == 2)
        .map(|(_, (_, num1, num2))| num1 * num2)
        .sum();

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day03/test.txt");
    #[test]
    fn test_day0_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(4361));
    }
}
