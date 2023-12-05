#[derive(Debug)]
pub struct Range {
    start: u64,
    end: u64,
}

pub struct MapRange {
    src_start: u64,
    dst_start: u64,
    length: u64,
}

pub struct Input {
    seeds: Vec<u64>,
    maps: Vec<Vec<MapRange>>,
}

// Assume the map ranges are sorted
fn map_range(src_range: &Range, map_ranges: &[MapRange]) -> Vec<Range> {
    let mut curr = src_range.start;
    let end = src_range.end;
    let mut out_ranges = Vec::new();

    let mut map_it = map_ranges.iter();
    let mut map = map_it.next();
    while curr < end {
        // MapRange is beyond our source range, no more mapping needed
        if map.is_none() || end < map.unwrap().src_start {
            out_ranges.push(Range { start: curr, end });
            break;
        }
        let map_range = map.unwrap();

        // Our current pointer is beyond this map, iterate the map
        if curr > map_range.src_start + map_range.length {
            map = map_it.next();
            continue;
        }

        if curr < map_range.src_start {
            out_ranges.push(Range {
                start: curr,
                end: map_range.src_start,
            });
            curr = map_range.src_start;
        }
        let offset = curr - map_range.src_start;
        if map_range.src_start + map_range.length < end {
            let new_range = Range {
                start: map_range.dst_start + offset,
                end: map_range.dst_start + map_range.length,
            };
            out_ranges.push(new_range);
        } else {
            let dist = end - map_range.src_start;
            let new_range = Range {
                start: map_range.dst_start + offset,
                end: map_range.dst_start + dist,
            };
            out_ranges.push(new_range);
        }
        curr = map_range.src_start + map_range.length;
        map = map_it.next();
    }

    out_ranges
}

pub fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let first = lines.next().unwrap().strip_prefix("seeds: ").unwrap();
    let seeds = first.split(' ').map(|s| s.parse().unwrap()).collect();

    let mut curr_map = Vec::new();
    let mut maps = Vec::new();
    let lines = lines.skip(2);
    for line in lines {
        if line.is_empty() {
            curr_map.sort_by_key(|r: &MapRange| r.src_start);
            maps.push(curr_map);
            curr_map = Vec::new();
            continue;
        }
        if line.ends_with(':') {
            continue;
        }

        let mut parsed = line.split_whitespace().map(|n| n.parse().unwrap());
        let range = MapRange {
            dst_start: parsed.next().unwrap(),
            src_start: parsed.next().unwrap(),
            length: parsed.next().unwrap(),
        };
        curr_map.push(range);
    }

    Input { seeds, maps }
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mapped_nums = input.seeds.iter().map(|&src| {
        let mut s = src;
        for map in &input.maps {
            for map_range in map {
                if map_range.src_start <= s && s < map_range.src_start + map_range.length {
                    let dst = s - map_range.src_start + map_range.dst_start;
                    s = dst;
                    break;
                }
            }
        }
        s
    });

    mapped_nums.min()
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    let mut ranges = Vec::new();
    let mut i = 0;
    while i < input.seeds.len() {
        let (start, length) = (input.seeds[i], input.seeds[i + 1]);
        ranges.push(Range {
            start,
            end: start + length,
        });
        i += 2;
    }

    for map in &input.maps {
        let mut new_ranges = Vec::new();
        for range in &ranges {
            let mut mapped_ranges = map_range(range, map);
            new_ranges.append(&mut mapped_ranges);
        }
        ranges = new_ranges;
    }

    ranges.iter().map(|r| r.start).min()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day05/test.txt");
    #[test]
    fn test_day05_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(35));
    }

    #[test]
    fn test_day05_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
