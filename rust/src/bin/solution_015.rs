#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, Solution};

/// Command-line argument range threshold type.
type GridSize = u64;

/// Command-line argument range threshold placeholder.
const GRID_SIZE: &str = "GRID_SIZE";

struct Solution015;
impl Solution for Solution015 {
    fn title(&self) -> String {
        "Lattice paths".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Number of routes through a {} by {} grid.",
            GRID_SIZE, GRID_SIZE
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![Arg::new(GRID_SIZE)
            .help("Width and height of the grid.")
            .required(true)
            .value_parser(value_parser!(GridSize))]
    }

    fn run(&self, arguments: &ArgMatches) -> u64 {
        let grid_size = *arguments
            .get_one::<GridSize>(GRID_SIZE)
            .expect("command-line arguments parser to get argument");

        (0..grid_size).into_iter().fold(1, |mut solution, i| {
            solution *= (2 * grid_size) - i;
            solution /= i + 1;

            return solution;
        })
    }
}

fn main() {
    run_solution(&Solution015 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution015;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_015_example() {
        let solution = Solution015 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "2"]);

        assert_eq!(solution.run(&arguments), 6);
    }

    #[bench]
    fn bench_solution_015(bencher: &mut Bencher) {
        let solution = Solution015 {};

        let arguments = command!()
            .args(solution.arguments())
            .get_matches_from(vec![APPLICATION_NAME, "20"]);

        bencher.iter(|| solution.run(&arguments));
    }
}
