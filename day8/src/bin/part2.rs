use std::collections::{BTreeMap, BTreeSet};
use std::time::Instant;
use regex::Regex;

struct Node {
    name: String,
    left: Option<usize>,
    right: Option<usize>,
}

#[derive(Copy, Clone)]
enum StepDirection {
    Left,
    Right
}

struct Graph {
    node_name_map: BTreeMap<String, usize>,
    nodes: Vec<Node>
}

impl Graph {
    fn new() -> Self {
        Self {
            node_name_map: BTreeMap::new(),
            nodes: Vec::new(),
        }
    }

    fn get_idx(&self, name: &str) -> Option<usize> {
        self.node_name_map.get(name).copied()
    }

    fn step(&self, from_idx: usize, step_direction: StepDirection) -> Option<usize> {
        match self.nodes.get(from_idx) {
            None => panic!("No node with idx: {}", from_idx),
            Some(node) => match step_direction {
                StepDirection::Left => node.left,
                StepDirection::Right => node.right
            }
        }
    }

    fn add(&mut self, name: &str, left: &str, right: &str) -> usize {
        let left_node_idx = *self.node_name_map.entry(left.to_string())
            .or_insert_with(|| {
                let n = Node {
                    name: left.to_string(),
                    left: None,
                    right: None
                };
                self.nodes.push(n);
                self.nodes.len() - 1
            });

        let right_node_idx = *self.node_name_map.entry(right.to_string())
            .or_insert_with(|| {
                let n = Node {
                    name: right.to_string(),
                    left: None,
                    right: None
                };
                self.nodes.push(n);
                self.nodes.len() - 1
            });

        let cur_node_idx = *self.node_name_map.entry(name.to_string())
            .or_insert_with(|| {
                let n = Node {
                    name: name.to_string(),
                    left: Some(left_node_idx),
                    right: Some(right_node_idx)
                };
                self.nodes.push(n);
                self.nodes.len() - 1
            });

        // Make sure the current node is linked to its descendants
        let cur_node = self.nodes.get_mut(cur_node_idx).unwrap();
        cur_node.left.replace(left_node_idx);
        cur_node.right.replace(right_node_idx);

        cur_node_idx
    }
}

fn parse(input: &str) -> (Vec<StepDirection>, Graph, BTreeSet<String>, BTreeSet<String>) {
    let mut lines = input.lines();

    let steps_line = lines.next().unwrap();
    let steps = steps_line.chars().map(|c| match c {
        'R' => StepDirection::Right,
        'L' => StepDirection::Left,
        x => panic!("Unknown char: {}", x)
    }).collect::<Vec<_>>();

    let _ = lines.next();

    let mut start_positions = BTreeSet::new();
    let mut end_positions = BTreeSet::new();

    let re = Regex::new(r"^(?<from>\w+) = \((?<left>\w+), (?<right>\w+)\)$").unwrap();
    let mut graph = Graph::new();
    for line in lines {
        let Some(caps) = re.captures(line) else {panic!("Could not parse line: {}", line)};
        let from = caps.name("from").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();

        for name in [from, left, right] {
            if name.ends_with("A") {
                start_positions.insert(name.to_string());
            }
            if name.ends_with("Z") {
                end_positions.insert(name.to_string());
            }
        }

        graph.add(from, left, right);
    }

    (steps, graph, start_positions, end_positions)
}

fn walk_graph(steps: &[StepDirection], graph: &Graph, start_position: &str, end_positions: &BTreeSet<String>) -> usize{
    let begin_idx = graph.get_idx(start_position).unwrap();
    let end_idxes = end_positions.iter().map(|s| graph.get_idx(s).unwrap()).collect::<Vec<_>>();

    let mut stepcount = 0;
    let mut cur_idx = begin_idx;

    while !end_idxes.contains(&cur_idx) {
        for step in steps.iter().copied() {
            stepcount += 1;
            cur_idx = graph.step(cur_idx, step).unwrap();
        }
    }

    stepcount
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        (a, b) = (b, a % b)
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn lcm_multiple(numbers: &[usize]) -> usize {
    if numbers.len() < 2 {
        panic!("Not enough numbers")
    }

    numbers.iter().skip(2)
        .fold(
            lcm(numbers[0], numbers[1]),
            |acc, el| lcm(acc, *el)
        )
}

fn main() {
    let input = include_str!("../input1.txt");

    let (steps, graph, start_positions, end_positions) = parse(input);

    let start = Instant::now();

    let stepcounts = start_positions.iter().map(|s| {
        walk_graph(&steps, &graph, s, &end_positions)
    }).collect::<Vec<_>>();

    let lcm_result = lcm_multiple(&stepcounts);

    let duration = start.elapsed();

    println!("Result: {}", lcm_result);
    println!("Time: {:?}", duration);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (steps, graph, start_positions, end_positions) = parse(input);

        let stepcounts = start_positions.iter().map(|s| {
            walk_graph(&steps, &graph, s, &end_positions)
        }).collect::<Vec<_>>();

        let lcm_result = lcm_multiple(&stepcounts);

        assert_eq!(lcm_result, 6);
    }

    #[test]
    fn test_lcm(){
        assert_eq!(lcm(2, 5), 10);
        assert_eq!(lcm(10, 5), 10);
        assert_eq!(lcm(8, 5), 40);
    }

    #[test]
    fn test_lcm_multiple(){
        assert_eq!(lcm_multiple(&[2, 5, 8]), 40);
        assert_eq!(lcm_multiple(&[2, 5, 8, 10]), 40);
        assert_eq!(lcm_multiple(&[2, 5, 8, 9]), 360);
    }
}