use std::iter::zip;

type Num = i64;
type FNum = f64;

fn evaluate(hold_len: Num, total_len: Num) -> Num {
    -hold_len * (hold_len - total_len)
}

enum SearchDirection {
    Left,
    Right
}

fn calc_mid(high: Num, low: Num, direction: &SearchDirection) -> Num {
    match direction {
        SearchDirection::Left => FNum::ceil((high - low) as FNum / 2.0) as Num + low,
        SearchDirection::Right => FNum::floor((high - low) as FNum / 2.0) as Num + low
    }
}
fn do_search(high: Num, low: Num, target: Num, total_len: Num, direction: SearchDirection) -> Num {
    let mut cur_high = high;
    let mut cur_low = low;
    let mut cur_mid = calc_mid(cur_high, cur_low, &direction);

    while cur_mid != cur_low && cur_mid != cur_high {
        let mid_val = evaluate(cur_mid, total_len);

        match direction {

            SearchDirection::Left => {
                if mid_val == target {
                    return cur_mid + 1
                }

                if mid_val > target {
                    cur_high = cur_mid;
                }

                if mid_val < target {
                    cur_low = cur_mid;
                }
            }

            SearchDirection::Right => {
                if mid_val == target {
                    return cur_mid - 1
                }

                if mid_val < target {
                    cur_high = cur_mid;
                }

                if mid_val > target {
                    cur_low = cur_mid;
                }
            }

        }
        cur_mid = calc_mid(cur_high, cur_low, &direction);
    }

    match direction {
        SearchDirection::Left => cur_high,
        SearchDirection::Right => cur_low,
    }
}

fn get_bounds(duration: Num) -> ((Num, Num), (Num, Num)) {
    let left_bounds = (0, FNum::floor(duration as FNum / 2.0) as Num);
    let right_bounds = (FNum::ceil(duration as FNum / 2.0) as Num, duration);
    (left_bounds, right_bounds)
}

fn search(duration: Num, record_distance: Num) -> (Num, Num) {
    let (left_bounds, right_bounds) = get_bounds(duration);
    let left = do_search(left_bounds.1, left_bounds.0, record_distance, duration, SearchDirection::Left);
    let right = do_search(right_bounds.1, right_bounds.0, record_distance, duration, SearchDirection::Right);
    (left, right)
}

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
