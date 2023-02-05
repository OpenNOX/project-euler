#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::iterators::CollatzSequence, Solution};

/// Command-line argument start number threshold type.
type StartNumberThreshold = u64;

/// Command-line argument start number threshold placeholder.
const START_NUMBER_THRESHOLD: &str = "START_NUMBER_THRESHOLD";

struct Solution014;
impl Solution for Solution014 {
    fn title(&self) -> String {
        "Longest Collatz sequence".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Start number of the longest Collatz sequence chain below {}.",
            START_NUMBER_THRESHOLD
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(START_NUMBER_THRESHOLD)
            .help("Start number threshold.")
            .required(true)
            .value_parser(value_parser!(StartNumberThreshold))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let start_number_threshold = *arguments
            .get_one::<StartNumberThreshold>(START_NUMBER_THRESHOLD)
            .expect("command-line arguments parser to get argument");

        fn longest_sequence_length<'a>(
            (_start_number, sequence_length): &'a (u64, usize),
        ) -> usize {
            *sequence_length
        }

        (2..start_number_threshold)
            .into_iter()
            .map(|start_number| (start_number, CollatzSequence::new(start_number).count()))
            .max_by_key(longest_sequence_length)
            .expect("start number threshold to be greater than 2")
            .0
    }
}

fn main() {
    run_solution(&Solution014 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution014;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_014_example() {
        let solution = Solution014 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10"]);

        assert_eq!(solution.run(&arguments), 9);
    }

    #[bench]
    fn bench_solution_014(bencher: &mut Bencher) {
        let solution = Solution014 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "1000000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
