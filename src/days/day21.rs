use std::collections::{HashMap, HashSet};

type Point = (i64, i64);
const ADJ: [Point; 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub struct Input {
    grid: Vec<Vec<u8>>,
    start: Point,
    steps1: i64,
    steps2: i64,
}

pub fn parse_input(input: &str) -> Input {
    let mut grid = Vec::new();
    let mut start: Point = (0, 0);
    for (i, line) in input.lines().enumerate() {
        if let Some(j) = line.find('S') {
            start = (j as i64, i as i64);
        };

        grid.push(line.as_bytes().to_vec());
    }

    Input {
        grid,
        start,
        steps1: 64,
        steps2: 26501365,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut points = HashSet::new();
    let height = input.grid.len();
    let width = input.grid[0].len();
    points.insert(input.start);
    for i in 0..input.steps1 {
        let mut next_points = HashSet::new();
        for point in points {
            let adj_points = ADJ
                .iter()
                .map(|&(x, y)| (point.0 + x, point.1 + y))
                .filter(|&(x, y)| 0 <= x && x < width as i64 && 0 <= y && y < height as i64)
                .filter(|&(x, y)| input.grid[y as usize][x as usize] != b'#');
            for adj_point in adj_points {
                next_points.insert(adj_point);
            }
        }

        points = next_points;
    }

    let sum = points.len();
    Some(sum as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut grid = Vec::new();
    for _ in 0..2 {
        for line in &input.grid {
            let mut new_line = Vec::new();
            new_line.extend_from_slice(line);
            new_line.extend_from_slice(line);
            grid.push(new_line);
        }
    }

    let mut points = HashSet::new();
    let mut seen = HashSet::new();
    let height = grid.len();
    let width = grid[0].len();
    println!(
        "height: {}, width: {}, oldheight: {}, oldwidth: {}",
        height,
        width,
        input.grid.len(),
        input.grid[0].len(),
    );

    let mut odds = 0;
    let mut evens = 1; // starting point is step 0
    let mut rocks = 0;
    points.insert(input.start);
    seen.insert(input.start);
    let mut map = HashMap::new();
    map.insert(0, (odds, evens));

    let mut prev_odds = 1;
    let mut prev_evens = 1;
    for i in 0..input.steps2 {
        // println!("i: {}\t\ti*i = {}", i, i * i);
        odds = i * i;
    }
    // for i in 1..400 {
    //     let mut next_points = HashSet::new();
    //     for point in points {
    //         let adj_points = ADJ
    //             .iter()
    //             .map(|&(x, y)| (point.0 + x, point.1 + y))
    //             .filter(|&(x, y)| 0 <= x && x < width as i64 && 0 <= y && y < height as i64);
    //         for (x, y) in adj_points {
    //             match grid[y as usize][x as usize] {
    //                 b'#' => {
    //                     let is_new = seen.insert((x, y));
    //                     if is_new {
    //                         rocks += 1;
    //                     }
    //                 }
    //                 _ => {
    //                     next_points.insert((x, y));
    //                     let is_new = seen.insert((x, y));
    //                     if is_new && i % 2 == 1 {
    //                         odds += 1;
    //                     } else if is_new && i % 2 == 0 {
    //                         evens += 1;
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     points = next_points;
    //     map.insert(i, (odds, evens));
    //     println!(
    //         "Finished step: {}, odds: {}, evens: {}, rocks: {}, sum: {}",
    //         i,
    //         odds,
    //         evens,
    //         rocks,
    //         odds + evens + rocks
    //     );
    // }

    // let (odds, evens) = println!("{:?}", map);

    Some(odds as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day21/test.txt");
    #[test]
    fn test_day21_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, None);
    }

    #[test]
    fn test_day21_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
