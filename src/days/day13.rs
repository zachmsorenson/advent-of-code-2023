#[derive(Debug)]
pub struct Grid {
    rows: Vec<u64>,
    cols: Vec<u64>,
}

pub fn parse_cols(rows: &[u64], width: usize) -> Vec<u64> {
    let mut cols = Vec::new();
    let height = rows.len();
    for i in 0..width {
        let mask = 1 << (width - 1 - i);
        let mut val = 0;
        for (j, row) in rows.iter().enumerate() {
            let bit = (mask & row) >> (width - i - 1);
            let bit = bit << (height - j - 1);
            val |= bit;
        }
        cols.push(val);
    }

    cols
}

pub fn check_reflection(idx: usize, data: &[u64]) -> bool {
    let length = usize::min(idx + 1, data.len() - idx - 1);

    ((idx + 1 - length)..=idx)
        .rev()
        .zip(idx + 1..idx + 1 + length)
        .all(|(i, j)| data[i] == data[j])
}

pub fn check_reflection2(idx: usize, data: &[u64]) -> bool {
    let length = usize::min(idx + 1, data.len() - idx - 1);

    let mut smudge_rows = 0;
    let reflects = ((idx + 1 - length)..=idx)
        .rev()
        .zip(idx + 1..idx + 1 + length)
        .all(|(i, j)| {
            let first = data[i];
            let second = data[j];
            let one_counts = (first ^ second).count_ones();
            if one_counts == 1 {
                smudge_rows += 1;
            }
            one_counts <= 1
        });
    reflects && smudge_rows == 1
}

pub fn parse_input(input: &str) -> Vec<Grid> {
    let mut grids = Vec::new();
    let mut rows = Vec::new();
    let mut width = 0;
    for line in input.lines() {
        if line.is_empty() {
            let cols = parse_cols(&rows, width);
            let grid = Grid { rows, cols };
            grids.push(grid);
            rows = Vec::new();
            continue;
        }

        width = line.len();
        let mut bit = 1 << (width - 1);
        let mut val = 0;
        for c in line.chars() {
            if let '#' = c {
                val |= bit
            }
            bit >>= 1;
        }
        rows.push(val);
    }
    let cols = parse_cols(&rows, width);
    let grid = Grid { rows, cols };
    grids.push(grid);

    grids
}

#[allow(unused_variables)]
pub fn part1(input: &[Grid]) -> Option<u64> {
    let mut sum = 0;

    for grid in input {
        let mut found = false;

        for i in 0..grid.rows.len() - 1 {
            if check_reflection(i, &grid.rows) {
                found = true;
                sum += 100 * (i as u64 + 1);
                break;
            }
        }

        if found {
            continue;
        }

        for i in 0..grid.cols.len() - 1 {
            if check_reflection(i, &grid.cols) {
                sum += i as u64 + 1;
                break;
            }
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Grid]) -> Option<u64> {
    let mut sum = 0;

    for grid in input {
        let mut found = false;

        for i in 0..grid.rows.len() - 1 {
            if check_reflection2(i, &grid.rows) {
                found = true;
                sum += 100 * (i as u64 + 1);
                break;
            }
        }

        if found {
            continue;
        }

        for i in 0..grid.cols.len() - 1 {
            if check_reflection2(i, &grid.cols) {
                sum += i as u64 + 1;
                break;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day13/test.txt");
    #[test]
    fn test_day13_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(405));
    }

    #[test]
    fn test_day13_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(400));
    }
}
