use num::integer::lcm;
use std::collections::HashMap;

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Node {
    name: String,
    left: String,
    right: String,
    a_node: bool,
    z_node: bool,
}

pub struct Input {
    instructions: Vec<Direction>,
    map: HashMap<String, Node>,
}

pub fn parse_input(input: &str) -> Input {
    let mut it = input.lines();
    let first = it.next().unwrap();
    let instructions = first
        .chars()
        .map(|c| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect();
    it.next();

    let mut map = HashMap::new();
    for line in it {
        let (name, line) = line.split_at(3);
        let (_, line) = line.split_at(4);
        let (left, line) = line.split_at(3);
        let (_, line) = line.split_at(2);
        let (right, _) = line.split_at(3);

        let name = name.to_string();
        let left = left.to_string();
        let right = right.to_string();
        let a_node = name.ends_with('A');
        let z_node = name.ends_with('Z');

        let node = Node {
            name: name.clone(),
            left,
            right,
            a_node,
            z_node,
        };

        // println!("{:?}", node);

        // println!("{}, {}, {}", name, left, right);
        map.insert(name, node);
    }

    Input { instructions, map }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut next = "AAA";
    let mut steps = 0;
    let mut i = 0;
    let m = input.instructions.len();
    while let Some(node) = &input.map.get(next) {
        if node.z_node {
            break;
        }

        let instruction = &input.instructions[i];
        next = match instruction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };

        steps += 1;
        i = (i + 1) % m;
    }

    Some(steps)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut nodes = input.map.values().filter(|&node| node.a_node);
    let num_nodes = nodes.clone().count();
    let m = input.instructions.len();

    let mut cycles = Vec::new();
    for node in nodes {
        let mut steps: u64 = 0;
        let mut i = 0;
        let mut next = &node.name;
        while let Some(node) = &input.map.get(next) {
            if node.z_node {
                cycles.push(steps);
                break;
            }

            let instruction = &input.instructions[i];
            next = match instruction {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };

            steps += 1;
            i = (i + 1) % m;
        }
    }

    let lcm = cycles.iter().fold(1, |acc, v| lcm(acc, *v));

    Some(lcm)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day08/test.txt");
    #[test]
    fn test_day08_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(6));
    }

    const TEST_INPUT2: &str = include_str!("../../input/day08/test2.txt");
    #[test]
    fn test_day08_part2() {
        let input = parse_input(TEST_INPUT2);

        let resp = part2(&input);

        assert_eq!(resp, Some(6));
    }
}
