#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::string_helpers::is_palindrome, Solution};

/// Command-line argument digit count type.
type DigitCount = usize;

/// Command-line argument digit count placeholder.
const DIGIT_COUNT: &str = "DIGIT_COUNT";

pub struct Solution004;
impl Solution for Solution004 {
    fn title(&self) -> String {
        "Largest palindrome product".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Largest palindrome product from two numbers of {}.",
            DIGIT_COUNT
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(DIGIT_COUNT)
            .help("Number of digits that make up the two numbers to multiply.")
            .required(true)
            .value_parser(value_parser!(DigitCount))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let digit_count = *arguments
            .get_one::<DigitCount>(DIGIT_COUNT)
            .expect("command-line arguments parser to get argument");

        let start: u64 = format!("{:0<digit_count$}", "1")
            .parse()
            .expect("one with zero padding appended to be parsable");
        let end = (start * 10) - 1;
        let mut solution = 0;

        for a in start..=end {
            for b in start..=end {
                let product = a * b;

                if is_palindrome(product.to_string().as_str()) && product > solution {
                    solution = product;
                }
            }
        }

        solution
    }
}

fn main() {
    run_solution(&Solution004 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution004;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_004_example() {
        let solution = Solution004 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "2"]);

        assert_eq!(solution.run(&arguments), 9009);
    }

    #[bench]
    fn bench_solution_004(bencher: &mut Bencher) {
        let solution = Solution004 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "3"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
