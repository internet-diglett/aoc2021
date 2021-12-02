// TODO Find a decent logging library?
pub mod day1;

pub trait PuzzleSolver {
    fn part_one(&self) -> String {
        String::from("Unsolved")
    }

    fn part_two(&self) -> String {
        String::from("Unsolved")
    }

    fn part_one_answer(&self) {
        println!("Part One: {}", self.part_one())
    }

    fn part_two_answer(&self) {
        println!("Part Two: {}", self.part_two())
    }

    fn solve(&self) {
        self.part_one_answer();
        self.part_two_answer();
    }
}

pub fn new_solver(day: &str, input: &str) -> Option<Box<dyn PuzzleSolver>> {
    match day {
        "1" => Some(Box::new(day1::Solver::new(input))),
        _ => None,
    }
}
