#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, Solution};

/// Command-line argument range threshold type.
type RangeThreshold = u64;

/// Command-line argument range threshold placeholder.
const RANGE_THRESHOLD: &str = "RANGE_THRESHOLD";

pub struct Solution006;
impl Solution for Solution006 {
    fn title(&self) -> String {
        "Sum square difference".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Difference between sum of squares and square of sums up to {}.",
            RANGE_THRESHOLD
        )
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

        let mut sum_of_squares = 0;
        let mut square_of_sums = 0;

        for i in 1..=range_threshold {
            sum_of_squares += i * i;
            square_of_sums += i;
        }

        (square_of_sums * square_of_sums) - sum_of_squares
    }
}

fn main() {
    run_solution(&Solution006 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution006;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_006_example() {
        let solution = Solution006 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10"]);

        assert_eq!(solution.run(&arguments), 2640);
    }

    #[bench]
    fn bench_solution_006(bencher: &mut Bencher) {
        let solution = Solution006 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "100"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
