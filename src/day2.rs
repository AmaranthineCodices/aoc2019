fn run_interpreter(
    source_program: &Vec<usize>,
    noun: Option<usize>,
    verb: Option<usize>,
) -> Vec<usize> {
    let mut opcodes = source_program.clone();

    if let Some(noun) = noun {
        opcodes[1] = noun;
    }

    if let Some(verb) = verb {
        opcodes[2] = verb;
    }

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
        let source_program: Vec<usize> = input
            .split(",")
            .map(|o| o.parse::<usize>().expect("could not parse to number"))
            .collect();

        let opcodes = run_interpreter(&source_program, Some(12), Some(2));
        println!("Part 1: Value at position 0: {}", opcodes.get(0).unwrap());

        const TARGET_VALUE: usize = 19_690_720;

        'outer: for noun in 0..=99 {
            for verb in 0..=99 {
                let output_memory = run_interpreter(&source_program, Some(noun), Some(verb));
                if output_memory[0] == TARGET_VALUE {
                    println!(
                        "Part 2:\n\tNoun: {}\n\tVerb: {}\n\t100 * noun + verb: {}",
                        noun,
                        verb,
                        100 * noun + verb
                    );

                    break 'outer;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn interpreter() {
        assert_eq!(
            run_interpreter(&vec![1, 0, 0, 0, 99], None, None),
            vec![2, 0, 0, 0, 99]
        );
        assert_eq!(
            run_interpreter(&vec![2, 3, 0, 3, 99], None, None),
            vec![2, 3, 0, 6, 99]
        );
        assert_eq!(
            run_interpreter(&vec![2, 4, 4, 5, 99, 0], None, None),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_interpreter(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99], None, None),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        )
    }
}
