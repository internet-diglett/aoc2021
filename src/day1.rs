use super::PuzzleSolver;

/// Takes a single string that is a series of integers,
/// each separated by a new line, and produces the solutions to the
/// puzzle using the PuzzleSolver trait methods.
///
#[derive(Debug)]
pub struct Solver {
    pub input: Vec<u32>,
}

impl PuzzleSolver for Solver {
    /// Solves part 1 by counting the number of depth measurement increases
    /// ```
    /// use aoc2021::{PuzzleSolver};
    /// use aoc2021::day1::{Solver};
    /// let example = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.part_one(), "7");
    /// ```
    fn part_one(&self) -> String {
        println!("counting the number of times a depth measurement increases");
        count_increases(&self.input).to_string()
    }

    /// Solves part 2 by counting the number of 3 sum increases
    /// ```
    /// use aoc2021::{PuzzleSolver};
    /// use aoc2021::day1::{Solver};
    /// let example = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.part_two(), "5");
    /// ```
    fn part_two(&self) -> String {
        println!("counting the number of times the sum of 3 consecutive measurements increases");
        count_range_increases(&self.input, 3).to_string()
    }
}

impl Solver {
    /// Constructor method for creating a new Solver from the puzzle input
    /// ```
    /// use aoc2021::day1::{Solver};
    /// let example = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    /// let solver = Solver::new(example);
    /// assert_eq!(solver.input, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
    /// ```
    pub fn new(input: &str) -> Solver {
        let integers: Vec<u32>;
        integers = input
            .trim()
            .split('\n')
            .into_iter()
            .map(|x| x.parse().expect("Not a number!"))
            .collect();
        Solver { input: integers }
    }
}

fn count_increases(input: &[u32]) -> u32 {
    let mut counter = 0;
    for pair in input.windows(2) {
        if pair[0] < pair[1] {
            counter += 1
        }
    }
    counter
}

fn count_range_increases(input: &[u32], window_size: usize) -> u32 {
    let sums: Vec<u32> = input
        .windows(window_size)
        .map(|depths| depths.iter().sum())
        .collect();
    count_increases(&sums)
}
