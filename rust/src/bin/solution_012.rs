#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::math_helpers::proper_divisors, Solution};

/// Command-line argument divisor count threshold type.
type DivisorCountThreshold = u64;

/// Command-line argument divisor count threshold placeholder.
const DIVISOR_COUNT_THRESHOLD: &str = "DIVISOR_COUNT_THRESHOLD";

pub struct Solution012;
impl Solution for Solution012 {
    fn title(&self) -> String {
        "Highly divisible triangular number".to_string()
    }

    fn description(&self) -> String {
        format!(
            "First triangle number to have over {} divisors.",
            DIVISOR_COUNT_THRESHOLD
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(DIVISOR_COUNT_THRESHOLD)
            .help("Divisor count threshold.")
            .required(true)
            .value_parser(value_parser!(DivisorCountThreshold))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let divisor_count_threshold = *arguments
            .get_one::<DivisorCountThreshold>(DIVISOR_COUNT_THRESHOLD)
            .expect("command-line arguments parser to get argument");

        let mut divisors = vec![];
        let mut solution = 0;

        let mut i = 1;
        while divisors.len() + 1 < divisor_count_threshold as usize {
            solution += i;
            divisors = proper_divisors(solution);

            i += 1;
        }

        solution
    }
}

fn main() {
    run_solution(&Solution012 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution012;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_012_example() {
        let solution = Solution012 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "5"]);

        assert_eq!(solution.run(&arguments), 28);
    }

    #[bench]
    fn bench_solution_012(bencher: &mut Bencher) {
        let solution = Solution012 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "500"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
