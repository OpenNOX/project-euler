#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::math_helpers::prime_factors, Solution};

/// Command-line argument input number type.
type InputNumber = u64;

/// Command-line argument input number placeholder.
const INPUT_NUMBER: &str = "INPUT_NUMBER";

pub struct Solution003;
impl Solution for Solution003 {
    fn title(&self) -> String {
        "Largest prime factor".to_string()
    }

    fn description(&self) -> String {
        format!("Largest prime factor of {}.", INPUT_NUMBER)
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(INPUT_NUMBER)
            .help("Input number.")
            .required(true)
            .value_parser(value_parser!(InputNumber))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let input_number = *arguments
            .get_one::<InputNumber>(INPUT_NUMBER)
            .expect("command-line arguments parser to get argument");

        prime_factors(input_number)
            .last()
            .expect("input number to have one or more prime factors")
            .base_value
    }
}

fn main() {
    run_solution(&Solution003 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution003;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_003_example() {
        let solution = Solution003 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "13195"]);

        assert_eq!(solution.run(&arguments), 29);
    }

    #[bench]
    fn bench_solution_003(bencher: &mut Bencher) {
        let solution = Solution003 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "600851475143"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
