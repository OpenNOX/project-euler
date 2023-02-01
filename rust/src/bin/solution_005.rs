#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, Solution};

/// Command-line argument maximum multiple type.
type MaximumMultiple = u64;

/// Command-line argument maximum multiple placeholder.
const MAXIMUM_MULTIPLE: &str = "MAXIMUM_MULTIPLE";

pub struct Solution005;
impl Solution for Solution005 {
    fn title(&self) -> String {
        "Smallest multiple".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Smallest number that is evenly divisible by all numbers from 1 to {}.",
            MAXIMUM_MULTIPLE
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(MAXIMUM_MULTIPLE)
            .help("Maximum multiple to be checked.")
            .required(true)
            .value_parser(value_parser!(MaximumMultiple))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let maximum_multiple = *arguments
            .get_one::<MaximumMultiple>(MAXIMUM_MULTIPLE)
            .expect("command-line arguments parser to get argument");

        let mut solution = maximum_multiple - 2;
        let mut solution_found = false;

        while !solution_found {
            for i in 3..=maximum_multiple {
                if solution % i != 0 {
                    break;
                } else if i == maximum_multiple {
                    solution_found = true;
                }
            }

            if !solution_found {
                solution += 1;
            }
        }

        solution
    }
}

fn main() {
    run_solution(&Solution005 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution005;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_005_example() {
        let solution = Solution005 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "10"]);

        assert_eq!(solution.run(&arguments), 2520);
    }

    #[bench]
    fn bench_solution_005(bencher: &mut Bencher) {
        let solution = Solution005 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "20"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
