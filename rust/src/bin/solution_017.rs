#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use lazy_static::lazy_static;
use project_euler::{run_solution, shared::ToEnglish, Solution};
use regex::Regex;

/// Command-line argument range threshold type.
type RangeThreshold = u64;

/// Command-line argument range threshold placeholder.
const RANGE_THRESHOLD: &str = "RANGE_THRESHOLD";

struct Solution017;
impl Solution for Solution017 {
    fn title(&self) -> String {
        "Number letter counts".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Number of letters when integers from 1 to {} are written out in words.",
            RANGE_THRESHOLD
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(RANGE_THRESHOLD)
            .help("Largest integer to be written out in words.")
            .required(true)
            .value_parser(value_parser!(RangeThreshold))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        lazy_static! {
            static ref ALPHA_REGEX: Regex = Regex::new(r"[a-z]+").expect("RegEx to be valid");
        }

        let range_threshold = *arguments
            .get_one::<RangeThreshold>(RANGE_THRESHOLD)
            .expect("command-line arguments parser to get argument");

        (1..=range_threshold).fold(0, |sum, i| {
            let index_english = i.to_english();

            let letter_count = ALPHA_REGEX
                .captures_iter(&index_english)
                .fold(0, |sum, captures| {
                    sum + captures
                        .iter()
                        .fold(0, |sum, capture| sum + capture.unwrap().as_str().len())
                }) as u64;

            sum + letter_count
        })
    }
}

fn main() {
    run_solution(&Solution017 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution017;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_017_example() {
        let solution = Solution017 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "5"]);

        assert_eq!(solution.run(&arguments), 19);
    }

    #[bench]
    fn bench_solution_017(bencher: &mut Bencher) {
        let solution = Solution017 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "1000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
