#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::file_helpers::read_to_vector, Solution};

/// Command-line argument series file path type.
type SeriesFilePath = String;

/// Command-line argument adjacent digit count type.
type AdjacentDigitCount = u64;

/// Command-line argument series file path placeholder.
const SERIES_FILE_PATH: &str = "SERIES_FILE_PATH";

/// Command-line argument adjacent digit count placeholder.
const ADJACENT_DIGIT_COUNT: &str = "ADJACENT_DIGIT_COUNT";

pub struct Solution008;
impl Solution for Solution008 {
    fn title(&self) -> String {
        "Largest product in a series".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Largest product made up of {} in series located in {}.",
            ADJACENT_DIGIT_COUNT, SERIES_FILE_PATH
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![
            Arg::new(SERIES_FILE_PATH)
                .help("File path to input series.")
                .required(true)
                .value_parser(value_parser!(SeriesFilePath)),
            Arg::new(ADJACENT_DIGIT_COUNT)
                .help("Number of adjacent digits to multiply.")
                .required(true)
                .value_parser(value_parser!(AdjacentDigitCount)),
        ]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let adjacent_digit_count = *arguments
            .get_one::<AdjacentDigitCount>(ADJACENT_DIGIT_COUNT)
            .expect("command-line arguments parser to get argument");
        let series_file_path = arguments
            .get_one::<SeriesFilePath>(SERIES_FILE_PATH)
            .expect("command-line arguments parser to get argument");

        let input_data = read_to_vector::<u64>(&series_file_path, None);
        let mut solution = 0;

        for i in 0..=(input_data.len() as AdjacentDigitCount - adjacent_digit_count) {
            let mut product = 1;

            for offset in 0..adjacent_digit_count {
                product *= input_data[(i + offset) as usize] as u64;
            }

            solution = if product > solution {
                product
            } else {
                solution
            };
        }

        solution
    }
}

fn main() {
    run_solution(&Solution008 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution008;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_008_example() {
        let solution = Solution008 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".test_input/008.dat",
            "2",
        ]);

        assert_eq!(solution.run(&arguments), 72);
    }

    #[bench]
    fn bench_solution_008(bencher: &mut Bencher) {
        let solution = Solution008 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".problem_input/008.dat",
            "13",
        ]);

        bencher.iter(|| solution.run(&arguments));
    }
}
