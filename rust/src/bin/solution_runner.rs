use clap::{command, value_parser, Arg};
use project_euler::ProblemIndex;
use std::{
    env,
    io::{self, Write},
    process::{self, Command},
};

/// Command-line argument problem index placeholder.
const PROBLEM_INDEX: &str = "PROBLEM_INDEX";

fn main() {
    println!();

    let solution_index = get_solution_index();

    let solution_path = match env::current_exe() {
        Ok(solution_runner_exe_path) => solution_runner_exe_path
            .parent()
            .expect("solution runner executable to exist in a directory")
            .join(format!("solution_{:03}.exe", solution_index)),
        Err(err) => {
            println!("Failed to get current exe path: {}\n", err);
            return;
        }
    };

    if !solution_path.exists() {
        println!(
            "No solution for problem {} has been implemented.\n",
            solution_index,
        );
        return;
    }

    let solution_arguments = env::args().skip(2).collect::<Vec<String>>().join(" ");

    let solution_executable = Command::new(&solution_path)
        .arg(solution_arguments)
        .output()
        .expect("solution to problem to be run");
    io::stdout()
        .write_all(&solution_executable.stdout)
        .expect("solution to run without interruption");
    io::stderr()
        .write_all(&solution_executable.stderr)
        .expect("solution to run without interruption");

    println!();
}

/// Get solution index from command-line arguments.
fn get_solution_index() -> ProblemIndex {
    let mut command = command!()
        .disable_help_flag(true)
        .disable_version_flag(true)
        .ignore_errors(true)
        .allow_external_subcommands(true)
        .about("Solution runner for Project Euler problems.")
        .arg(
            Arg::new(PROBLEM_INDEX)
                .required(true)
                .value_parser(value_parser!(ProblemIndex))
                .help("Problem number to run solution for."),
        );

    let arg_matches = command.get_matches_mut();
    match arg_matches.get_one::<ProblemIndex>(PROBLEM_INDEX) {
        Some(problem_index) => *problem_index,
        None => {
            command
                .print_long_help()
                .expect("help message to be printed");

            println!();

            process::exit(0);
        }
    }
}
