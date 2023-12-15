#![feature(ascii_char)]
#![feature(hasher_prefixfree_extras)]

use std::hash::{Hash, Hasher};

struct HolidayHasher {
    state: u64,
}

impl HolidayHasher {
    fn new() -> Self {
        Self {state: 0}
    }
}

impl Hasher for HolidayHasher {
    fn finish(&self) -> u64 {
        self.state
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.state += *byte as u64;
            self.state *= 17;
            self.state %= 256;
        }
    }

    fn write_str(&mut self, s: &str) {
        if !s.is_ascii() {
            panic!("Not an ascii string")
        }
        self.write(s.as_bytes());
    }
}

fn main() {
    let input = include_str!("../input1.txt").strip_suffix('\n').unwrap();
    let result = input.split(',')
        .map(|part| {
            let mut hasher = HolidayHasher::new();
            part.hash(&mut hasher);
            hasher.finish()
        })
        .sum::<u64>();
    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use std::hash::Hash;
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = input.split(',')
            .map(|part| {
                let mut hasher = HolidayHasher::new();
                part.hash(&mut hasher);
                hasher.finish()
            })
            .sum::<u64>();
        assert_eq!(result, 1320);
    }
    #[test]
    fn test_hasher_1() {
        let mut hasher = HolidayHasher::new();
        "rn=1".hash(&mut hasher);
        assert_eq!(hasher.finish(), 30);
    }

    #[test]
    fn test_hasher_2() {
        let mut hasher = HolidayHasher::new();
        "cm-".hash(&mut hasher);
        assert_eq!(hasher.finish(), 253);
    }

    #[test]
    fn test_hasher_3() {
        let mut hasher = HolidayHasher::new();
        "qp=3".hash(&mut hasher);
        assert_eq!(hasher.finish(), 97);
    }
}