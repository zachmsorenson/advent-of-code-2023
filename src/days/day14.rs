use std::{cell::RefCell, collections::HashMap};

pub fn parse_input(input: &str) -> RefCell<Vec<Vec<u8>>> {
    let mut bytes = Vec::new();
    for line in input.lines() {
        bytes.push(line.as_bytes().to_vec());
    }

    RefCell::new(bytes)
}

// pub fn spin(input: &mut Input) {
//     // north
//     for x in 0..input.width {
//         let mut round_count = 0;
//         let mut start = 0;
//         for y in 0..input.height {
//             let space = input.spaces[y * input.width + x];
//             match space {
//                 Space::Round => round_count += 1;
//                 Space::Empty => continue,
//                 Space::Square =>
//
//             }
//         }
//     }
// }

pub fn shift_north(input: &mut Vec<Vec<u8>>) {
    for x in 0..input[0].len() {
        let mut round_count = 0;
        let mut prev = 0;
        for y in 0..input.len() {
            match input[y][x] {
                b'O' => round_count += 1,
                b'#' => {
                    if round_count > 0 {
                        for j in prev..prev + round_count {
                            input[j][x] = b'O';
                        }
                        for j in prev + round_count..y {
                            input[j][x] = b'.';
                        }
                    }
                    prev = y + 1;
                    round_count = 0;
                }
                _ => continue,
            }
        }
        if round_count > 0 {
            for j in prev..prev + round_count {
                input[j][x] = b'O';
            }
            for j in prev + round_count..input.len() {
                input[j][x] = b'.';
            }
        }
    }
}

pub fn shift_west(input: &mut Vec<Vec<u8>>) {
    for y in 0..input.len() {
        let mut round_count = 0;
        let mut prev = 0;
        for x in 0..input[0].len() {
            match input[y][x] {
                b'O' => round_count += 1,
                b'#' => {
                    if round_count > 0 {
                        for j in prev..prev + round_count {
                            input[y][j] = b'O';
                        }
                        for j in prev + round_count..x {
                            input[y][j] = b'.';
                        }
                    }
                    prev = x + 1;
                    round_count = 0;
                }
                _ => continue,
            }
        }
        if round_count > 0 {
            for j in prev..prev + round_count {
                input[y][j] = b'O';
            }
            for j in prev + round_count..input[0].len() {
                input[y][j] = b'.';
            }
        }
    }
}

pub fn shift_south(input: &mut Vec<Vec<u8>>) {
    for x in 0..input[0].len() {
        let mut round_count = 0;
        let mut prev = input.len();
        for y in (0..input.len()).rev() {
            match input[y][x] {
                b'O' => round_count += 1,
                b'#' => {
                    if round_count > 0 {
                        for j in y + 1..prev - round_count {
                            input[j][x] = b'.';
                        }
                        for j in prev - round_count..prev {
                            input[j][x] = b'O';
                        }
                    }
                    prev = y;
                    round_count = 0;
                }
                _ => continue,
            }
        }
        if round_count > 0 {
            for j in 0..prev - round_count {
                input[j][x] = b'.';
            }
            for j in prev - round_count..prev {
                input[j][x] = b'O';
            }
        }
    }
}

pub fn shift_east(input: &mut Vec<Vec<u8>>) {
    for y in 0..input.len() {
        let mut round_count = 0;
        let mut prev = input[0].len();
        for x in (0..input[0].len()).rev() {
            match input[y][x] {
                b'O' => round_count += 1,
                b'#' => {
                    if round_count > 0 {
                        for j in x + 1..prev - round_count {
                            input[y][j] = b'.';
                        }
                        for j in prev - round_count..prev {
                            input[y][j] = b'O';
                        }
                    }
                    prev = x;
                    round_count = 0;
                }
                _ => continue,
            }
        }
        if round_count > 0 {
            for j in 0..prev - round_count {
                input[y][j] = b'.';
            }
            for j in prev - round_count..prev {
                input[y][j] = b'O';
            }
        }
    }
}

#[allow(unused_variables)]
pub fn part1(input: &RefCell<Vec<Vec<u8>>>) -> Option<u64> {
    let mut sum = 0;
    let mut input = input.borrow_mut();

    shift_north(&mut input);

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            match input[y][x] {
                b'O' => sum += input.len() - y,
                _ => continue,
            }
        }
    }

    Some(sum as u64)
}

#[allow(unused_variables)]
pub fn part2(input: &RefCell<Vec<Vec<u8>>>) -> Option<u64> {
    const END: usize = 1000000000;
    let mut sum = 0;
    let mut input = input.borrow_mut();

    let mut state_counts: HashMap<String, i32> = HashMap::new();
    let mut cycle_length = 0;
    let mut curr = 0;
    let mut first_cycle_found = false;

    for i in 0..END {
        shift_north(&mut input);
        shift_west(&mut input);
        shift_south(&mut input);
        shift_east(&mut input);

        let mut s = String::new();
        for bytes in input.iter() {
            s.push_str(&String::from_utf8(bytes.clone()).unwrap());
        }
        let count = state_counts.entry(s).and_modify(|e| *e += 1).or_insert(1);
        if *count == 2 && !first_cycle_found {
            curr = i;
            first_cycle_found = true;
        }
        if *count == 3 {
            cycle_length = i - curr;
            curr = i;
            break;
        }
    }

    let target = (END - 1 - curr) % cycle_length;
    let mut curr = 0;
    while curr != target {
        shift_north(&mut input);
        shift_west(&mut input);
        shift_south(&mut input);
        shift_east(&mut input);
        curr = (curr + 1) % cycle_length;

        let mut new_sum = 0;
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                match input[y][x] {
                    b'O' => new_sum += input.len() - y,
                    _ => continue,
                }
            }
        }
    }

    for x in 0..input[0].len() {
        for y in 0..input.len() {
            match input[y][x] {
                b'O' => sum += input.len() - y,
                _ => continue,
            }
        }
    }

    Some(sum as u64)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day14/test.txt");
    #[test]
    fn test_day14_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(136));
    }

    #[test]
    fn test_day14_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(64));
    }
}
