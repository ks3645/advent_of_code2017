use utils;
use utils::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 8).unwrap();

    let mut out = 0;
    let mut running_max = 0;

    let mut registers = HashMap::new();

    for line in input.lines() {
        let tokens: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();

        let reg = tokens[0].clone();
        let op = tokens[1].clone();
        let adjust_value: i32 = tokens[2].clone().parse().unwrap();
        let check = tokens[4].clone();
        let condition = tokens[5].clone();
        let check_value: i32 = tokens[6].clone().parse().unwrap();

        if !registers.contains_key(&reg) { registers.insert(reg.clone(), 0); }

        if !registers.contains_key(&check) { registers.insert(check.clone(), 0); }

        let success = match condition.as_ref() {
            ">" => registers.get(&check).unwrap() > &check_value,
            "<" => registers.get(&check).unwrap() < &check_value,
            ">=" => registers.get(&check).unwrap() >= &check_value,
            "<=" => registers.get(&check).unwrap() <= &check_value,
            "==" => registers.get(&check).unwrap() == &check_value,
            "!=" => registers.get(&check).unwrap() != &check_value,
            _ => panic!("Day Eight: unexpected condition"),
        };

        if success {
            match op.as_ref() {
                "inc" => { *registers.get_mut(&reg).unwrap() += adjust_value; }
                "dec" => { *registers.get_mut(&reg).unwrap() -= adjust_value; }
                _ => { panic!("Day Eight: unexpected register operation"); }
            }
            let max = registers.values().max().unwrap();
            if *max > running_max {running_max = max.clone();}
        }
    }

    match part {
        Part::PartOne => { out = *registers.values().max().unwrap(); }
        Part::PartTwo => {
            out = running_max;
        }
    }


    out
}