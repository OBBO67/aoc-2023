use std::{collections::HashMap, fmt::Display};

const SYMBOLS: &'static str = "@#$%^&*/-+=";

struct EngineSchematic {
    vec: Vec<char>,
    rows: usize,
    cols: usize,
}

impl EngineSchematic {
    fn new(height: usize, width: usize) -> Self {
        EngineSchematic {
            vec: vec!['.'; height * width],
            rows: height,
            cols: width,
        }
    }

    fn new_with_values(height: usize, width: usize, values: Vec<&str>) -> Self {
        let mut engine_schematic = EngineSchematic::new(height, width);

        for (row, str) in values.iter().enumerate() {
            for (col, c) in str.char_indices() {
                engine_schematic.set_cell(row, col, c);
            }
        }

        engine_schematic
    }

    fn get_by_index(&self, row: usize, col: usize) -> &char {
        let idx = self.cols * row;
        &self.vec[idx + col]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: char) {
        let idx = self.cols * row;
        self.vec[idx + col] = value;
    }

    fn adjacent_symbol(&self, row: usize, col: usize, num_len: usize, symbols: &str) -> bool {
        for num_pos in 0..num_len {
            if let Some(col) = col.checked_sub(num_pos) {
                if self.in_bounds(row.checked_sub(1), Some(col)) {
                    // above
                    if symbols.contains(*self.get_by_index(row - 1, col)) {
                        return true;
                    }
                }
                if self.in_bounds(row.checked_sub(1), Some(col + 1)) {
                    // upper right
                    if symbols.contains(*self.get_by_index(row - 1, col + 1)) {
                        return true;
                    }
                }
                if self.in_bounds(Some(row), Some(col + 1)) {
                    // right
                    if symbols.contains(*self.get_by_index(row, col + 1)) {
                        return true;
                    }
                }
                if self.in_bounds(Some(row + 1), Some(col + 1)) {
                    // lower right
                    if symbols.contains(*self.get_by_index(row + 1, col + 1)) {
                        return true;
                    }
                }
                if self.in_bounds(Some(row + 1), Some(col)) {
                    // below
                    if symbols.contains(*self.get_by_index(row + 1, col)) {
                        return true;
                    }
                }
                if self.in_bounds(Some(row + 1), col.checked_sub(1)) {
                    // lower left
                    if symbols.contains(*self.get_by_index(row + 1, col - 1)) {
                        return true;
                    }
                }
                if self.in_bounds(Some(row), col.checked_sub(1)) {
                    // left
                    if symbols.contains(*self.get_by_index(row, col - 1)) {
                        return true;
                    }
                }
                if self.in_bounds(row.checked_sub(1), col.checked_sub(1)) {
                    // upper left
                    if symbols.contains(*self.get_by_index(row - 1, col - 1)) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn find_gears(&self, row: usize, col: usize, num_len: usize, symbols: &str) -> String {
        for num_pos in 0..num_len {
            if let Some(col) = col.checked_sub(num_pos) {
                if self.in_bounds(row.checked_sub(1), Some(col)) {
                    // above
                    if symbols.contains(*self.get_by_index(row - 1, col)) {
                        return format!("{},{}", row - 1, col);
                    }
                }
                if self.in_bounds(row.checked_sub(1), Some(col + 1)) {
                    // upper right
                    if symbols.contains(*self.get_by_index(row - 1, col + 1)) {
                        return format!("{},{}", row - 1, col + 1);
                    }
                }
                if self.in_bounds(Some(row), Some(col + 1)) {
                    // right
                    if symbols.contains(*self.get_by_index(row, col + 1)) {
                        return format!("{},{}", row, col + 1);
                    }
                }
                if self.in_bounds(Some(row + 1), Some(col + 1)) {
                    // lower right
                    if symbols.contains(*self.get_by_index(row + 1, col + 1)) {
                        return format!("{},{}", row + 1, col + 1);
                    }
                }
                if self.in_bounds(Some(row + 1), Some(col)) {
                    // below
                    if symbols.contains(*self.get_by_index(row + 1, col)) {
                        return format!("{},{}", row + 1, col);
                    }
                }
                if self.in_bounds(Some(row + 1), col.checked_sub(1)) {
                    // lower left
                    if symbols.contains(*self.get_by_index(row + 1, col - 1)) {
                        return format!("{},{}", row + 1, col - 1);
                    }
                }
                if self.in_bounds(Some(row), col.checked_sub(1)) {
                    // left
                    if symbols.contains(*self.get_by_index(row, col - 1)) {
                        return format!("{},{}", row, col - 1);
                    }
                }
                if self.in_bounds(row.checked_sub(1), col.checked_sub(1)) {
                    // upper left
                    if symbols.contains(*self.get_by_index(row - 1, col - 1)) {
                        return format!("{},{}", row - 1, col - 1);
                    }
                }
            }
        }
        "".to_string()
    }

    fn in_bounds(&self, row: Option<usize>, col: Option<usize>) -> bool {
        if let (Some(row), Some(col)) = (row, col) {
            row < self.rows && col < self.cols
        } else {
            false
        }
    }
}

impl Display for EngineSchematic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{} ", self.get_by_index(row, col))?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let engine_schematic = EngineSchematic::new_with_values(height, width, lines);
    let mut num_str = String::new();
    let mut sum = 0;
    let mut nums_counting = vec![];

    for row in 0..engine_schematic.rows {
        for col in 0..engine_schematic.cols {
            if engine_schematic.get_by_index(row, col).is_numeric() {
                num_str.push(*engine_schematic.get_by_index(row, col));
            } else {
                if !num_str.is_empty() {
                    let (row, col) = match col {
                        0 => (row, engine_schematic.cols - 1),
                        _ => (row, col - 1),
                    };
                    if engine_schematic.adjacent_symbol(row, col, num_str.len(), SYMBOLS) {
                        println!("{} has adjacent symbol", num_str);
                        let to_add = num_str.parse::<u32>().unwrap();
                        nums_counting.push(to_add);
                        sum = sum + to_add;
                    } else {
                        println!("{} doesn't have adjacent symbol", num_str);
                    }
                }
                num_str.clear();
            }
        }

        // edge case at the end of a line
        if !num_str.is_empty() {
            if engine_schematic.adjacent_symbol(
                row,
                engine_schematic.cols - 1,
                num_str.len(),
                SYMBOLS,
            ) {
                println!("{} has adjacent symbol", num_str);
                let to_add = num_str.parse::<u32>().unwrap();
                nums_counting.push(to_add);
                sum = sum + to_add;
            } else {
                println!("{} doesn't have adjacent symbol", num_str);
            }
        }

        num_str.clear();
    }

    println!("{}", nums_counting.len());

    sum
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let height = lines.len();
    let width = lines[0].len();
    let engine_schematic = EngineSchematic::new_with_values(height, width, lines);
    let mut num_str = String::new();
    let mut sum = 0;
    let mut gears = HashMap::new();

    for row in 0..engine_schematic.rows {
        for col in 0..engine_schematic.cols {
            if engine_schematic.get_by_index(row, col).is_numeric() {
                num_str.push(*engine_schematic.get_by_index(row, col));
            } else {
                if !num_str.is_empty() {
                    let (row, col) = match col {
                        0 => (row, engine_schematic.cols - 1),
                        _ => (row, col - 1),
                    };
                    let gear_pos = engine_schematic.find_gears(row, col, num_str.len(), "*");
                    if !gear_pos.is_empty() {
                        let gear = num_str.parse::<u32>().unwrap();
                        if gears.contains_key(&gear_pos) {
                            let other_gear = gears.get(&gear_pos).unwrap();
                            println!("{} {}", gear, other_gear);
                            sum = sum + (other_gear * gear);
                        } else {
                            gears.insert(gear_pos, gear);
                        }
                    }
                }
                num_str.clear();
            }
        }

        if !num_str.is_empty() {
            let gear_pos =
                engine_schematic.find_gears(row, engine_schematic.cols - 1, num_str.len(), "*");
            if !gear_pos.is_empty() {
                let gear = num_str.parse::<u32>().unwrap();
                if gears.contains_key(&gear_pos) {
                    let other_gear = gears.get(&gear_pos).unwrap();
                    sum = sum + (other_gear * gear);
                } else {
                    gears.insert(gear_pos, gear);
                }
            }
        }

        num_str.clear();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);

        assert_eq!(result, 4361);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);

        assert_eq!(result, 467835);
    }
}
