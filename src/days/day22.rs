use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct Brick {
    xlow: i32,
    xhigh: i32,
    ylow: i32,
    yhigh: i32,
    zlow: i32,
    zhigh: i32,
    supports: Vec<usize>,
    supported_by: Vec<usize>,
}

pub fn parse_input(input: &str) -> Vec<Brick> {
    let mut bricks = Vec::new();
    for line in input.lines() {
        let splits = line.split(&['~', ',']);
        let mut nums = splits.map(|n| n.parse().unwrap());

        let xlow = nums.next().unwrap();
        let ylow = nums.next().unwrap();
        let zlow = nums.next().unwrap();
        let xhigh = nums.next().unwrap();
        let yhigh = nums.next().unwrap();
        let zhigh = nums.next().unwrap();

        let brick = Brick {
            xlow,
            xhigh,
            ylow,
            yhigh,
            zlow,
            zhigh,
            supports: vec![],
            supported_by: vec![],
        };
        bricks.push(brick);
    }
    bricks.sort_by_key(|brick| brick.zlow);

    let mut fallen_bricks: Vec<Brick> = Vec::new();
    for brick in bricks.iter() {
        let stop = fallen_bricks
            .iter()
            .filter(|b| b.zhigh < brick.zlow)
            .filter(|b| {
                brick.xlow <= b.xhigh
                    && brick.xhigh >= b.xlow
                    && brick.ylow <= b.yhigh
                    && brick.yhigh >= b.ylow
            })
            .filter(|&b| brick != b)
            .max_by_key(|b| b.zhigh);

        let new_zlow = match stop {
            None => 1,
            Some(b) => b.zhigh + 1,
        };
        let new_zhigh = brick.zhigh - brick.zlow + new_zlow;
        fallen_bricks.push(Brick {
            xlow: brick.xlow,
            xhigh: brick.xhigh,
            ylow: brick.ylow,
            yhigh: brick.yhigh,
            zlow: new_zlow,
            zhigh: new_zhigh,
            supports: vec![],
            supported_by: vec![],
        });
    }
    let mut bricks = fallen_bricks;
    bricks.sort_by_key(|b| b.zlow);

    for i in 0..bricks.len() {
        for j in 0..bricks.len() {
            if bricks[j].zlow > bricks[i].zhigh + 1 {
                break;
            }

            if bricks[i].xhigh < bricks[j].xlow
                || bricks[i].xlow > bricks[j].xhigh
                || bricks[i].yhigh < bricks[j].ylow
                || bricks[i].ylow > bricks[j].yhigh
            {
                // Don't touch horizontally
                continue;
            }

            if bricks[i].zhigh + 1 == bricks[j].zlow {
                bricks[i].supports.push(j);
            }
            if bricks[i].zlow == bricks[j].zhigh + 1 {
                bricks[i].supported_by.push(j);
            }
        }
    }

    bricks
}

#[allow(unused_variables)]
pub fn part1(input: &[Brick]) -> Option<u64> {
    let mut sum = 0;
    for i in 0..input.len() {
        let mut supported_counts = input[i]
            .supports
            .iter()
            .map(|&j| input[j].supported_by.len());
        if supported_counts.all(|c| c > 1) {
            sum += 1;
        }
    }

    Some(sum)
}

#[allow(unused_variables)]
pub fn part2(input: &[Brick]) -> Option<u64> {
    let mut sum = 0;
    for i in 0..input.len() {
        let mut seen = HashSet::new();
        let mut to_process = vec![i];
        seen.insert(i);

        while let Some(idx) = to_process.pop() {
            for &j in &input[idx].supports {
                if input[j].supported_by.iter().all(|k| seen.contains(k)) && seen.insert(j) {
                    to_process.push(j);
                    sum += 1;
                }
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day22/test.txt");
    #[test]
    fn test_day22_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(5));
    }

    #[test]
    fn test_day22_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, Some(7));
    }
}
