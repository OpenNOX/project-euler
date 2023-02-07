#![feature(is_some_and)]
#![feature(iter_advance_by)]
#![feature(test)]

pub mod shared;

use clap::{command, Arg, ArgMatches};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;

/// Command-line argument problem index type.
pub type ProblemIndex = u16;

/// Represents a Project Euler problem solution.
pub trait Solution {
    /// Get solution title.
    fn title(&self) -> String;

    /// Get solution description.
    fn description(&self) -> String;

    /// Get solution input arguments.
    fn arguments(&self) -> Vec<Arg>;

    /// Run solution.
    fn run(&self, arguments: &ArgMatches) -> u64;
}

/// Run the passed in solution.
/// - **solution:** Solution to be run.
pub fn run_solution(solution: &dyn Solution) {
    lazy_static! {
        static ref SOLUTION_INDEX_REGEX: Regex =
            Regex::new(r"solution_(\d{3}).exe$").expect("RegEx to be valid");
    }

    let solution_path = match env::current_exe() {
        Ok(exe_path) => exe_path.display().to_string(),
        Err(err) => {
            println!("Failed to get current exe path: {}\n", err);
            return;
        }
    };

    let solution_index = match SOLUTION_INDEX_REGEX.captures(&solution_path) {
        Some(captures) => captures[1]
            .parse::<ProblemIndex>()
            .expect("RegEx to only capture digits"),
        None => {
            println!("Failed to get solution exe path in: {}\n", solution_path);
            return;
        }
    };

    let arguments = command!()
        .disable_version_flag(true)
        .before_help(solution.title())
        .about(solution.description())
        .args(solution.arguments())
        .get_matches();

    println!("Running problem {}'s solution...\n", solution_index);

    println!("Answer: {}", solution.run(&arguments));
}
