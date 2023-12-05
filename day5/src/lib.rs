use std::{collections::HashMap, ops::Range, str::FromStr};

struct Mapping {
    src: String,
    dest: String,
    range_map: HashMap<Range<i32>, Range<i32>>,
}

impl Mapping {
    fn get_dest_from_src(&self, src: i32) -> i32 {
        for src_range in self.range_map.keys() {
            if src_range.contains(&src) {
                let dest_range = self.range_map.get(src_range).unwrap();
                let offset = dest_range.start - src_range.start;
                return src + offset;
            }
        }

        src
    }
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
            let dest_start = mapping[0].parse::<i32>().unwrap();
            let src_start = mapping[1].parse::<i32>().unwrap();
            let range = mapping[2].parse::<i32>().unwrap();

            let dest_end = dest_start + range;
            let src_end = src_start + range;

            // end in range is exclusive (start <= x < end)
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
    let parts: Vec<&str> = input.lines().collect();
    println!("{:?}", parts);

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");
    const MAPPING_TEST_INPUT: &'static str = include_str!("../mapping_test_input.txt");

    #[test]
    fn mapping_from_str_test() {
        let mapping = MAPPING_TEST_INPUT.parse::<Mapping>().unwrap();
        assert_eq!(String::from("seed"), mapping.src);
        assert_eq!(String::from("soil"), mapping.dest);
        assert_eq!((50..52), *mapping.range_map.get(&(98..100)).unwrap());
    }

    #[test]
    fn get_dest_from_src_test() {
        let mapping = MAPPING_TEST_INPUT.parse::<Mapping>().unwrap();
        let dest1 = mapping.get_dest_from_src(0);
        let dest2 = mapping.get_dest_from_src(50);
        let dest3 = mapping.get_dest_from_src(99);

        assert_eq!(dest1, 0);
        assert_eq!(dest2, 52);
        assert_eq!(dest3, 51);
    }

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 4);
    }
}
