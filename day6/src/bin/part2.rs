use std::time::Instant;
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

    let start_search = Instant::now();
    let (low, high) = search(time, distance);
    let duration_search = start_search.elapsed();
    let score_search = high - low + 1;

    let start_calc = Instant::now();
    let (low, high) = calculate(time, distance);
    let duration_calc = start_calc.elapsed();
    let score_calc = high - low + 1;

    assert_eq!(score_calc, score_search, "Scores not equal!");

    println!("Score: {}", score_search);
    println!("Search took: {:?}", duration_search);
    println!("Calc took: {:?}", duration_calc);
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
