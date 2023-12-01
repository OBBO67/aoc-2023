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

pub fn part2(input: &str) -> i32 {
    // leave first and last character of digit words
    // to solve the issue with overlaps
    let array = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    input
        .lines()
        .map(|line| {
            let mut new_line = String::from(line);
            for x in array {
                new_line = new_line.replace(x.0, x.1);
            }

            new_line
        })
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
        assert_eq!(result, 54877);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(result, 54100);
    }
}
