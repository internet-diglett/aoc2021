use super::PuzzleSolver;

/// Takes a single string that is a series of strings containing a binary value,
/// each separated by a new line, such as:
/// ```text
/// 00100
/// 11110
/// 10110
/// 10111
/// 10101
/// 01111
/// 00111
/// 11100
/// 10000
/// 11001
/// 00010
/// 01010
/// ```
/// and produces the solutions to the
/// puzzle using the PuzzleSolver trait methods.
///
#[derive(Debug)]
pub struct Solver {
    pub input: Vec<String>,
}

impl Solver {
    /// Constructor method for creating a new Solver from the puzzle input
    /// ```
    /// use std::fs;
    /// use aoc2021::day3::{Solver};
    /// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
    /// let solver = Solver::new(&example);
    /// assert_eq!(solver.input[0], "00100")
    /// ```
    pub fn new(input: &str) -> Solver {
        let input = input
            .trim()
            .split('\n')
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        Solver { input }
    }
}

impl PuzzleSolver for Solver {
    /// Solves part 1 by multiplying the gamma rate by the epsilon rate
    /// ```
    /// use std::fs;
    /// use aoc2021::day3::{Solver};
    /// use aoc2021::{PuzzleSolver};
    /// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
    /// let solver = Solver::new(&example);
    /// assert_eq!(solver.part_one(), "198");
    /// ```
    fn part_one(&self) -> String {
        println!("Multiplying gamma rate by epsilon rate");
        let bit_counts = count_bits(&self.input);
        let gamma_rate = generate_gamma_rate(&bit_counts, self.input.len());
        let epsilon_rate = generate_epsilon_rate(&bit_counts, self.input.len());
        (gamma_rate * epsilon_rate).to_string()
    }

    /// Solves part 2 by multiplying the oxygen generator rating
    /// by the CO2 scrubber rating
    /// ```
    /// use std::fs;
    /// use aoc2021::day3::{Solver};
    /// use aoc2021::{PuzzleSolver};
    /// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
    /// let solver = Solver::new(&example);
    /// assert_eq!(solver.part_two(), "230");
    /// ```
    fn part_two(&self) -> String {
        println!("Multiplying O2 rate by CO2 rate");
        let o2_rating = find_oxygen_rating(&self.input);
        let co2_rating = find_co2_rating(&self.input);
        (o2_rating * co2_rating).to_string()
    }
}

/// Counts the number of bits set to 1 for each position
/// Returns vector counting the number of 1s in each position
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let solver = Solver::new(&example);
/// assert_eq!(count_bits(&solver.input), vec![7,5,8,7,5]);
/// ```
pub fn count_bits(binary_data: &[String]) -> Vec<usize> {
    let mut counters = vec![0; binary_data[0].len()];
    for entry in binary_data.iter() {
        for (idx, bit) in entry.chars().enumerate() {
            if bit == '1' {
                counters[idx] += 1;
            }
        }
    }
    counters
}

/// Sorts collection into two separate collections based on the bit value of a given position
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let solver = Solver::new(&example);
/// assert_eq!(sort_by_bit_position(&solver.input, 0).0[0], "11110");
/// ```
pub fn sort_by_bit_position(binary_data: &[String], pos: usize) -> (Vec<String>, Vec<String>) {
    let mut ones: Vec<String> = vec![];
    let mut zeros: Vec<String> = vec![];
    for entry in binary_data.iter() {
        if entry.chars().nth(pos).expect("Unable to index string") == '1' {
            ones.push(entry.to_owned())
        } else {
            zeros.push(entry.to_owned())
        }
    }
    (ones, zeros)
}

/// Finds the O2 rating by reducing the list of binary numbers based on most common
/// bit value per position
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let solver = Solver::new(&example);
/// assert_eq!(find_oxygen_rating(&solver.input), 23);
/// ```
pub fn find_oxygen_rating(binary_data: &[String]) -> usize {
    let mut remaining_numbers = binary_data.to_owned();
    // iterate through each digit of the binary string
    for i in 0..binary_data[0].len() {
        if remaining_numbers.len() == 1 {
            break;
        }
        let (ones, zeros) = sort_by_bit_position(&remaining_numbers, i);
        // reject smaller collection, select ones if tied
        if ones.len() >= zeros.len() {
            remaining_numbers = ones;
        } else {
            remaining_numbers = zeros;
        }
    }
    usize::from_str_radix(&remaining_numbers[0], 2).unwrap()
}

/// Finds the CO2 scrubber rating by reducing the list of binary numbers based on least common
/// bit value per position
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let solver = Solver::new(&example);
/// assert_eq!(find_co2_rating(&solver.input), 10);
/// ```
pub fn find_co2_rating(binary_data: &[String]) -> usize {
    let mut remaining_numbers = binary_data.to_owned();
    // iterate through each digit of the binary string
    for i in 0..binary_data[0].len() {
        if remaining_numbers.len() == 1 {
            break;
        }
        let (ones, zeros) = sort_by_bit_position(&remaining_numbers, i);
        // reject larger collection, select zeros if tied
        if ones.len() < zeros.len() {
            remaining_numbers = ones;
        } else {
            remaining_numbers = zeros;
        }
    }
    usize::from_str_radix(&remaining_numbers[0], 2).unwrap()
}

/// Uses results from count_bits and total length of data to generate binary number
/// and determine gamma rate
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let bit_counts = vec![7,5,8,7,5];
/// let solver = Solver::new(&example);
/// assert_eq!(generate_gamma_rate(&bit_counts, solver.input.len()), 22);
/// ```
pub fn generate_gamma_rate(bit_counts: &[usize], data_length: usize) -> u32 {
    let cutoff = data_length / 2;
    let mut result = String::from("");
    for count in bit_counts.iter() {
        if count > &cutoff {
            result.push('1');
        } else {
            result.push('0');
        }
    }
    u32::from_str_radix(&result, 2).unwrap()
}

/// Uses results from count_bits and total length of data to generate binary number
/// and determine epsilon rate
/// ```
/// use std::fs;
/// use aoc2021::day3::*;
/// let example = fs::read_to_string("day3_example.txt").expect("Error while reading file");
/// let bit_counts = vec![7,5,8,7,5];
/// let solver = Solver::new(&example);
/// assert_eq!(generate_epsilon_rate(&bit_counts, solver.input.len()), 9);
/// ```
pub fn generate_epsilon_rate(bit_counts: &[usize], data_length: usize) -> u32 {
    let cutoff = data_length / 2;
    let mut result = String::from("");
    for count in bit_counts.iter() {
        if count > &cutoff {
            result.push('0');
        } else {
            result.push('1');
        }
    }
    u32::from_str_radix(&result, 2).unwrap()
}
