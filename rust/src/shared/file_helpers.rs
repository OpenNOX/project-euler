use std::{fmt::Debug, fs, str::FromStr};

/// Create vector from input file.
/// - **file_path:** Input file path.
/// - **delimiter:** If None separate file by character, otherwise split file by specified delimiter
///   pattern.
/// ## Notes
/// - Newline characters are ignored.
pub fn read_to_vector<T>(file_path: &str, delimiter: Option<&char>) -> Vec<T>
where
    T: FromStr + From<u32>,
    <T as FromStr>::Err: Debug,
{
    let file = fs::read_to_string(file_path)
        .expect("file to exist and be readable")
        .replace('\r', "");
    let file = file.trim_end_matches('\n');

    match delimiter {
        Some(delimiter) => file
            .replace('\n', &delimiter.to_string())
            .split(*delimiter)
            .map(|i| i.parse().expect("all words to be integers"))
            .collect(),
        None => file
            .chars()
            .filter(|char| *char != '\n')
            .map(|char| {
                char.to_digit(10)
                    .expect("all characters to be base 10 digits")
                    .into()
            })
            .collect(),
    }
}

/// Create grid (2D vector) from input file.
/// - **file_path:** Input file path.
/// - **delimiter:** If None separate file by character, otherwise split file by specified delimiter
///   pattern.
/// ## Notes
/// - Newline and end-of-file characters are treated as row terminators.
pub fn read_to_grid(file_path: &str, delimiter: Option<&char>) -> Vec<Vec<u64>> {
    let file = fs::read_to_string(file_path)
        .expect("file to exist and be readable")
        .replace('\r', "");
    let file = file.trim_end_matches('\n').split('\n');

    match delimiter {
        Some(delimiter) => file
            .map(|line| {
                line.split(*delimiter)
                    .map(|i| i.parse().expect("all cells to be integers"))
                    .collect()
            })
            .collect(),
        None => file
            .map(|line| {
                line.chars()
                    .map(|char| {
                        char.to_digit(10)
                            .expect("all characters to be base 10 digits")
                            .into()
                    })
                    .collect()
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    extern crate test;

    use super::{read_to_grid, read_to_vector};
    use test::Bencher;

    static EXPECTED_RESULTS: &[[u64; 4]; 4] =
        &[[4, 6, 2, 3], [1, 5, 0, 4], [6, 3, 1, 9], [5, 0, 2, 8]];

    static INPUT_FILE_PATH: &str = ".test_input/file_helpers_4x4_grid.dat";
    static INPUT_FILE_PATH_DELIMITED: &str = ".test_input/file_helpers_4x4_grid_delimited.dat";
    static INPUT_FILE_DELIMITER: char = '|';

    #[test]
    fn read_to_vector_returns_vector_of_parsed_file_contents() {
        assert_eq!(
            read_to_vector::<u64>(INPUT_FILE_PATH, None),
            EXPECTED_RESULTS
                .iter()
                .flatten()
                .map(|num| *num)
                .collect::<Vec<u64>>()
        );
    }

    #[test]
    fn read_to_vector_returns_vector_of_parsed_delimited_file_contents() {
        assert_eq!(
            read_to_vector::<u64>(INPUT_FILE_PATH_DELIMITED, Some(&INPUT_FILE_DELIMITER)),
            EXPECTED_RESULTS
                .iter()
                .flatten()
                .map(|num| *num)
                .collect::<Vec<u64>>()
        );
    }

    #[test]
    fn read_to_grid_returns_2d_vector_of_parsed_file_contents() {
        assert_eq!(read_to_grid(INPUT_FILE_PATH, None), EXPECTED_RESULTS);
    }

    #[test]
    fn read_to_grid_returns_2d_vector_of_parsed_delimited_file_contents() {
        assert_eq!(
            read_to_grid(INPUT_FILE_PATH_DELIMITED, Some(&INPUT_FILE_DELIMITER)),
            EXPECTED_RESULTS
        );
    }

    #[bench]
    fn bench_read_to_vector_using_a_delimiter(bencher: &mut Bencher) {
        bencher
            .iter(|| read_to_vector::<u64>(INPUT_FILE_PATH_DELIMITED, Some(&INPUT_FILE_DELIMITER)));
    }

    #[bench]
    fn bench_read_to_grid_using_a_delimiter(bencher: &mut Bencher) {
        bencher.iter(|| read_to_grid(INPUT_FILE_PATH_DELIMITED, Some(&INPUT_FILE_DELIMITER)));
    }
}
