#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, Solution};

/// Command-line argument range threshold type.
type RangeThreshold = u64;

/// Command-line argument range threshold placeholder.
const RANGE_THRESHOLD: &str = "RANGE_THRESHOLD";

struct Solution001;
impl Solution for Solution001 {
    fn title(&self) -> String {
        "Multiples of 3 or 5".to_string()
    }

    fn description(&self) -> String {
        format!("Sum of multiples of 3 or 5 below {}.", RANGE_THRESHOLD)
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(RANGE_THRESHOLD)
            .help("Range threshold.")
            .required(true)
            .value_parser(value_parser!(RangeThreshold))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let range_threshold = *arguments
            .get_one::<RangeThreshold>(RANGE_THRESHOLD)
            .expect("command-line arguments parser to get argument");

        (0..range_threshold)
            .into_iter()
            .filter(|n| n % 3 == 0 || n % 5 == 0)
            .sum()
    }
}

fn main() {
    run_solution(&Solution001 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution001;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_001_example() {
        let solution = Solution001 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10"]);

        assert_eq!(solution.run(&arguments), 23);
    }

    #[bench]
    fn bench_solution_001(bencher: &mut Bencher) {
        let solution = Solution001 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "1000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
