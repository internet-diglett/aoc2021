use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use std::fs;

fn main() {
    include_str!("../Cargo.toml");
    // Process args
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Plaintext file containing puzzle input. Defaults to day<N>.txt"),
        )
        .arg(
            Arg::with_name("day")
                .short("d")
                .long("day")
                .takes_value(true)
                .help("Day of puzzle you want to solve"),
        )
        .get_matches();

    // Read and format data from puzzle input file
    let day = matches
        .value_of("day")
        // TODO Print error message and exit instead of panicking
        .expect("You must specify the day of the puzzle!");
    let default = format!("day{}.txt", day);
    let puzzle_input_file = matches.value_of("file").unwrap_or(&default);
    // TODO Print error message and exit instead of panicking
    let input = fs::read_to_string(puzzle_input_file).expect("Error while reading file");

    // Solve requested puzzle
    // TODO Print error message and exit instead of panicking
    let solver = aoc2021::new_solver(day, &input).expect("Not a valid day!");
    solver.solve();
}
