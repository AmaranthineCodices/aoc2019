fn get_fuel_cost(mass: usize) -> usize {
    (mass / 3) - 2
}

fn get_total_fuel_cost(masses: Vec<usize>) -> usize {
    masses.iter().map(|m| get_fuel_cost(*m)).sum()
}

pub struct DayOne;

impl crate::PuzzleSolver for DayOne {
    fn description(&self) -> &'static str {
        "Day One: The Tyrrany of the Rocket Equation"
    }

    fn solve(&self, input: &str) {
        let masses = input
            .lines()
            .map(|l| {
                l.parse::<usize>()
                    .expect("Malformed input; unable to parse to usize")
            })
            .collect();

        let total_fuel_cost = get_total_fuel_cost(masses);
        println!("Part one (total fuel cost): {}", total_fuel_cost);
    }
}
