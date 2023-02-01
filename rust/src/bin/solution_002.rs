#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::iterators::FibonacciSequence, Solution};

/// Command-line argument sequence threshold type.
type SequenceThreshold = u64;

/// Command-line argument sequence threshold placeholder.
const SEQUENCE_THRESHOLD: &str = "SEQUENCE_THRESHOLD";

struct Solution002;
impl Solution for Solution002 {
    fn title(&self) -> String {
        "Even Fibonacci numbers".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Sum of even Fibonacci sequence terms that do not exceed {}.",
            SEQUENCE_THRESHOLD
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(SEQUENCE_THRESHOLD)
            .help("Fibonacci sequence term threshold.")
            .required(true)
            .value_parser(value_parser!(SequenceThreshold))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let sequence_threshold = *arguments
            .get_one::<SequenceThreshold>(SEQUENCE_THRESHOLD)
            .expect("command-line arguments parser to get argument");

        let mut solution = 0;

        for term in FibonacciSequence::default().filter(|term| term % 2 == 0) {
            if term >= sequence_threshold {
                break;
            }

            solution += term;
        }

        solution
    }
}

fn main() {
    run_solution(&Solution002 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution002;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_002_example() {
        let solution = Solution002 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "100"]);

        assert_eq!(solution.run(&arguments), 44);
    }

    #[bench]
    fn bench_solution_002(bencher: &mut Bencher) {
        let solution = Solution002 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "4000000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
