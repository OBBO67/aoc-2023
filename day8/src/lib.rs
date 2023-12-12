use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct HauntedWasteland {
    instructions: Vec<char>,
    network_map: HashMap<String, (String, String)>,
}

impl FromStr for HauntedWasteland {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instructions, network) = s.split_once("\n\n").unwrap();

        println!("{}", instructions);
        println!("{}", network);

        let mut network_map = HashMap::new();

        for line in network.lines() {
            let (key, value) = line.split_once(" = (").unwrap();
            let (left_val, right_val) = value.split_once(", ").unwrap();
            let right_val = right_val.strip_suffix(')').unwrap();

            network_map.insert(
                key.to_string(),
                (left_val.to_string(), right_val.to_string()),
            );
        }

        Ok(HauntedWasteland {
            instructions: instructions.chars().collect(),
            network_map,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let haunted_wasteland = input.parse::<HauntedWasteland>().unwrap();

    let mut steps: u32 = 0;
    let mut location = "AAA".to_string();
    let target_location = "ZZZ".to_string();

    for instruction in haunted_wasteland.instructions.iter().cycle() {
        if *instruction == 'L' {
            location = haunted_wasteland
                .network_map
                .get(&location)
                .unwrap()
                .0
                .clone();
        } else {
            location = haunted_wasteland
                .network_map
                .get(&location)
                .unwrap()
                .1
                .clone();
        }
        steps = steps + 1;

        if location.eq(&target_location) {
            break;
        }
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 6);
    }
}
