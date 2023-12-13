#[derive(Debug)]
pub enum State {
    Good,
    Bad,
    Unknown,
    Empty,
}

#[derive(Debug)]
pub struct Record {
    data: Vec<State>,
    groups: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    for line in input.lines() {
        let mut data = vec![];
        let mut groups = Vec::new();
        let mut it = line.chars();
        while let Some(c) = it.next() {
            match c {
                '.' => data.push(State::Good),
                '#' => data.push(State::Bad),
                '?' => data.push(State::Unknown),
                ' ' => continue,
                '0'..='9' => {
                    let mut v = c as u64 - '0' as u64;
                    for c in it.by_ref() {
                        if c.is_ascii_digit() {
                            v = 10 * v + c as u64 - '0' as u64;
                        } else {
                            break;
                        }
                    }
                    groups.push(v as usize);
                }
                _ => {
                    println!("Bad: {}", c);
                    unreachable!();
                }
            }
        }
        data.push(State::Empty);

        records.push(Record { data, groups });
    }

    records
}

pub fn solve(data: &[State], groups: &[usize]) -> u64 {
    if groups.is_empty() {
        if data.iter().any(|state| matches!(state, State::Bad)) {
            return 0;
        } else {
            return 1;
        }
    }

    let group = groups[0];
    if data.len() < group {
        return 0;
    }

    let mut sum = 0;
    let mut start = 0;
    while start < data.len() - group {
        let end = start + group;

        if !matches!(data[start], State::Bad | State::Unknown) {
            start += 1;
            continue;
        }

        let mut prod = 1;
        if data[start..end]
            .iter()
            .all(|state| matches!(state, State::Bad | State::Unknown))
            && matches!(data[end], State::Good | State::Unknown | State::Empty)
        {
            if end < data.len() {
                prod *= solve(&data[end + 1..], &groups[1..]);
            }
        } else {
            prod = 0;
        }
        sum += prod;
        start += 1;
    }
    sum
}

pub fn fill_row(
    data: &[State],
    prev_memo: &[u64],
    row_start: usize,
    group: usize,
    curr_memo: &mut [u64],
) -> Option<usize> {
    let mut prev_sum = 0;
    let mut first_fill = None;

    (0..data.len()).for_each(|i| {
        curr_memo[i] = 0;
    });

    for start in row_start..data.len() - group - 1 {
        prev_sum += prev_memo[start];
        let end = start + group + 1;
        if matches!(data[start], State::Good | State::Unknown | State::Empty)
            && matches!(data[end], State::Good | State::Unknown | State::Empty)
            && data[start + 1..end]
                .iter()
                .all(|state| matches!(&state, &State::Bad | &State::Unknown))
        {
            // group can fit into this slice. Set memo[end] to current sum of previous memo
            curr_memo[end] = prev_sum;

            if first_fill.is_none() {
                first_fill = Some(end);
            }
        } else {
            curr_memo[end] = 0;
        }
    }

    first_fill
}

#[allow(unused_variables)]
pub fn part1(input: &[Record]) -> Option<u64> {
    let mut sum = 0;

    for record in input {
        let count = solve(&record.data, &record.groups);
        sum += count;
        // let mut memos = Vec::new();
        // let start_memo = vec![0; record.data.len()];
        // start_memo[0] = 1;
        // memos.push(start_memo);
        // for _ in &record.groups {
        //     memos.push(vec![0; record.data.len()]);
        // }
        // let mut row_start = 0;
        //
        // // Fill the table
        // for (i, &group) in &record.groups.enumerate() {
        //     let first_filled = fill_row(&record.data, &prev_memo, row_start, group, &mut curr_memo);
        //     std::mem::swap(&mut curr_memo, &mut prev_memo);
        //     row_start = first_filled.unwrap();
        // }
        //
        // // Sum up the last row, which is now prev_memo
        // let count = prev_memo.iter().sum::<u64>();
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Record]) -> Option<u64> {
    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day12/test.txt");
    #[test]
    fn test_day12_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(21));
    }

    #[test]
    fn test_day12_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
