fn run_interpreter(source_program: &Vec<usize>) -> Vec<usize> {
    let mut opcodes = source_program.clone();
    let mut program_counter = 0;

    loop {
        if program_counter >= opcodes.len() {
            break;
        }

        let opcode = *opcodes.get(program_counter).expect("out of bounds PC");

        if opcode == 99 {
            break;
        }

        let lhs_location = *opcodes.get(program_counter + 1).expect("invalid program");
        let rhs_location = *opcodes.get(program_counter + 2).expect("invalid program");
        let target_location = *opcodes.get(program_counter + 3).expect("invalid program");

        let lhs = opcodes.get(lhs_location).expect("out of bounds LHS");
        let rhs = opcodes.get(rhs_location).expect("out of bounds RHS");

        match opcode {
            1 => {
                opcodes[target_location] = lhs + rhs;
            }
            2 => {
                opcodes[target_location] = lhs * rhs;
            }
            _ => panic!("unknown opcode {}", opcode),
        }

        program_counter += 4;
    }

    opcodes
}

pub struct DayTwo;

impl crate::PuzzleSolver for DayTwo {
    fn description(&self) -> &'static str {
        "Day 2: 1202 Program Alarm"
    }

    fn solve(&self, input: &str) {
        let source_program = input
            .split(",")
            .map(|o| o.parse::<usize>().expect("could not parse to number"))
            .collect();

        let mut opcodes = source_program.clone();

        // Apply hardcoded adjustments to opcodes
        opcodes[1] = 12;
        opcodes[2] = 2;

        let opcodes = run_interpreter(&opcodes);
        println!("Part 1: Value at position 0: {}", opcodes.get(0).unwrap());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpreter() {
        assert_eq!(run_interpreter(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_interpreter(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_interpreter(&vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_interpreter(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }
}
