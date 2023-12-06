use day6::*;

fn parse_game(input: &str) -> (Num, Num) {
    let mut lines = input.lines();

    let time: Num = lines.next().unwrap()
        .strip_prefix("Time:").unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();

    let distance: Num = lines.next().unwrap()
        .strip_prefix("Distance:").unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("")
        .parse().unwrap();

    (time, distance)
}

fn main() {
    let input = include_str!("../input1.txt");
    let (time, distance) = parse_game(input);
    let (low, high) = search(time, distance);
    let score = high - low + 1;

    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let (time, distance) = parse_game(input);
        let (low, high) = search(time, distance);
        let score = high - low + 1;

        assert_eq!(score, 71503)
    }
}
