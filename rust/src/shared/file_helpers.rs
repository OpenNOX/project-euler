use std::{fmt::Debug, fs, str::FromStr};

/// Create vector from input file.
/// - **file_path:** Input file path.
/// - **delimiter:** If None separate file by character, ignoring newlines, otherwise split file by
///   specified delimiter pattern.
pub fn read_to_vector<T>(file_path: &str, delimiter: Option<&str>) -> Vec<T>
where
    T: FromStr + From<u32>,
    <T as FromStr>::Err: Debug,
{
    let file = fs::read_to_string(file_path).expect("file to exist and be readable");

    match delimiter {
        Some(delimiter) => file
            .split(delimiter)
            .map(|i| {
                i.replace("\n", "")
                    .parse()
                    .expect("all words to be integers")
            })
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
pub fn read_to_grid(file_path: &str) -> Vec<Vec<u64>> {
    fs::read_to_string(file_path)
        .expect("file to exist and be readable")
        .split('\n')
        .map(|line| {
            line.split(' ')
                .map(|cell| cell.parse().expect("all cells to be integers"))
                .collect()
        })
        .collect()
}
