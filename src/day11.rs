use utils;
use utils::Part;

pub fn solve(part:Part) -> u32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 11).unwrap();

    let mut distance = 0;

    let steps:Vec<String> = input.trim().split(',')
        .map(|s| s.to_string()).collect();

    for step in steps {

    }

    distance
}