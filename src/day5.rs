fn get_digits(number: usize) -> Vec<u8> {
    let mut digits = Vec::new();
    let mut remainder = number;

    while remainder > 0 {
        let current_digit = (remainder % 10) as u8;
        digits.push(current_digit);
        remainder = remainder / 10;
    }

    digits.reverse();
    digits
}

fn reconstitute_from_digits(digits: &Vec<u8>) -> usize {
    let mut number = 0;

    for (index, &digit) in digits.iter().rev().enumerate() {
        number += 10usize.pow(index as u32) * digit as usize;
    }

    number
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

fn decode_opcode(raw_opcode: usize) -> (usize, Vec<ParameterMode>) {
    let mut digits = get_digits(raw_opcode);
    digits.reverse();
    let opcode_digits = digits.iter().take(2).map(|&r| r).rev().collect();
    let opcode = reconstitute_from_digits(&opcode_digits);

    let modes = digits
        .iter()
        .skip(opcode_digits.len())
        .map(|d| match d {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("unknown parameter mode {} in raw opcode {}", d, raw_opcode),
        })
        .collect();

    (opcode, modes)
}

fn resolve_value(memory: &Vec<isize>, value: isize, mode: ParameterMode) -> isize {
    match mode {
        ParameterMode::Immediate => value,
        ParameterMode::Position => *memory
            .get(value as usize)
            .expect("out of bounds position value"),
    }
}

fn run_interpreter(source_program: &Vec<isize>, input_value: isize) -> Vec<isize> {
    let mut memory = source_program.clone();
    let mut output = Vec::new();
    let mut program_counter = 0;

    loop {
        if program_counter >= memory.len() {
            break;
        }

        let raw_opcode = *memory.get(program_counter).expect("out of bounds PC");
        // Assumption: opcodes are non-negative values
        let (opcode, modes) = decode_opcode(raw_opcode as usize);

        match opcode {
            1 | 2 | 7 | 8 => {
                let target_location = *memory.get(program_counter + 3).expect("invalid program");
                assert_eq!(
                    *modes.get(2).unwrap_or(&ParameterMode::Position),
                    ParameterMode::Position
                );

                let lhs = resolve_value(
                    &memory,
                    *memory.get(program_counter + 1).expect("invalid program"),
                    *modes.get(0).unwrap_or(&ParameterMode::Position),
                );

                let rhs = resolve_value(
                    &memory,
                    *memory.get(program_counter + 2).expect("invalid program"),
                    *modes.get(1).unwrap_or(&ParameterMode::Position),
                );

                match opcode {
                    1 => memory[target_location as usize] = lhs + rhs,
                    2 => memory[target_location as usize] = lhs * rhs,
                    7 => {
                        let value = if lhs < rhs { 1 } else { 0 };

                        memory[target_location as usize] = value;
                    }
                    8 => {
                        let value = if lhs == rhs { 1 } else { 0 };

                        memory[target_location as usize] = value;
                    }
                    _ => unreachable!(),
                };

                program_counter += 4;
            }
            3 => {
                let target_location = *memory.get(program_counter + 1).expect("invalid program");
                assert_eq!(
                    *modes.get(0).unwrap_or(&ParameterMode::Position),
                    ParameterMode::Position
                );

                memory[target_location as usize] = input_value;
                program_counter += 2;
            }
            4 => {
                let value = resolve_value(
                    &memory,
                    *memory.get(program_counter + 1).expect("invalid program"),
                    *modes.get(0).unwrap_or(&ParameterMode::Position),
                );

                output.push(value);
                program_counter += 2;
            }
            5 | 6 => {
                let test = resolve_value(
                    &memory,
                    *memory.get(program_counter + 1).expect("invalid program"),
                    *modes.get(0).unwrap_or(&ParameterMode::Position),
                );

                let new_location = resolve_value(
                    &memory,
                    *memory.get(program_counter + 2).expect("invalid program"),
                    *modes.get(1).unwrap_or(&ParameterMode::Position),
                );

                let cond = match opcode {
                    5 => test != 0,
                    6 => test == 0,
                    _ => unreachable!(),
                };

                if cond {
                    program_counter = new_location as usize;
                } else {
                    program_counter += 3;
                }
            }
            99 => break,
            _ => panic!(
                "unknown opcode {} at program counter {}\nmemory dump: {:#?}",
                opcode, program_counter, memory
            ),
        }
    }

    output
}

pub struct DayFive;

impl crate::PuzzleSolver for DayFive {
    fn description(&self) -> &'static str {
        "Day 5: Sunny with a Chance of Asteroids"
    }

    fn solve(&self, input: &str) {
        let source_program: Vec<isize> = input
            .split(",")
            .map(|o| o.parse::<isize>().expect("could not parse to number"))
            .collect();

        println!("Part 1: Running program for input value 1");
        let output = run_interpreter(&source_program, 1);
        println!(
            "All checks passed: {}",
            output.iter().take(output.len() - 1).all(|&v| v == 0)
        );
        println!(
            "Final output value (answer to part 1): {}",
            output.last().unwrap()
        );

        println!("Part 2: Running program for input value 5");
        let output = run_interpreter(&source_program, 5);
        println!(
            "Final output value (answer to part 2): {}",
            output.last().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpreter() {
        let source_program = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let output = run_interpreter(&source_program, 8);
        assert_eq!(output, vec![1]);

        let output = run_interpreter(&source_program, 1);
        assert_eq!(output, vec![0]);
    }
}
