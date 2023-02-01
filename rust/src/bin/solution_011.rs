#![feature(test)]

use clap::{value_parser, Arg, ArgMatches};
use project_euler::{run_solution, shared::file_helpers::read_to_grid, Solution};

/// Command-line argument grid file path type.
type GridFilePath = String;

/// Command-line argument adjacent digit count type.
type AdjacentDigitCount = usize;

/// Command-line argument grid file path placeholder.
const GRID_FILE_PATH: &str = "GRID_FILE_PATH";

/// Command-line argument adjacent digit count placeholder.
const ADJACENT_DIGIT_COUNT: &str = "ADJACENT_DIGIT_COUNT";

pub struct Solution011;
impl Solution for Solution011 {
    fn title(&self) -> String {
        "Largest product in a grid".to_string()
    }

    fn description(&self) -> String {
        format!(
            "Largest product made up of {} in same direction in the grid located in {}.",
            ADJACENT_DIGIT_COUNT, GRID_FILE_PATH
        )
    }

    fn arguments(&self) -> Vec<Arg> {
        vec![
            Arg::new(GRID_FILE_PATH)
                .help("File path to input grid.")
                .required(true)
                .value_parser(value_parser!(GridFilePath)),
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
        let grid_file_path = arguments
            .get_one::<GridFilePath>(GRID_FILE_PATH)
            .expect("command-line arguments parser to get argument");

        let input_data = read_to_grid(grid_file_path);
        let mut solution = 0;

        for y in 0..input_data.len() {
            for x in 0..input_data[y].len() {
                let mut products: [u64; 4] = [1, 1, 1, 1];
                let y_in_bounds = y + adjacent_digit_count <= input_data.len();
                let x_in_bounds = x + adjacent_digit_count <= input_data[y].len();
                let reverse_diagonal_x_in_bounds =
                    (x as i32 - (adjacent_digit_count as i32 - 1)) as i32 >= 0;

                for y_offset in 0..adjacent_digit_count {
                    for x_offset in 0..adjacent_digit_count {
                        let offset_y = y + y_offset;
                        let offset_x = x + x_offset;
                        let is_diagonal = x_offset == y_offset;

                        if y_in_bounds && x_offset == 0 {
                            // Horizontal product
                            products[0] *= input_data[offset_y][offset_x] as u64;
                        }

                        if x_in_bounds && y_offset == 0 {
                            // Vertical product
                            products[1] *= input_data[offset_y][offset_x] as u64;
                        }

                        if y_in_bounds && x_in_bounds && is_diagonal {
                            // Left-to-right diagonal product
                            products[2] *= input_data[offset_y][offset_x] as u64;
                        }

                        if y_in_bounds && reverse_diagonal_x_in_bounds && is_diagonal {
                            // Reverse diagonal product
                            products[3] *= input_data[offset_y][offset_x - (x_offset * 2)] as u64;
                        }
                    }
                }

                products.sort();

                if products[3] > solution {
                    solution = products[3]
                }
            }
        }

        solution
    }
}

fn main() {
    run_solution(&Solution011 {});
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::Solution011;
    use clap::command;
    use project_euler::Solution;
    use test::Bencher;

    const APPLICATION_NAME: &str = "test_app";

    #[test]
    fn solves_problem_011_example() {
        let solution = Solution011 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".test_input/011.dat",
            "4",
        ]);

        assert_eq!(solution.run(&arguments), 1788696);
    }

    #[bench]
    fn bench_solution_011(bencher: &mut Bencher) {
        let solution = Solution011 {};

        let arguments = command!().args(solution.arguments()).get_matches_from(vec![
            APPLICATION_NAME,
            ".problem_input/011.dat",
            "4",
        ]);

        bencher.iter(|| solution.run(&arguments));
    }
}
