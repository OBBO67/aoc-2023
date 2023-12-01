use std::str::FromStr;

struct CalibartionValue {
    value: String,
}

impl CalibartionValue {
    fn new(first: &str, last: &str) -> Self {
        CalibartionValue {
            value: format!("{first}{last}"),
        }
    }
}

impl FromStr for CalibartionValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.matches(char::is_numeric).collect();

        match nums.len() {
            0 => Err(()),
            _ => Ok(CalibartionValue::new(
                nums.first().unwrap(),
                nums.last().unwrap(),
            )),
        }
    }
}

pub fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.parse::<CalibartionValue>().unwrap().value)
        .map(|value| value.parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 142);
    }
}
