#![feature(let_chains)]

use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use itertools::Itertools;
use regex::Regex;
use rustc_hash::{FxHasher, FxHashMap};
use crate::Operator::Immediate;

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Operator {
    GT,
    LT,
    Immediate,
}

#[derive(Eq, PartialEq, Debug)]
struct Rule<'a> {
    category: Option<&'a str>,
    operator: Operator,
    value: Option<i32>,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn new(input: &'a str) -> Self {
        let rule_re = Regex::new(r"^(?<category>\w)(?<operator>[<|>])(?<value>\d+):(?<target>\w+)$").unwrap();
        if let Some(captures) = rule_re.captures(input) {
            let category = Some(captures.name("category").unwrap().as_str());
            let operator = match captures.name("operator").unwrap().as_str() {
                ">" => Operator::GT,
                "<" => Operator::LT,
                other => panic!("Unknown operator: {}", other)
            };
            let value = Some(captures.name("value").unwrap().as_str().parse().unwrap());
            let target = captures.name("target").unwrap().as_str();

            return Self {
                category,
                operator,
                value,
                target,
            };
        }

        Self {
            category: None,
            value: None,
            operator: Immediate,
            target: input,
        }
    }

    fn apply(&self, input: &Part) -> Option<&str> {
        match (self.operator, self.category, self.value) {
            (Immediate, _, _) => Some(self.target),
            (Operator::GT, Some(category), Some(value)) =>
                if input.get(category) > value {
                    Some(self.target)
                } else {
                    None
                },
            (Operator::LT, Some(category), Some(value)) =>
                if input.get(category) < value {
                    Some(self.target)
                } else {
                    None
                },
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn new(input: &'a str) -> Self {
        let re = Regex::new(r"^(?<name>\w+)\{(?<rules>\S+)}$").unwrap();
        let captures = re.captures(input).unwrap();

        let name = captures.name("name").unwrap().as_str();
        let rules = captures.name("rules").unwrap().as_str()
            .split(",")
            .map(Rule::new)
            .collect::<Vec<_>>();

        Self {
            name,
            rules,
        }
    }

    fn apply(&self, part: &Part) -> &str {
        for rule in &self.rules {
            match rule.apply(part) {
                None => continue,
                Some(dest) => return dest
            }
        }
        panic!("No destination found");
    }
}

#[derive(Debug)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"^\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)}$").unwrap();
        let captures = re.captures(input).unwrap();
        let x = captures.name("x").unwrap().as_str().parse().unwrap();
        let m = captures.name("m").unwrap().as_str().parse().unwrap();
        let a = captures.name("a").unwrap().as_str().parse().unwrap();
        let s = captures.name("s").unwrap().as_str().parse().unwrap();
        Self { x, m, a, s }
    }

    fn get(&self, category: &str) -> i32 {
        match category {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            x => panic!("Invalid category: {}", x)
        }
    }

    fn value(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn parse(input: &str) -> (HashMap<&str, Workflow, BuildHasherDefault<FxHasher>>, Vec<Part>) {
    let [input_workflows, input_parts] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let workflow_map = FxHashMap::from_iter(input_workflows.lines()
        .map(Workflow::new)
        .map(|w| (w.name, w))
    );

    let parts: Vec<Part> = input_parts.lines()
        .map(Part::new)
        .collect();

    (workflow_map, parts)
}

fn run_workflows<'a>(workflows: &HashMap<&str, Workflow, BuildHasherDefault<FxHasher>>, parts: &'a Vec<Part>) -> (Vec<&'a Part>, Vec<&'a Part>) {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for part in parts {

        let mut dest: &str = "in";

        while dest != "A" && dest != "R" {
            print!("{} -> ", dest);
            let workflow = workflows.get(dest).unwrap();
            dest = workflow.apply(part);
        }

        println!("{}", dest);

        if dest == "A" {
            accepted.push(part);
        }

        if dest == "R" {
            rejected.push(part);
        }
    }

    return (accepted, rejected);
}

fn main() {
    let input = include_str!("../input.txt");

    let (workflows, parts) = parse(input);
    let (accepted, rejected) = run_workflows(&workflows, &parts);

    let result = accepted.iter()
        .map(|part| part.value())
        .sum::<i32>();

    println!("Result: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    static TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    #[test]
    fn test_input_1() {
        let input = TEST_INPUT;
        let (workflows, parts) = parse(input);
        let (accepted, rejected) = run_workflows(&workflows, &parts);

        let result = accepted.iter()
            .map(|part| part.value())
            .sum::<i32>();

        assert_eq!(result, 19114);
    }

    #[test]
    fn test_parse() {
        let input = TEST_INPUT;
        parse(input);
    }

    #[test]
    fn test_parse_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";
        let workflow = Workflow::new(input);

        assert_eq!(workflow.name, "px");
        assert_eq!(workflow.rules[0], Rule {
            category: Some("a"),
            operator: Operator::LT,
            value: Some(2006),
            target: "qkq",
        });
        assert_eq!(workflow.rules[1], Rule {
            category: Some("m"),
            operator: Operator::GT,
            value: Some(2090),
            target: "A",
        });
        assert_eq!(workflow.rules[2], Rule {
            category: None,
            operator: Immediate,
            value: None,
            target: "rfg",
        });
    }

    #[test]
    fn test_apply_workflow_1() {
        let workflow = Workflow::new("px{a<2006:qkq,m>2090:A,rfg}");
        let part = Part::new("{x=787,m=2655,a=1222,s=2876}");
        let destination = workflow.apply(&part);
        assert_eq!(destination, "qkq");
    }

    #[test]
    fn test_apply_workflow_2() {
        let workflow = Workflow::new("px{a<2006:qkq,m>2090:A,rfg}");
        let part = Part::new("{x=1679,m=44,a=2067,s=496}");
        let destination = workflow.apply(&part);
        assert_eq!(destination, "rfg");
    }
}