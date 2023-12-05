use std::{collections::HashMap, ops::Range, str::FromStr};

struct Mapping {
    src: String,
    dest: String,
    range_map: HashMap<Range<u32>, Range<u32>>,
}

impl FromStr for Mapping {
    type Err = ();

    /// Takes a mapping string and converts it to the Mapping struct
    /// Example:
    /// seed-to-soil map:
    /// 50 98 2
    /// 52 50 48
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (src_dest, mappings) = s.split_once("\n").unwrap();

        let (src_dest, _) = src_dest.split_once(' ').unwrap();
        let (src, dest) = src_dest.split_once("-to-").unwrap();

        let mappings: Vec<&str> = mappings.split("\n").collect();
        let mut range_map = HashMap::new();

        for mapping in mappings {
            let mapping: Vec<&str> = mapping.splitn(3, ' ').collect();
            let dest_start = mapping[0].parse::<u32>().unwrap();
            let src_start = mapping[1].parse::<u32>().unwrap();
            let range = mapping[2].parse::<u32>().unwrap();

            let dest_end = dest_start + range;
            let src_end = src_start + range;

            range_map.insert(src_start..src_end, dest_start..dest_end);
        }

        Ok(Mapping {
            src: src.to_string(),
            dest: dest.to_string(),
            range_map,
        })
    }
}

// dest range start, src range start, range length
// upper num = (range start - 1) + range length
pub fn part1(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 4);
    }
}
