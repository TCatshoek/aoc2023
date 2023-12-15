#![feature(ascii_char)]
#![feature(hasher_prefixfree_extras)]
#![feature(linked_list_remove)]

use std::collections::{HashMap, LinkedList};
use std::hash::{BuildHasherDefault, Hash, Hasher};
use itertools::Itertools;

#[derive(Default)]
struct HolidayHasher {
    state: u64,
}

impl HolidayHasher {
    fn new() -> Self {
        Self { state: 0 }
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
    let input = include_str!("../input1.txt");
    let boxes = initialization_sequence(input);
    let result = boxes.iter()
        .map(|(idx, contents)| {
            (*idx as usize + 1) * contents.iter().enumerate()
                .map(|(i, (_, f))| (i + 1) * f).sum::<usize>()
        }).sum::<usize>();
    println!("Result: {}", result);
}

type HolidayHashMap<K, V> = HashMap<K, V, BuildHasherDefault<HolidayHasher>>;

fn get_hash(input: &str) -> u64 {
    let mut hasher = HolidayHasher::new();
    input.hash(&mut hasher);
    hasher.finish()
}

fn initialization_sequence(input: &str) -> HashMap<u64, LinkedList<(&str, usize)>, BuildHasherDefault<HolidayHasher>> {
    let instructions = input.strip_suffix('\n')
        .unwrap_or(input)
        .split(',')
        .collect::<Vec<_>>();

    let mut boxes = HolidayHashMap::default();

    for instruction in instructions {
        if instruction.contains('=') {
            let (label, number) = instruction.split('=').collect_tuple().unwrap();
            let number = number.parse::<usize>().unwrap();

            boxes.entry(get_hash(label))
                .and_modify(|box_contents: &mut LinkedList<(_, _)>| {
                    if let Some(x) = box_contents.iter_mut().find(|el| el.0 == label) {
                        x.1 = number
                    } else {
                        box_contents.push_back((label, number));
                    }
                })
                .or_insert(LinkedList::from([(label, number)]));
        }

        if instruction.contains('-') {
            let (label, _) = instruction.split('-').collect_tuple().unwrap();
            boxes.entry(get_hash(label))
                .and_modify(|box_contents| {
                    if let Some((idx, _)) = box_contents.iter().find_position(|el| el.0 == label) {
                        box_contents.remove(idx);
                    }
                });
        }
    }

    boxes
}

#[cfg(test)]
mod test {
    use std::hash::Hash;
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let boxes = initialization_sequence(input);
        let result = boxes.iter()
            .map(|(idx, contents)| {
                (*idx as usize + 1) * contents.iter().enumerate().map(|(i, (_, f))| (i + 1) * f).sum::<usize>()
            }).sum::<usize>();

        assert_eq!(result, 145);
    }

    #[test]
    fn test_hasher_1() {
        let mut hasher = HolidayHasher::new();
        "rn".hash(&mut hasher);
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn test_hasher_2() {
        let mut hasher = HolidayHasher::new();
        "cm".hash(&mut hasher);
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn test_hasher_3() {
        let mut hasher = HolidayHasher::new();
        "ot".hash(&mut hasher);
        assert_eq!(hasher.finish(), 3);
    }

    #[test]
    fn test_hasher_4() {
        let mut hasher = HolidayHasher::new();
        "ab".hash(&mut hasher);
        assert_eq!(hasher.finish(), 3);
    }

    #[test]
    fn test_hasher_5() {
        let mut hasher = HolidayHasher::new();
        "pc".hash(&mut hasher);
        assert_eq!(hasher.finish(), 3);
    }
}