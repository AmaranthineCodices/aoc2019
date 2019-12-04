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

fn has_consecutive_digits(number: usize) -> bool {
    let digits = get_digits(number);
    let mut last_digit = digits[0];

    for &digit in digits.iter().skip(1) {
        if digit == last_digit {
            return true;
        }

        last_digit = digit;
    }

    false
}

fn has_exactly_two_consecutive_digits(number: usize) -> bool {
    let digits = get_digits(number);

    let mut scanned_index = 0;

    for &digit in digits.iter().take(5) {
        let successive_count = digits[scanned_index..]
            .iter()
            .take_while(|&&d| d == digit)
            .count();

        if successive_count == 2 {
            return true;
        }

        scanned_index += successive_count;
    }

    false
}

fn do_digits_ascend(number: usize) -> bool {
    let digits = get_digits(number);
    let mut last_digit = digits[0];

    for &digit in digits.iter().skip(1) {
        if digit < last_digit {
            return false;
        }

        last_digit = digit;
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

        let part_two = possible_passwords
            .iter()
            .filter(|p| has_exactly_two_consecutive_digits(**p))
            .count();

        println!("Part 2: {} possible passwords under new rules", part_two);
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

    #[test]
    fn test_has_exactly_two_consecutive_digits() {
        assert!(has_exactly_two_consecutive_digits(112345));
        assert!(!has_exactly_two_consecutive_digits(111234));
        assert!(has_exactly_two_consecutive_digits(123345));
        assert!(has_exactly_two_consecutive_digits(123455));
        assert!(!has_exactly_two_consecutive_digits(123334));
        assert!(!has_exactly_two_consecutive_digits(123444));
        assert!(has_exactly_two_consecutive_digits(111122));
        assert!(!has_exactly_two_consecutive_digits(101234));
    }
}
