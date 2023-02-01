#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, Solution};

/// Command-line argument target triplet sum type.
type TargetTripletSum = u64;

/// Command-line argument target triplet sum placeholder.
const TARGET_TRIPLET_SUM: &str = "TARGET_TRIPLET_SUM";

pub struct Solution009;
impl Solution for Solution009 {
    fn title(&self) -> String {
        "Special Pythagorean triplet".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Product of a * b * c when a + b + c equals {}.",
            TARGET_TRIPLET_SUM
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(TARGET_TRIPLET_SUM)
            .help("Target pythagorean triplet sum.")
            .required(true)
            .value_parser(value_parser!(TargetTripletSum))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let target_triplet_sum = *arguments
            .get_one::<TargetTripletSum>(TARGET_TRIPLET_SUM)
            .expect("command-line arguments parser to get argument");

        let mut solution = 0;

        'outer: for c in 3..target_triplet_sum {
            for b in 2..target_triplet_sum {
                for a in 1..target_triplet_sum {
                    if (a * a) + (b * b) == (c * c) && a + b + c == target_triplet_sum {
                        solution = a * b * c;
                        break 'outer;
                    }
                }
            }
        }

        solution
    }
}

fn main() {
    run_solution(&Solution009 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution009;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_009_example() {
        let solution = Solution009 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "12"]);

        assert_eq!(solution.run(&arguments), 60);
    }

    #[bench]
    fn bench_solution_009(bencher: &mut Bencher) {
        let solution = Solution009 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "1000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
