pub mod days;
pub mod solution;
pub mod utils;

use clap::Parser;
use solution::{print_table, Solution};

#[derive(Parser, Debug)]
struct Args {
    day: Option<u32>,
}

fn main() {
    let solutions = solution::all_solutions();
    let mut all_results = Vec::new();
    for Solution { day, input, func } in solutions {
        let results = (func)(input);
        all_results.push(results);
    }

    print_table(all_results);
}
