use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
};

pub struct Point {
    x: i32,
    y: i32,
}

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_unit(&self) -> Point {
        match self {
            Direction::North => Point { x: 0, y: -1 },
            Direction::East => Point { x: 1, y: 0 },
            Direction::South => Point { x: 0, y: 1 },
            Direction::West => Point { x: -1, y: 0 },
        }
    }
}

pub struct Node {
    score: u64,
    cursor: Cursor,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

pub struct Graph {
    data: Vec<Vec<u8>>,
}

pub struct Cursor {
    x: i32,
    y: i32,
    facing: Direction,
}

// impl Graph {
//     pub fn moves(&self, from: Point, facing: Direction) -> Vec<Cursor> {
//         let mut neighbors = match facing {
//             Direction::North | Direction::South => {
//                 vec![
//                     (-3, 0, Direction::West),
//                     (-2, 0, Direction::West),
//                     (-1, 0, Direction::West),
//                     (1, 0, Direction::East),
//                     (2, 0, Direction::East),
//                     (3, 0, Direction::East),
//                 ]
//             }
//             Direction::East | Direction::West => {
//                 vec![
//                     (0, -3, Direction::South),
//                     (0, -2, Direction::South),
//                     (0, -1, Direction::South),
//                     (0, 1, Direction::North),
//                     (0, 2, Direction::North),
//                     (0, 3, Direction::North),
//                 ]
//             }
//         };
//
//         let neighbors = neighbors
//             .iter()
//             .map(|(x, y, dir)| (from.x + x, from.y + y, dir))
//             .filter(|&(x, y, &dir)| {
//                 0 <= x && x <= self.data[0].len() as i32 && 0 <= y && y <= self.data.len() as i32
//             })
//             .map(|(x, y, dir)| Cursor {
//                 x,
//                 y,
//                 facing: dir.clone(),
//             })
//             .collect();
//         neighbors
//     }
//
//     pub fn cost(&self, from: Point, to: Point, direction: Direction) -> u64 {
//         let mut sum = 0;
//         let mut curr = from;
//         let unit = direction.as_unit();
//         while curr.x != to.x && curr.y != to.y {
//             curr.x += unit.x;
//             curr.y += unit.y;
//             sum += self.data[curr.y as usize][curr.x as usize] as u64;
//         }
//         sum
//     }
// }

pub fn parse_input(input: &str) -> Graph {
    let mut bytes = Vec::new();
    for line in input.lines() {
        let mut v = Vec::new();
        for b in line.as_bytes() {
            let b = b - '0' as u8;
            v.push(b);
        }
        bytes.push(v);
    }

    Graph { data: bytes }
}

// pub fn dijkstra(graph: &Graph, start: Point, target: Point) {
//     let mut dist_map = HashMap::new();
//     let mut prev_point_map = HashMap::new();
//
//     let mut heap = BinaryHeap::new();
//
//     let starting_moves = graph.moves(start, Direction::South);
//     starting_moves.append(&mut graph.moves(start, Direction::East));
//
//     for Cursor { x, y, facing } in starting_moves {
//         let cost = graph.cost(start, Point { x, y }, facing);
//         heap.push(Node {
//             score: cost,
//             cursor: Cursor { x, y, facing },
//         });
//     }
//
//     while let Some(node) = heap.pop() {
//         let moves = graph.neighbors(node.point);
//     }
// }

// pub fn best_path(start: Point, end: Point, data: &[Vec<u8>], )

#[allow(unused_variables)]
pub fn part1(input: &Graph) -> Option<u64> {
    // let curr = (0, 0);
    // let end = (input.data[0].len() - 1, input.data.len() - 1);
    //
    // let mut cost = 0;
    // let mut heap = BinaryHeap::new();
    // while curr != end {}

    None
}

#[allow(unused_variables)]
pub fn part2(input: &Graph) -> Option<u64> {
    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day17/test.txt");
    #[test]
    fn test_day17_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, None);
    }

    #[test]
    fn test_day17_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
