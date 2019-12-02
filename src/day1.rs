fn get_fuel_cost(mass: usize) -> usize {
    (mass / 3) - 2
}

fn requires_fuel_addition(mass: usize) -> bool {
    // We need to add some amount of fuel for a mass if, when divided by three,
    // it's greater than 2 still.
    mass / 3 > 2
}

pub struct DayOne;

impl crate::PuzzleSolver for DayOne {
    fn description(&self) -> &'static str {
        "Day One: The Tyrrany of the Rocket Equation"
    }

    fn solve(&self, input: &str) {
        let masses: Vec<usize> = input
            .lines()
            .map(|l| {
                l.parse::<usize>()
                    .expect("Malformed input; unable to parse to usize")
            })
            .collect();

        let module_fuel_costs: Vec<usize> = masses.iter().map(|m| get_fuel_cost(*m)).collect();

        let module_fuel_cost: usize = module_fuel_costs.iter().sum();
        println!("Part one (module fuel cost): {}", module_fuel_cost);

        let finalized_fuel_cost: usize = module_fuel_costs
            .iter()
            .map(|f| {
                let mut last_fuel = *f;
                let mut accumulator: usize = 0;
                while requires_fuel_addition(last_fuel) {
                    let new_fuel = get_fuel_cost(last_fuel);
                    accumulator = accumulator + new_fuel;
                    last_fuel = new_fuel;
                }

                f + accumulator
            })
            .sum();

        println!("Part two (total fuel cost): {}", finalized_fuel_cost)
    }
}
