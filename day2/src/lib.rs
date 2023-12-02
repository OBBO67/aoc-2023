use std::{cmp, str::FromStr};

#[derive(Default)]
struct CubeSet {
    green: u32,
    blue: u32,
    red: u32,
}

struct Game {
    id: u32,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn game_is_possible(&self) -> bool {
        for cube_set in &self.cube_sets {
            if cube_set.red > 12 || cube_set.green > 13 || cube_set.blue > 14 {
                return false;
            }
        }
        true
    }

    fn min_cubes_required(&self) -> CubeSet {
        let mut cube_set = CubeSet::default();

        for set in &self.cube_sets {
            cube_set.blue = cmp::max(cube_set.blue, set.blue);
            cube_set.green = cmp::max(cube_set.green, set.green);
            cube_set.red = cmp::max(cube_set.red, set.red);
        }

        cube_set
    }
}

impl FromStr for Game {
    type Err = ();

    /// Create a game from a game string line
    /// Example: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, cube_sets) = s.split_once(':').unwrap();
        let (_, game_id) = game_id.split_once(' ').unwrap();
        let id = game_id.parse::<u32>().unwrap();

        let cube_sets: Vec<CubeSet> = cube_sets
            .split(';')
            .map(|cube_set| cube_set.parse::<CubeSet>().unwrap())
            .collect();

        Ok(Game { id, cube_sets })
    }
}

impl FromStr for CubeSet {
    type Err = ();

    /// Create Cubes set from a cube set string line
    /// Example: "1 red, 2 green, 6 blue"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cube_set = CubeSet::default();

        for cube in s.split(',') {
            let cube = cube.trim();
            let (count, colour) = cube.split_once(' ').unwrap();
            let count = count.parse::<u32>().unwrap();
            match colour {
                "red" => cube_set.red = count,
                "blue" => cube_set.blue = count,
                "green" => cube_set.green = count,
                _ => (),
            }
        }

        Ok(cube_set)
    }
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter(|game| game.game_is_possible())
        .map(|game| game.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| {
            let min_cube_set = game.min_cubes_required();
            min_cube_set.blue * min_cube_set.green * min_cube_set.red
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 8);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(result, 2286);
    }
}
