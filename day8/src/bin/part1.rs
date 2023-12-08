use std::collections::BTreeMap;
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
        let mut cur_node = self.nodes.get_mut(cur_node_idx).unwrap();
        cur_node.left.replace(left_node_idx);
        cur_node.right.replace(right_node_idx);

        cur_node_idx
    }
}

fn parse(input: &str) -> (Vec<StepDirection>, Graph) {
    let mut lines = input.lines();

    let steps_line = lines.next().unwrap();
    let steps = steps_line.chars().map(|c| match c {
        'R' => StepDirection::Right,
        'L' => StepDirection::Left,
        x => panic!("Unknown char: {}", x)
    }).collect::<Vec<_>>();

    let _ = lines.next();

    let re = Regex::new(r"^(?<from>\w+) = \((?<left>\w+), (?<right>\w+)\)$").unwrap();
    let mut graph = Graph::new();
    for line in lines {
        let Some(caps) = re.captures(line) else {panic!("Could not parse line: {}", line)};
        let from = caps.name("from").unwrap().as_str();
        let left = caps.name("left").unwrap().as_str();
        let right = caps.name("right").unwrap().as_str();

        graph.add(from, left, right);
    }

    (steps, graph)
}

fn walk_graph(steps: Vec<StepDirection>, graph: &Graph) -> usize{
    let begin_idx = graph.get_idx("AAA").unwrap();
    let end_idx = graph.get_idx("ZZZ").unwrap();

    let mut stepcount = 0;
    let mut cur_idx = begin_idx;

    while cur_idx != end_idx {
        for step in steps.iter().copied() {
            stepcount += 1;
            cur_idx = graph.step(cur_idx, step).unwrap();
        }
    }

    stepcount
}


fn main() {
    let input = include_str!("../input1.txt");

    let (steps, graph) = parse(input);

    let stepcount = walk_graph(steps, &graph);

    println!("Result: {}", stepcount)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (steps, graph) = parse(input);

        let stepcount = walk_graph(steps, &graph);

        assert_eq!(stepcount, 2);
    }

    #[test]
    fn test_input_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (steps, graph) = parse(input);

        let stepcount = walk_graph(steps, &graph);

        assert_eq!(stepcount, 6);
    }
}