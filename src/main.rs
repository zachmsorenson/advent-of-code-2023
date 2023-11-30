pub mod days;
pub mod solution;
pub mod utils;

use clap::Parser;
use solution::{Results, Solution};
use std::time::{Duration, Instant};

#[derive(Parser, Debug)]
struct Args {
    day: Option<u32>,
}

fn all_solutions() -> Vec<Solution> {
    vec![
        solution!(day01),
        // solution!(day02),
        // solution!(day03),
        // solution!(day04),
        // solution!(day05),
        // solution!(day06),
        // solution!(day07),
        // solution!(day08),
        // solution!(day09),
    ]
}

fn main() {
    let solutions = all_solutions();
    for Solution { day, input, func } in solutions {
        let results = (func)(input);
        println!(
            "Day: {}, Part1: {:?}, Part2: {:?}, Times: {:?} {:?} {:?}",
            day,
            results.part1_output,
            results.part2_output,
            results.parse_time,
            results.part1_time,
            results.part2_time
        )
    }
}
