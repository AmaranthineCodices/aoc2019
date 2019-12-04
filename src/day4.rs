fn has_consecutive_digits(number: usize) -> bool {
    let mut last_digit = number % 10;
    let mut remainder = number / 10;

    while remainder > 0 {
        let current_digit = remainder % 10;

        if current_digit == last_digit {
            return true;
        }

        last_digit = current_digit;
        remainder = remainder / 10;
    }

    false
}

fn do_digits_ascend(number: usize) -> bool {
    let mut last_digit = number % 10;
    let mut remainder = number / 10;

    // This goes in reverse order so we test if the current digit
    // is greater than the last - this means that for 10, current digit
    // will be 1 and last digit will be 0, so if current digit > last
    // digit, they're not ascending
    while remainder > 0 {
        let current_digit = remainder % 10;

        if current_digit > last_digit {
            return false;
        }

        last_digit = current_digit;
        remainder = remainder / 10;
    }

    true
}

pub struct DayFour;

impl crate::PuzzleSolver for DayFour {
    fn description(&self) -> &'static str {
        "Day 4: Secure Container"
    }

    fn solve(&self, input: &str) {
        let mut parts = input.split("-");
        let minimum = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("couldn't parse minimum");

        let maximum = parts
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("couldn't parse maximum");

        let mut possible_passwords = Vec::new();

        for possible_password in minimum..=maximum {
            if has_consecutive_digits(possible_password) && do_digits_ascend(possible_password) {
                possible_passwords.push(possible_password);
            }
        }

        println!("Part 1: {} possible passwords", possible_passwords.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_consecutive_digits() {
        assert!(has_consecutive_digits(112345));
        assert!(!has_consecutive_digits(123456));
        assert!(has_consecutive_digits(123445));
    }

    #[test]
    fn test_do_digits_ascend() {
        assert!(do_digits_ascend(123456));
        assert!(!do_digits_ascend(123245));
    }
}