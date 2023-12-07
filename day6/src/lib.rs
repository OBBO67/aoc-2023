pub fn part1(input: &str) -> u32 {
    let (time, distance) = input.split_once("\n").unwrap();
    let (_, time) = time.split_once("Time:").unwrap();
    let time: Vec<&str> = time.split_ascii_whitespace().collect();
    let (_, distance) = distance.split_once("Distance:").unwrap();
    let distance: Vec<&str> = distance.split_ascii_whitespace().collect();

    let time_distance_iter = time.iter().zip(distance.iter());

    let mut ways_to_win_vec = vec![];
    let mut ways_to_win: u32 = 0;

    for (_, (time, distance)) in time_distance_iter.enumerate() {
        println!("Time: {} Distance: {}", time, distance);
        let time = time.parse::<u32>().unwrap();
        let distance = distance.parse::<u32>().unwrap();
        ways_to_win = 0;

        for speed in (1..time) {
            let time_moving = time - speed;
            let dist = time_moving * speed;
            if dist > distance {
                ways_to_win = ways_to_win + 1;
            }
        }

        ways_to_win_vec.push(ways_to_win);
    }

    ways_to_win_vec.into_iter().fold(1, |acc, x| acc * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 288);
    }
}
