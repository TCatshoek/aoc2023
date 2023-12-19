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

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
struct Rule<'a> {
    category: Option<&'a str>,
    operator: Operator,
    value: Option<i32>,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn opposite(&self) -> Rule<'a> {
        match self.operator {
            Operator::GT => Self {
                category: self.category,
                operator: Operator::LT,
                value: Some(self.value.unwrap() + 1),
                target: self.target
            },
            Operator::LT => Self {
                category: self.category,
                operator: Operator::GT,
                value: Some(self.value.unwrap() - 1),
                target: self.target
            },
            Immediate => panic!("Can't get opposite of immediate rule")
        }
    }

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
}

type WorkflowMap<'a> = HashMap<&'a str, Workflow<'a>, BuildHasherDefault<FxHasher>>;

fn parse(input: &str) -> WorkflowMap {
    let [input_workflows, _input_parts] = input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let workflow_map = FxHashMap::from_iter(input_workflows.lines()
        .map(Workflow::new)
        .map(|w| (w.name, w))
    );

    workflow_map
}

#[derive(Copy, Clone, Debug)]
struct Constraint<'a> {
    category: &'a str,
    operator: Operator,
    value: i32,
}

impl<'a> From<Rule<'a>> for Constraint<'a> {
    fn from(rule: Rule<'a>) -> Self {
        Constraint {
            category: rule.category.unwrap(),
            operator: rule.operator,
            value: rule.value.unwrap(),
        }
    }
}


fn get_constraints<'a>(workflows: WorkflowMap<'a>) -> Vec<Vec<Constraint<'a>>> {
    let constraints = Vec::new();
    let initial_workflow = "in";
    let initial_rule = 0;
    return _get_constraints(constraints, &workflows, initial_workflow, initial_rule)
        .iter()
        .filter_map(|x| x.clone())
        .collect()
}

fn _get_constraints<'a>(constraints: Vec::<Constraint<'a>>, workflows: &WorkflowMap<'a>, cur_workflow: &str, cur_rule: usize) -> Vec<Option<Vec<Constraint<'a>>>> {

    match cur_workflow {
        "A" => return vec![Some(constraints.clone())],
        "R" => return vec![None],
        _ => {}
    };

    let workflow = workflows.get(cur_workflow).unwrap();
    let rule = workflow.rules[cur_rule];

    return match (rule.operator, rule.category, rule.value, rule.target) {
        (Immediate, _, _, "A") => vec![Some(constraints.clone())],
        (Immediate, _, _, "R") => vec![None],
        (Immediate, _, _, target) => _get_constraints(constraints.clone(), workflows, target, 0),

        (Operator::GT | Operator::LT, _, _, target) => {

            let mut new_constraints_left = constraints.clone();
            new_constraints_left.push(rule.into());
            let mut new_constraints_right = constraints.clone();
            new_constraints_right.push(rule.opposite().into());

            let mut tmp = _get_constraints(new_constraints_left, workflows, target, 0);
            tmp.append(&mut _get_constraints(new_constraints_right, workflows, cur_workflow, cur_rule + 1));
            return tmp;

        }
    };
}

fn find_possibilities(constraint_sets: Vec<Vec<Constraint>>) -> u64 {
    let mut total: u64 = 0;

    for constraints in constraint_sets {

        let mut x_min = 0;
        let mut m_min = 0;
        let mut a_min = 0;
        let mut s_min = 0;
        let mut x_max = 4000;
        let mut m_max = 4000;
        let mut a_max = 4000;
        let mut s_max = 4000;

        for Constraint {category, operator, value} in constraints {
            match operator {
                Operator::GT => {
                    match category {
                        "x" => x_min = value as u64,
                        "m" => m_min = value as u64,
                        "a" => a_min = value as u64,
                        "s" => s_min = value as u64,
                        _ => panic!()
                    }
                }
                Operator::LT => {
                    match category {
                        "x" => x_max = value as u64 - 1,
                        "m" => m_max = value as u64 - 1,
                        "a" => a_max = value as u64 - 1,
                        "s" => s_max = value as u64 - 1,
                        _ => panic!()
                    }
                }
                Immediate => panic!()
            }
        }

        total += (x_max - x_min) * (m_max - m_min) * (a_max - a_min) * (s_max - s_min)

    }
    total
}

fn main() {
    let input = include_str!("../input.txt");
    let workflows = parse(input);
    let constraints = get_constraints(workflows);
    let result = find_possibilities(constraints);
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
        let workflows = parse(input);
        let constraints = get_constraints(workflows);
        let result = find_possibilities(constraints);
        println!("{:?}", result);
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn test_input_2() {
        let input = "in{x>3998:pb,R}
pb{m>3998:pc,R}
pc{a>3998:pd,R}
pd{s>3998:A,R}

";
        let workflows = parse(input);
        let constraints = get_constraints(workflows);
        let result = find_possibilities(constraints);
        assert_eq!(result, 16);
    }

    #[test]
    fn test_input_3() {
        let input = "in{x>3999:pb,x<2:pe,R}
pb{m>3999:pc,R}
pc{a>3999:pd,R}
pd{s>3999:A,R}
pe{m<2:pf,R}
pf{a<2:pg,R}
pg{s<2:A,R}

";
        let workflows = parse(input);
        let constraints = get_constraints(workflows);
        let result = find_possibilities(constraints);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_input_4() {
        let input = "in{x>3998:pb,x<3:pe,R}
pb{m>3998:pc,R}
pc{a>3998:pd,R}
pd{s>3998:A,R}
pe{m<3:pf,R}
pf{a<3:pg,R}
pg{s<3:A,R}

";
        let workflows = parse(input);
        let constraints = get_constraints(workflows);
        let result = find_possibilities(constraints);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_input_5() {
        let input = "in{x>3998:pb,x>3998:pe,R}
pb{m>3998:pc,R}
pc{a>3998:pd,R}
pd{s>3998:A,R}
pe{m>3998:pf,R}
pf{a>3998:pg,R}
pg{s>3998:A,R}

";
        let workflows = parse(input);
        let constraints = get_constraints(workflows);
        let result = find_possibilities(constraints);
        assert_eq!(result, 16);
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

}