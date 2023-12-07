pub fn part1(input: &str) -> u32 {
    let (time, distance) = input.split_once('\n').unwrap();
    let (_, time) = time.split_once("Time:").unwrap();
    let time: Vec<&str> = time.split_ascii_whitespace().collect();
    let (_, distance) = distance.split_once("Distance:").unwrap();
    let distance: Vec<&str> = distance.split_ascii_whitespace().collect();

    let time_distance_iter = time.iter().zip(distance.iter());

    let mut ways_to_win_vec = vec![];

    for (_, (time, distance)) in time_distance_iter.enumerate() {
        println!("Time: {} Distance: {}", time, distance);
        let time = time.parse::<u32>().unwrap();
        let distance = distance.parse::<u32>().unwrap();
        let mut ways_to_win: u32 = 0;

        for speed in 1..time {
            let time_moving = time - speed;
            let dist = time_moving * speed;
            if dist > distance {
                ways_to_win += 1;
            }
        }

        ways_to_win_vec.push(ways_to_win);
    }

    ways_to_win_vec.into_iter().product::<u32>()
}

pub fn part2(input: &str) -> u64 {
    let (time, distance) = input.split_once('\n').unwrap();
    let (_, time) = time.split_once("Time:").unwrap();
    let time: Vec<&str> = time.split_ascii_whitespace().collect();
    let (_, distance) = distance.split_once("Distance:").unwrap();
    let distance: Vec<&str> = distance.split_ascii_whitespace().collect();

    let time = time.into_iter().fold(String::new(), |mut acc, t| {
        acc.push_str(t);
        acc
    });

    let distance = distance.into_iter().fold(String::new(), |mut acc, d| {
        acc.push_str(d);
        acc
    });

    let mut ways_to_win: u64 = 0;
    println!("Time: {} Distance: {}", time, distance);
    let time = time.parse::<u64>().unwrap();
    let distance = distance.parse::<u64>().unwrap();

    for speed in 1..time {
        let time_moving = time - speed;
        let dist = time_moving * speed;
        if dist > distance {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 288);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(result, 71503);
    }
}
