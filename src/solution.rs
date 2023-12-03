use crate::days;
use prettytable::{row, Row, Table};
use std::time::{Duration, Instant};

pub struct Results {
    pub day: &'static str,
    pub part1_output: Option<u32>,
    pub part2_output: Option<u32>,
    pub parse_time: Duration,
    pub part1_time: Duration,
    pub part2_time: Duration,
    pub total_time: Duration,
}

impl Results {
    pub fn as_row(&self) -> Row {
        row![
            self.day,
            self.part1_output.unwrap_or(0),
            self.part2_output.unwrap_or(0),
            format!("{:?}", self.parse_time),
            format!("{:?}", self.part1_time),
            format!("{:?}", self.part2_time),
            format!("{:?}", self.total_time),
        ]
    }
}

pub fn print_table(results: Vec<Results>) {
    let mut table = Table::new();

    table.set_titles(row![
        "Day",
        "Part1 Output",
        "Part2 Output",
        "Parse Time",
        "Part1 Time",
        "Part2 Time",
        "Total Time"
    ]);
    for r in results {
        table.add_row(r.as_row());
    }

    table.printstd();
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
                    day: stringify!($day),
                    part1_output,
                    part2_output,
                    parse_time: t[1] - t[0],
                    part1_time: t[2] - t[1],
                    part2_time: t[3] - t[2],
                    total_time: t[3] - t[0],
                }
            },
        }
    };
}

pub fn all_solutions() -> Vec<Solution> {
    vec![
        solution!(day01),
        solution!(day02),
        solution!(day03),
        // solution!(day04),
        // solution!(day05),
        // solution!(day06),
        // solution!(day07),
        // solution!(day08),
        // solution!(day09),
    ]
}
