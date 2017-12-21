use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Program {
    weight: u32,
    holding: Vec<String>,
}

pub fn solve(part: Part) -> String {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 7).unwrap();

    let mut out = String::new();

    let program_strings: Vec<String> = input.lines()
        .map(|s| s.replace(",", ""))
        .map(|s| s.replace("->", ""))
        .map(|s| s.replace("(", ""))
        .map(|s| s.replace(")", ""))
        .collect();

    let mut programs = HashMap::new();
    let mut holding_names = HashSet::new();

    for s in program_strings {
        let mut links = Vec::new();

        let mut tokens = s.split_whitespace();

        let prog = String::from(tokens.next().unwrap());
        let weight: u32 = tokens.next().unwrap().parse().unwrap();

        while let Some(name) = tokens.next() {
            holding_names.insert(String::from(name));
            links.push(String::from(name));
        }

        programs.insert(prog, Program { weight: weight, holding: links });
    }

    let root_set =
        &HashSet::from_iter(programs.keys().cloned()) - &holding_names;
    let root: String = root_set.iter().next().unwrap().clone();

    match part {
        Part::PartOne => { out = root; }
        Part::PartTwo => {}
    }

    out
}