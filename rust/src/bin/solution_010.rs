#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::iterators::PrimeNumbers, Solution};

/// Command-line argument range threshold type.
type RangeThreshold = u32;

/// Command-line argument range threshold placeholder.
const RANGE_THRESHOLD: &str = "RANGE_THRESHOLD";

pub struct Solution010;
impl Solution for Solution010 {
    fn title(&self) -> String {
        "Smallest multiple".to_string()
    }

    fn description(&self) -> String {
        format!("Sum of primes below {}.", RANGE_THRESHOLD)
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

        let mut solution = 0;

        for prime_number in PrimeNumbers::default() {
            if prime_number >= range_threshold.into() {
                break;
            }

            solution += prime_number;
        }

        solution
    }
}

fn main() {
    run_solution(&Solution010 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution010;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_010_example() {
        let solution = Solution010 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10"]);

        assert_eq!(solution.run(&arguments), 17);
    }

    #[bench]
    fn bench_solution_010(bencher: &mut Bencher) {
        let solution = Solution010 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "2000000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
