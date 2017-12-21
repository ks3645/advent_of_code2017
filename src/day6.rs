use utils;
use utils::Part;
use std::collections::HashMap;

pub fn solve(part: Part) -> u32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 6).unwrap();

    let mut banks: Vec<u32> = input.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    let mut sum: u32 = 0;

    let mut states = HashMap::new();

    states.insert(banks.to_vec(), sum);

    loop {
        let mut max: (usize, u32) = (0, 0);
        for i in 0..banks.len() {
            if banks[i] > max.1 { max = (i, banks[i]); }
        }

        banks[max.0] = 0;

        for i in 0..max.1 {
            let pos = (max.0 + i as usize + 1) % banks.len();
            banks[pos] += 1;
        }

        sum += 1;
        if states.contains_key(&banks) { break; }

        states.insert(banks.to_vec(), sum);
    }

    match part {
        Part::PartOne => sum,
        Part::PartTwo => sum - states.get(&banks).unwrap(),
    }
}