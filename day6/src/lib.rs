pub type Num = i64;
pub type FNum = f64;

pub fn evaluate(hold_len: Num, total_len: Num) -> Num {
    -hold_len * (hold_len - total_len)
}

pub enum SearchDirection {
    Left,
    Right
}

pub fn calc_mid(high: Num, low: Num, direction: &SearchDirection) -> Num {
    match direction {
        SearchDirection::Left => FNum::ceil((high - low) as FNum / 2.0) as Num + low,
        SearchDirection::Right => FNum::floor((high - low) as FNum / 2.0) as Num + low
    }
}
pub fn do_search(high: Num, low: Num, target: Num, total_len: Num, direction: SearchDirection) -> Num {
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

pub fn get_bounds(duration: Num) -> ((Num, Num), (Num, Num)) {
    let left_bounds = (0, FNum::floor(duration as FNum / 2.0) as Num);
    let right_bounds = (FNum::ceil(duration as FNum / 2.0) as Num, duration);
    (left_bounds, right_bounds)
}

pub fn search(duration: Num, record_distance: Num) -> (Num, Num) {
    let (left_bounds, right_bounds) = get_bounds(duration);
    let left = do_search(left_bounds.1, left_bounds.0, record_distance, duration, SearchDirection::Left);
    let right = do_search(right_bounds.1, right_bounds.0, record_distance, duration, SearchDirection::Right);

    (left, right)
}

pub fn calculate(duration: Num, record_distance: Num) -> (Num, Num) {
    let a = 1.0;
    let b = duration as FNum;
    let c = record_distance as FNum;

    let mut left_f = -(-b + (b * b - 4.0 * a * c).sqrt()) / 2.0 * a;
    let mut right_f = -(-b - (b * b - 4.0 * a * c).sqrt()) / 2.0 * a;

    // To prevent rounding issues on "whole" numbers
    if left_f.fract() == 0.0 {
        left_f += 1e-32;
    }
    if right_f.fract() == 0.0 {
        right_f -= 1e-32;
    }

    let left_ceil = left_f.ceil() as Num;
    let right_floor = right_f.floor() as Num;

    (left_ceil, right_floor)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounds_even() {
        let (left_bounds, right_bounds) = get_bounds(10);
        assert_eq!(left_bounds.0, 0);
        assert_eq!(left_bounds.1, 5);
        assert_eq!(right_bounds.0, 5);
        assert_eq!(right_bounds.1, 10);
    }

    #[test]
    fn test_bounds_odd() {
        let (left_bounds, right_bounds) = get_bounds(11);
        assert_eq!(left_bounds.0, 0);
        assert_eq!(left_bounds.1, 5);
        assert_eq!(right_bounds.0, 6);
        assert_eq!(right_bounds.1, 11);
    }

    #[test]
    fn test_search_left() {
        let (left_bounds, _) = get_bounds(7);
        let x = do_search(left_bounds.1, left_bounds.0, 9, 7, SearchDirection::Left);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_search_right() {
        let (_, right_bounds) = get_bounds(7);
        let x = do_search(right_bounds.1, right_bounds.0, 9, 7, SearchDirection::Right);
        assert_eq!(x, 5);
    }

    #[test]
    fn test_search_left_2() {
        let (left_bounds, _) = get_bounds(15);
        let x = do_search(left_bounds.1, left_bounds.0, 40, 15, SearchDirection::Left);
        assert_eq!(x, 4);
    }

    #[test]
    fn test_search_right_2() {
        let (_, right_bounds) = get_bounds(15);
        let x = do_search(right_bounds.1, right_bounds.0, 40, 15, SearchDirection::Right);
        assert_eq!(x, 11);
    }

    #[test]
    fn test_search_left_3() {
        let (left_bounds, _) = get_bounds(30);
        let x = do_search(left_bounds.1, left_bounds.0, 200, 30, SearchDirection::Left);
        assert_eq!(x, 11);
    }

    #[test]
    fn test_search_right_3() {
        let (_, right_bounds) = get_bounds(30);
        let x = do_search(right_bounds.1, right_bounds.0, 200, 30, SearchDirection::Right);
        assert_eq!(x, 19);
    }
}