use std::collections::HashSet;

pub struct Point {
    x: u64,
    y: u64,
}

pub struct Input {
    galaxy_coords: Vec<Point>,
    row_costs: Vec<bool>,
    col_costs: Vec<bool>,
}

pub fn parse_input(input: &str) -> Input {
    let mut galaxies = Vec::new();
    let mut row_costs = Vec::new();
    let mut col_costs = Vec::new();
    let mut width = 0;
    let mut occupied_columns = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let mut empty_row = true;
        for (x, c) in line.chars().enumerate() {
            width = x + 1;
            match c {
                '.' => continue,
                '#' => {
                    galaxies.push(Point {
                        x: x as u64,
                        y: y as u64,
                    });
                    empty_row = false;
                    occupied_columns.insert(x);
                }
                _ => unreachable!(),
            }
        }
        if empty_row {
            row_costs.push(true);
        } else {
            row_costs.push(false);
        }
    }

    for i in 0..width {
        if occupied_columns.contains(&i) {
            col_costs.push(false);
        } else {
            col_costs.push(true);
        }
    }

    Input {
        galaxy_coords: galaxies,
        row_costs,
        col_costs,
    }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut sum = 0;
    let row_cost_scan: Vec<u64> = input
        .row_costs
        .iter()
        .map(|b| match b {
            true => 2,
            false => 1,
        })
        .scan(0, |acc, cost| {
            *acc += cost;
            Some(*acc)
        })
        .collect();
    let col_cost_scan: Vec<u64> = input
        .col_costs
        .iter()
        .map(|b| match b {
            true => 2,
            false => 1,
        })
        .scan(0, |acc, cost| {
            *acc += cost;
            Some(*acc)
        })
        .collect();
    for (i, &Point { x: x1, y: y1 }) in input.galaxy_coords.iter().enumerate() {
        for &Point { x: x2, y: y2 } in &input.galaxy_coords[i + 1..] {
            let mut cost = 0;

            let xl = u64::min(x1, x2);
            let xh = u64::max(x1, x2);
            let yl = u64::min(y1, y2);
            let yh = u64::max(y1, y2);

            cost += row_cost_scan[yh as usize] - row_cost_scan[yl as usize];
            cost += col_cost_scan[xh as usize] - col_cost_scan[xl as usize];
            sum += cost;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut sum = 0;
    let row_cost_scan: Vec<u64> = input
        .row_costs
        .iter()
        .map(|b| match b {
            true => 1000000,
            false => 1,
        })
        .scan(0, |acc, cost| {
            *acc += cost;
            Some(*acc)
        })
        .collect();
    let col_cost_scan: Vec<u64> = input
        .col_costs
        .iter()
        .map(|b| match b {
            true => 1000000,
            false => 1,
        })
        .scan(0, |acc, cost| {
            *acc += cost;
            Some(*acc)
        })
        .collect();
    for (i, &Point { x: x1, y: y1 }) in input.galaxy_coords.iter().enumerate() {
        for &Point { x: x2, y: y2 } in &input.galaxy_coords[i + 1..] {
            let mut cost = 0;

            let xl = u64::min(x1, x2);
            let xh = u64::max(x1, x2);
            let yl = u64::min(y1, y2);
            let yh = u64::max(y1, y2);

            cost += row_cost_scan[yh as usize] - row_cost_scan[yl as usize];
            cost += col_cost_scan[xh as usize] - col_cost_scan[xl as usize];
            sum += cost;
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day11/test.txt");
    #[test]
    fn test_day11_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(374));
    }

    #[test]
    fn test_day11_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(82000210));
    }
}
