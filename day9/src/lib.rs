pub type Num = i32;

pub fn parse_line(input: &str) -> Vec<Num> {
    input.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn all_zeroes(input: &[Num]) -> bool {
    for num in input {
        if *num != 0 {
            return false;
        }
    }
    true
}


// Lets try to do this without allocating :D
pub fn calc_next(buf: &mut Vec<Num>) {
    for idx in 0..buf.len() - 1 {
        buf[idx] = buf[idx + 1] - buf[idx]
    }
    buf.pop();
}

pub fn extrapolate(input: &[Num]) -> Num {
    let mut last_nums = Vec::new();
    let mut cur_level = Vec::from(input);

    while !all_zeroes(&cur_level) {
        last_nums.push(*cur_level.last().unwrap());
        calc_next(&mut cur_level);
    }

    if cur_level.is_empty() {
        panic!("Invalid calculation")
    }

    last_nums.iter().sum()
}