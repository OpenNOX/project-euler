#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use num::{BigUint, FromPrimitive};
use project_euler::{run_solution, Solution};

/// Command-line argument range threshold type.
type ExponentPower = u32;

/// Command-line argument range threshold placeholder.
const EXPONENT_POWER: &str = "EXPONENT_POWER";

struct Solution016;
impl Solution for Solution016 {
    fn title(&self) -> String {
        "Power digit sum".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Sum of the digits from the result of 2 raised to the power of {}.",
            EXPONENT_POWER
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(EXPONENT_POWER)
            .help("Power to raise 2 by.")
            .required(true)
            .value_parser(value_parser!(ExponentPower))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let exponent_power = *arguments
            .get_one::<ExponentPower>(EXPONENT_POWER)
            .expect("command-line arguments parser to get argument");

        BigUint::from_i32(2)
            .expect("hard coded integer to be parsable")
            .pow(exponent_power.into())
            .to_string()
            .chars()
            .fold(0, |sum, char| {
                sum + char
                    .to_digit(10)
                    .expect("all characters to be base 10 digits") as u64
            })
    }
}

fn main() {
    run_solution(&Solution016 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution016;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_016_example() {
        let solution = Solution016 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "15"]);

        assert_eq!(solution.run(&arguments), 26);
    }

    #[bench]
    fn bench_solution_016(bencher: &mut Bencher) {
        let solution = Solution016 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "1000"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
