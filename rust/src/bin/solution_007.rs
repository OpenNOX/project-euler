#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::iterators::PrimeNumbers, Solution};

/// Command-line argument prime index type.
type PrimeIndex = usize;

/// Command-line argument prime index placeholder.
const PRIME_INDEX: &str = "PRIME_INDEX";

pub struct Solution007;
impl Solution for Solution007 {
    fn title(&self) -> String {
        "10001st prime".to_string()
    }

    fn description(&self) -> String {
        format!("Get the Nth prime number. Where N is {}.", PRIME_INDEX)
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(PRIME_INDEX)
            .help("Nth prime number to be returned.")
            .required(true)
            .value_parser(value_parser!(PrimeIndex))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let prime_index = *arguments
            .get_one::<PrimeIndex>(PRIME_INDEX)
            .expect("command-line arguments parser to get argument");

        PrimeNumbers::default()
            .into_iter()
            .take(prime_index)
            .last()
            .expect("List has one or more prime numbers.")
    }
}

fn main() {
    run_solution(&Solution007 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution007;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_007_example() {
        let solution = Solution007 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "6"]);

        assert_eq!(solution.run(&arguments), 13);
    }

    #[bench]
    fn bench_solution_007(bencher: &mut Bencher) {
        let solution = Solution007 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10001"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
