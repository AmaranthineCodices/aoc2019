use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

trait PuzzleSolver {
    fn description(&self) -> &'static str;
    fn solve(&self, input: &str);
}

fn main() {
    let puzzle_solvers: Vec<Box<dyn PuzzleSolver>> = vec![
        Box::new(day1::DayOne),
        Box::new(day2::DayTwo),
        Box::new(day3::DayThree),
        Box::new(day4::DayFour),
        Box::new(day5::DayFive),
        Box::new(day6::DaySix),
    ];

    let args: Vec<String> = env::args().collect();
    let puzzle_day = args
        .get(1)
        .expect("expected argument 1 to be present")
        .parse::<usize>()
        .expect("expected argument 1 to be a number");
    let input_file = args.get(2).expect("expected argument 2 to be present");

    let puzzle_input = fs::read_to_string(input_file).expect("could not read input file");
    // Subtract 1 from puzzle_day since Vec indices are 0-based, but days are 1-based
    let puzzle_solver = puzzle_solvers
        .get(puzzle_day - 1)
        .expect("invalid puzzle index");

    println!("Solving {}", puzzle_solver.description());
    puzzle_solver.solve(&puzzle_input);
}
