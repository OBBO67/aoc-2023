use core::num;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    card_winning_nums: u32,
    card_points: u32,
}

impl Card {
    fn determine_card_points(&mut self) {
        self.count_winning_numbers();
        if self.card_winning_nums != 0 {
            self.card_points = u32::pow(2, self.card_winning_nums - 1);
        }
    }

    fn count_winning_numbers(&mut self) {
        let mut count: u32 = 0;

        for num in &self.numbers {
            if self.winning_numbers.contains(num) {
                count = count + 1;
            }
        }

        self.card_winning_nums = count;
    }
}

/// Creates a Card struct from a card string line
/// Example: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_id, numbers) = s.split_once(':').unwrap();
        let (_, id) = card_id.split_once(' ').unwrap();

        let (winning_numbers, numbers) = numbers.trim().split_once('|').unwrap();

        let winning_numbers: Vec<u32> = winning_numbers
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect();

        let numbers: Vec<u32> = numbers
            .split_whitespace()
            .map(|number| number.parse::<u32>().unwrap())
            .collect();

        Ok(Card {
            id: id.trim().parse::<u32>().unwrap(),
            winning_numbers,
            numbers,
            card_winning_nums: 0,
            card_points: 0,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let total_card_points: u32 = input
        .lines()
        .map(|line| {
            let mut card = line.parse::<Card>().unwrap();
            card.determine_card_points();
            card.card_points
        })
        .sum();

    total_card_points
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 13);
    }
}
