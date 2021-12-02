use super::PuzzleSolver;
use regex::Regex;

/// Takes a single string that is a series of movement commands,
/// each separated by a new line, such as:
/// ```text
/// forward 5
/// down 5
/// forward 8
/// up 3
/// down 8
/// forward 2
/// ```
/// and produces the solutions to the
/// puzzle using the PuzzleSolver trait methods.
///
#[derive(Debug)]
pub struct Solver {
    pub input: Vec<Command>,
}

impl PuzzleSolver for Solver {
    /// Solves part 1 by calculating the horizontal and depth position, then multiplying
    /// ```
    /// use aoc2021::{PuzzleSolver};
    /// use aoc2021::day2::{Solver};
    /// let example = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.part_one(), "150");
    /// ```
    fn part_one(&self) -> String {
        println!("Multiplying final horizontal position by final depth");
        let position = calculate_position(&self.input);
        (position.horizontal * position.depth).to_string()
    }

    /// Solves part 2 by calculating the horizontal and depth positions (using trajectory to
    /// influence depth), then multiplying horizontal and depth positions.
    /// ```
    /// use aoc2021::{PuzzleSolver};
    /// use aoc2021::day2::{Solver};
    /// let example = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.part_two(), "900");
    /// ```
    fn part_two(&self) -> String {
        println!("Multiplying final horizontal position by final depth");
        let position = calculate_position_v2(&self.input);
        (position.horizontal * position.depth).to_string()
    }
}

impl Solver {
    /// Constructor method for creating a new Solver from the puzzle input
    /// ```
    /// use aoc2021::day2::{Solver, Command};
    /// let example = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.input[0], Command::Forward(5))
    /// ```
    pub fn new(input: &str) -> Solver {
        let commands: Vec<Command>;
        let re = Regex::new(r"^(forward|up|down) (\d+)$").expect("failed to compile regex");
        commands = input
            .trim()
            .split('\n')
            .into_iter()
            .map(|x| parse_command(&re, x).unwrap())
            .collect();
        Solver { input: commands }
    }
}

fn calculate_position(commands: &[Command]) -> Position {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };
    for command in commands.iter() {
        match command {
            Command::Forward(x) => position.horizontal += x,
            Command::Up(x) => position.depth -= x,
            Command::Down(x) => position.depth += x,
        }
    }
    position
}

fn calculate_position_v2(commands: &[Command]) -> Position {
    let mut aim: usize = 0;
    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };
    for command in commands.iter() {
        match command {
            Command::Forward(x) => {
                position.horizontal += x;
                position.depth += aim * x;
            }
            Command::Up(x) => aim -= x,
            Command::Down(x) => aim += x,
        }
    }
    position
}

fn parse_command(regex: &Regex, command: &str) -> Option<Command> {
    let caps = regex.captures(command).expect("Not a valid command!");
    let direction = caps.get(1)?.as_str();
    let distance = caps.get(2)?.as_str();
    let distance: usize = distance.parse().ok()?;
    match direction {
        "forward" => Some(Command::Forward(distance)),
        "down" => Some(Command::Down(distance)),
        "up" => Some(Command::Up(distance)),
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: usize,
    depth: usize,
}

/// Variations of commands used to move submarine
#[derive(Debug, PartialEq)]
pub enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}
