/*
part 1:

part 2:

*/

use fxhash::FxHashMap;
use regex::Regex;
use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
enum Heading {
    Condition(bool),
    Next(String),
}

impl Heading {
    fn from_str(heading_str: &str) -> Self {
        match heading_str {
            "A" => Heading::Condition(true),
            "R" => Heading::Condition(false),
            heading_str => Heading::Next(heading_str.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
struct Comparison {
    current: char,
    expected: cmp::Ordering,
    num: u32,
}
#[derive(Debug)]
struct Part {
    ratings: FxHashMap<char, u32>,
    condition: Option<bool>,
}

impl Part {
    fn from_str(part_str: &str) -> Self {
        let mut ratings = FxHashMap::default();
        let qualities: Vec<&str> = part_str[1..part_str.len() - 1].split(",").collect();

        for q in qualities {
            let (quality, num) = q.split_once("=").unwrap();

            ratings.insert(quality.chars().next().unwrap(), num.parse().unwrap());
        }

        Part {
            ratings,
            condition: None,
        }
    }
}
#[derive(Debug, Clone)]
struct PartComp {
    comp: Option<Comparison>,
    heading: Heading,
}

impl PartComp {
    fn from_str(comp_str: &str) -> Self {
        if comp_str.contains(":") {
            let (comp_str, next) = comp_str.split_once(":").unwrap();
            if comp_str.contains("<") {
                let (curr_str, num_str) = comp_str.split_once("<").unwrap();

                PartComp {
                    comp: Some(Comparison {
                        current: curr_str.chars().next().unwrap(),
                        expected: cmp::Ordering::Less,
                        num: num_str.parse().unwrap(),
                    }),
                    heading: Heading::from_str(next),
                }
            } else {
                let (curr_str, num_str) = comp_str.split_once(">").unwrap();
                PartComp {
                    comp: Some(Comparison {
                        current: curr_str.chars().next().unwrap(),
                        expected: cmp::Ordering::Greater,
                        num: num_str.parse().unwrap(),
                    }),
                    heading: Heading::from_str(next),
                }
            }
        } else {
            PartComp {
                comp: None,
                heading: Heading::from_str(comp_str),
            }
        }
    }
}

#[derive(Debug)]
struct Workflows {
    workflows: FxHashMap<String, Vec<PartComp>>,
    parts: Vec<Part>,
}

impl Workflows {
    fn from_str(input: &[String]) -> Self {
        let mut workflows: FxHashMap<String, Vec<PartComp>> = FxHashMap::default();
        let mut parts = vec![];

        let mut is_workflow = true;

        let workflow_regex = Regex::new(r"([a-z]+)\{(.*)\}").unwrap();

        for line in input {
            if line.is_empty() {
                is_workflow = false;
                continue;
            }

            if is_workflow {
                let caps = workflow_regex.captures(line).unwrap();

                let mut part_comps = vec![];

                let name = &caps[1];
                let conds_str = &caps[2];
                let all_conds: Vec<&str> = conds_str.split(",").collect();
                for c in all_conds {
                    part_comps.push(PartComp::from_str(c));
                }

                workflows.insert(name.to_owned(), part_comps);
            } else {
                parts.push(Part::from_str(line));
            }
        }

        Workflows { workflows, parts }
    }
}

fn main() {
    let my_input = read_file("my_input.txt");

    part_1(&my_input);
    part_2(&my_input);
}

fn part_1(_my_input: &[String]) {
    let example_1 = read_file("example_1.txt");
    dbg!(&example_1);

    let example_sum = accepted_part_quality_sum_1(&example_1);
    dbg!(&example_sum);
    assert_eq!(example_sum, 19114);

    let my_sum = accepted_part_quality_sum_1(_my_input);
    dbg!(my_sum);
}

fn part_2(_my_input: &[String]) {
    let example_2 = read_file("example_2.txt");
    dbg!(&example_2);

    let example_sum = accepted_combinations_sum_2(&example_2);
    dbg!(&example_sum);
    assert_eq!(example_sum, 167409079868000);

    let my_sum = accepted_combinations_sum_2(_my_input);
    dbg!(my_sum);
}

fn accepted_part_quality_sum_1(input: &[String]) -> u64 {
    let mut all_workflows = Workflows::from_str(input);

    for part in all_workflows.parts.iter_mut() {
        let mut workflow_str = "in".to_owned();
        while part.condition.is_none() {
            for part_comp in all_workflows.workflows.get(&workflow_str).unwrap() {
                // compare the part quality with the workflow comparison step
                let comp_result = if let Some(comp) = &part_comp.comp {
                    part.ratings[&comp.current].cmp(&comp.num) == comp.expected
                } else {
                    true
                };

                if comp_result {
                    match &part_comp.heading {
                        Heading::Condition(cond) => part.condition = Some(*cond),
                        Heading::Next(heading) => workflow_str = heading.clone(),
                    }

                    break; // go to new workflow
                } else {
                    continue; // continue in the current workflow
                }
            }
        }
    }

    all_workflows
        .parts
        .iter()
        .filter(|part| matches!(part.condition, Some(true)))
        .map(|part| part.ratings.values().map(|&v| v as u64).sum::<u64>())
        .sum()
}

fn accepted_combinations_sum_2(input: &[String]) -> u64 {
    let all_workflows = Workflows::from_str(input);
    0
}

fn read_file(file_name: &str) -> Vec<String> {
    let file = File::open(file_name).expect("File not found");
    let reader = BufReader::new(file);

    reader.lines().map_while(Result::ok).collect()
}
