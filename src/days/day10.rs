use std::collections::{HashMap, HashSet};

type Point = (i32, i32);

pub fn add_points(p1: Point, p2: Point) -> Point {
    (p1.0 + p2.0, p1.1 + p2.1)
}

#[derive(PartialEq)]
pub enum Token {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Token {
    pub fn as_directions(&self) -> (Point, Point) {
        match self {
            Token::NorthEast => ((1, 0), (0, -1)),
            Token::Vertical => ((0, -1), (0, 1)),
            Token::NorthWest => ((-1, 0), (0, -1)),
            Token::Horizontal => ((1, 0), (-1, 0)),
            Token::SouthEast => ((0, 1), (1, 0)),
            Token::SouthWest => ((0, 1), (-1, 0)),
            Token::Ground => ((0, 0), (0, 0)),
            Token::Start => ((0, 0), (0, 0)),
        }
    }
}

pub struct Input {
    tokens: Vec<Vec<Token>>,
    width: i32,
    height: i32,
    start: (i32, i32),
}

impl Input {
    pub fn at(&self, x: i32, y: i32) -> &Token {
        if x < 0 || x >= self.width || y < 0 || y > self.height {
            return &Token::Ground;
        }
        &self.tokens[y as usize][x as usize]
    }

    pub fn at_move(&self, p1: Point, p2: Point) -> &Token {
        let x = p1.0 + p2.0;
        let y = p1.1 + p2.1;
        if x < 0 || x >= self.width || y < 0 || y > self.height {
            return &Token::Ground;
        }
        &self.tokens[y as usize][x as usize]
    }
}

pub fn parse_input(input: &str) -> Input {
    let mut vecs = Vec::new();
    let mut start = (0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut tokens = Vec::new();
        for (j, c) in line.chars().enumerate() {
            let token = match c {
                '|' => Token::Vertical,
                '-' => Token::Horizontal,
                'L' => Token::NorthEast,
                'J' => Token::NorthWest,
                '7' => Token::SouthWest,
                'F' => Token::SouthEast,
                '.' => Token::Ground,
                'S' => {
                    start = (j as i32, i as i32);
                    Token::Start
                }
                _ => {
                    println!("{}", c);
                    unreachable!();
                }
            };
            tokens.push(token);
        }
        vecs.push(tokens);
    }

    let width = vecs[0].len() as i32;
    let height = vecs.len() as i32;

    Input {
        tokens: vecs,
        width,
        height,
        start,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut curr_pos = input.start;

    let upper = input.at(curr_pos.0, curr_pos.1 - 1);
    let lower = input.at(curr_pos.0, curr_pos.1 + 1);
    let right = input.at(curr_pos.0 + 1, curr_pos.1);
    let left = input.at(curr_pos.0 - 1, curr_pos.1);

    let north_conn = [Token::Vertical, Token::SouthWest, Token::SouthEast].contains(upper);
    let south_conn = [Token::Vertical, Token::NorthWest, Token::NorthEast].contains(lower);
    let east_conn = [Token::Horizontal, Token::SouthWest, Token::NorthWest].contains(right);
    let west_conn = [Token::Horizontal, Token::SouthEast, Token::NorthEast].contains(left);

    let mut curr_token = match (north_conn, east_conn, south_conn, west_conn) {
        (true, true, false, false) => &Token::NorthEast,
        (true, false, true, false) => &Token::Vertical,
        (true, false, false, true) => &Token::NorthWest,
        (false, true, true, false) => &Token::SouthEast,
        (false, true, false, true) => &Token::Horizontal,
        (false, false, true, true) => &Token::SouthWest,
        _ => unreachable!(),
    };

    let mut next_move = curr_token.as_directions().0;
    let mut next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);

    let mut steps = 0;
    while curr_token != &Token::Start {
        let next_token = input.at(next_pos.0, next_pos.1);
        steps += 1;

        let possible_moves = next_token.as_directions();

        if (-next_move.0, -next_move.1) == possible_moves.0 {
            next_move = possible_moves.1;
        } else {
            next_move = possible_moves.0;
        }

        curr_pos = next_pos;
        curr_token = next_token;
        next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);
    }

    Some(steps >> 1)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut curr_pos = input.start;
    let mut loop_points = HashSet::new();

    let upper = input.at(curr_pos.0, curr_pos.1 - 1);
    let lower = input.at(curr_pos.0, curr_pos.1 + 1);
    let right = input.at(curr_pos.0 + 1, curr_pos.1);
    let left = input.at(curr_pos.0 - 1, curr_pos.1);

    let north_conn = [Token::Vertical, Token::SouthWest, Token::SouthEast].contains(upper);
    let south_conn = [Token::Vertical, Token::NorthWest, Token::NorthEast].contains(lower);
    let east_conn = [Token::Horizontal, Token::SouthWest, Token::NorthWest].contains(right);
    let west_conn = [Token::Horizontal, Token::SouthEast, Token::NorthEast].contains(left);

    let mut start_token = match (north_conn, east_conn, south_conn, west_conn) {
        (true, true, false, false) => &Token::NorthEast,
        (true, false, true, false) => &Token::Vertical,
        (true, false, false, true) => &Token::NorthWest,
        (false, true, true, false) => &Token::SouthEast,
        (false, true, false, true) => &Token::Horizontal,
        (false, false, true, true) => &Token::SouthWest,
        _ => unreachable!(),
    };

    let mut curr_token = start_token;
    let mut next_move = curr_token.as_directions().0;
    let mut next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);

    while curr_token != &Token::Start {
        let next_token = input.at(next_pos.0, next_pos.1);
        loop_points.insert(curr_pos);

        let possible_moves = next_token.as_directions();

        if (-next_move.0, -next_move.1) == possible_moves.0 {
            next_move = possible_moves.1;
        } else {
            next_move = possible_moves.0;
        }

        curr_pos = next_pos;
        curr_token = next_token;
        next_pos = (curr_pos.0 + next_move.0, curr_pos.1 + next_move.1);
    }

    let mut count = 0;
    for (y, line) in input.tokens.iter().enumerate() {
        let mut inside = false;
        let mut open_north = false;
        let mut it = line.iter();
        let mut x = 0;
        while let Some(mut token) = it.next() {
            if !loop_points.contains(&(x, y as i32)) {
                if inside {
                    count += 1;
                }
                x += 1;
                continue;
            }
            if token == &Token::Start {
                token = start_token;
            }

            x += 1;
            match *token {
                Token::Vertical => inside = !inside,
                Token::SouthEast => open_north = false,
                Token::NorthEast => open_north = true,
                Token::SouthWest => {
                    if open_north {
                        inside = !inside;
                    }
                }
                Token::NorthWest => {
                    if !open_north {
                        inside = !inside;
                    }
                }
                _ => continue,
            }
        }
    }

    Some(count)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day10/test.txt");
    #[test]
    fn test_day10_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(8));
    }

    const TEST_INPUT2: &str = include_str!("../../input/day10/test2.txt");
    #[test]
    fn test_day10_part2() {
        let input = parse_input(TEST_INPUT2);

        let resp = part2(&input);

        assert_eq!(resp, Some(10));
    }
}
