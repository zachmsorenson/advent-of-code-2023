use crate::days;

use std::time::{Duration, Instant};

pub struct Results {
    pub part1_output: Option<u32>,
    pub part2_output: Option<u32>,
    pub parse_time: Duration,
    pub part1_time: Duration,
    pub part2_time: Duration,
}

pub struct Solution {
    pub day: &'static str,
    pub input: &'static str,
    pub func: fn(&str) -> Results,
}

#[macro_export]
macro_rules! solution {
    ($day:tt) => {
        Solution {
            day: stringify!($day),
            input: include_str!(concat!("../input/", stringify!($day), "/input.txt")),
            func: |data: &str| {
                let mut t = [Instant::now(); 4];
                t[0] = Instant::now();
                let input = days::$day::parse_input(data);
                t[1] = Instant::now();
                let part1_output = days::$day::part1(&input);
                t[2] = Instant::now();
                let part2_output = days::$day::part2(&input);
                t[3] = Instant::now();

                Results {
                    part1_output,
                    part2_output,
                    parse_time: t[1] - t[0],
                    part1_time: t[2] - t[1],
                    part2_time: t[3] - t[2],
                }
            },
        }
    };
}
