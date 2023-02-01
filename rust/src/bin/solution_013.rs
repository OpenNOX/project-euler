#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use num::BigUint;
use project_euler::{run_solution, shared::file_helpers::read_to_vector, Solution};

/// Command-line argument file path type.
type FilePath = String;

/// Command-line argument digit count type.
type DigitCount = usize;

/// Command-line argument file path placeholder.
const FILE_PATH: &str = "FILE_PATH";

/// Command-line argument digit count placeholder.
const DIGIT_COUNT: &str = "DIGIT_COUNT";

pub struct Solution013;
impl Solution for Solution013 {
    fn title(&self) -> String {
        "Large sum".to_string()
    }

    fn description(&self) -> String {
        format!(
            "First {} digits of the sum of numbers located in {}.",
            DIGIT_COUNT, FILE_PATH
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![
            Arg::new(FILE_PATH)
                .help("File path to input series.")
                .required(true)
                .value_parser(value_parser!(FilePath)),
            Arg::new(DIGIT_COUNT)
                .help("Number of digits.")
                .required(true)
                .value_parser(value_parser!(DigitCount)),
        ]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let file_path = arguments
            .get_one::<FilePath>(FILE_PATH)
            .expect("command-line arguments parser to get argument");
        let digit_count = arguments
            .get_one::<DigitCount>(DIGIT_COUNT)
            .expect("command-line arguments parser to get argument");

        read_to_vector::<BigUint>(file_path, Some("\n"))
            .iter()
            .sum::<BigUint>()
            .to_string()[0..*digit_count]
            .parse()
            .expect("first digits of sum to be parsable")
    }
}

fn main() {
    run_solution(&Solution013 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution013;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_013_example() {
        let solution = Solution013 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".test_input/013.dat",
            "2",
        ]);

        assert_eq!(solution.run(&arguments), 27);
    }

    #[bench]
    fn bench_solution_013(bencher: &mut Bencher) {
        let solution = Solution013 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".problem_input/013.dat",
            "10",
        ]);

        bencher.iter(|| solution.run(&arguments));
    }
}
